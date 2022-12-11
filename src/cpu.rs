use crate::display;
use crate::keyboard;
use bevy::prelude::*;
use std::fs::File;
use std::io::Read;
use bevy::input::keyboard::KeyboardInput;
const REGISTER_SIZE: usize = 16;

#[derive(Component)]
pub struct Cpu {
    memory: Vec<u8>,
    registers: [u8; REGISTER_SIZE as usize],
    address: u16,
    delay_timer: u32,
    sound_timer: u32,
    pc: u16,
    stack: Vec<u16>,
    paused: bool,
    speed: u32,
    started: bool,
}

const SPRITE_LENGTH: usize = 80;
const SPRITES: [u8; SPRITE_LENGTH] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 4096],
            registers: [0; 16],
            address: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack: Vec::new(),
            paused: true,
            speed: 10,
            started: false,
        }
    }
}

impl Cpu {
    fn load_sprites_into_memory(&mut self) {
        for (index, &sprite) in SPRITES.iter().enumerate() {
            self.memory[index] = sprite;
        }
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn clear_pause(&mut self, key: u8) {
        let opcode: u16 = ((self.memory[self.pc as usize] as u16) << 8) as u16
            | (self.memory[(self.pc as usize) + 1 as usize]) as u16;
        let x = (opcode & 0x0F00) >> 8;
        self.registers[x as usize] = key;
        self.paused = false;
    }

    fn execute_instruction(
        &mut self,
        opcode: u16,
        display: &mut display::Display,
        keyboard: &keyboard::Keyboard,
    ) {
        self.pc += 2;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => {
                    display.clear();
                }
                0x00EE => match self.stack.pop() {
                    Some(instr) => self.pc = instr,
                    None => {}
                },
                _ => {}
            },
            0x1000 => self.pc = (opcode as u16 & 0xFFF as u16) as u16,
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = (opcode as u16 & 0xFFF as u16) as u16;
            }
            0x3000 => {
                if self.registers[x as usize] == (opcode & 0xFF as u16) as u8 {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.registers[x as usize] != (opcode & 0xFF as u16) as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                self.registers[x as usize] = (opcode & 0xFF) as u8;
            }
            0x7000 => {
                self.registers[x as usize] =
                    self.registers[x as usize].wrapping_add((opcode & 0xFF) as u8);
            }
            0x8000 => match opcode & 0xF {
                0x0 => self.registers[x as usize] = self.registers[y as usize],
                0x1 => self.registers[x as usize] |= self.registers[y as usize],
                0x2 => self.registers[x as usize] &= self.registers[y as usize],
                0x3 => self.registers[x as usize] ^= self.registers[y as usize],
                0x4 => match self.registers[x as usize].checked_add(self.registers[y as usize]) {
                    Some(value) => {
                        self.registers[0xF] = 0;
                        self.registers[x as usize] = value;
                    }
                    None => {
                        self.registers[x as usize] =
                            self.registers[x as usize].wrapping_add(self.registers[y as usize]);
                        self.registers[0xF] = 1;
                    }
                },
                0x5 => {
                    self.registers[0xF] = 0;
                    if self.registers[x as usize] < self.registers[y as usize] {
                        self.registers[0xF] = 1;
                    }
                    self.registers[x as usize] =
                        self.registers[x as usize].wrapping_sub(self.registers[y as usize]);
                }
                0x6 => {
                    self.registers[0xF] = self.registers[x as usize] & 0x1;
                    self.registers[x as usize] >>= 1;
                }
                0x7 => {
                    self.registers[0xF] = 0;

                    if self.registers[y as usize] < self.registers[x as usize] {
                        self.registers[0xF] = 1;
                    }

                    self.registers[x as usize] =
                        self.registers[y as usize].wrapping_sub(self.registers[x as usize]);
                }
                0xE => {
                    self.registers[0xF] = self.registers[x as usize] & 0x80;
                    self.registers[x as usize] <<= 1;
                }
                _ => {}
            },
            0x9000 => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.address = opcode & 0xFFF;
            }
            0xB000 => {
                self.address = (opcode & 0xFFF) + self.registers[0] as u16;
            }
            0xC000 => {
                let rand: u8 = rand::random();
                self.registers[x as usize] = rand & (opcode & 0xFF) as u8;
            }
            0xD000 => {
                let x_reg: u8 = self.registers[x as usize] & display::COLS - 1;
                let y_reg: u8 = self.registers[y as usize] & display::ROWS - 1;
                let width: u8 = 8;
                let height: u8 = (opcode & 0xF) as u8;
                self.registers[0xF] = 0;
                for row in 0..height {
                    let mut sprite: u8 = self.memory[(self.address + row as u16) as usize];
                    for col in 0..width {
                        if sprite & 0x80 > 0 {
                            if !display
                                .is_offscreen(x_reg.wrapping_add(col), y_reg.wrapping_add(row))
                            {
                                if display
                                    .set_pixel(x_reg.wrapping_add(col), y_reg.wrapping_add(row))
                                {
                                    self.registers[0xF] = 1;
                                }
                            } else {
                                break;
                            }
                        }
                        sprite <<= 1;
                    }
                }
            }
            0xE000 => match opcode & 0xFF {
                0x9E => {
                    if keyboard.is_key_pressed(self.registers[x as usize] as u8) {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    if !keyboard.is_key_pressed(self.registers[x as usize] as u8) {
                        self.pc += 2;
                    }
                }
                _ => {}
            },
            0xF000 => match opcode & 0xFF {
                0x07 => {
                    self.registers[x as usize] = self.delay_timer as u8;
                }
                0x0A => {
                    self.paused = true;
                }
                0x15 => {
                    self.delay_timer = self.registers[x as usize] as u32;
                }
                0x18 => {
                    self.sound_timer = self.registers[x as usize] as u32;
                }
                0x1E => {
                    self.address += self.registers[x as usize] as u16;
                }
                0x29 => {
                    self.address = self.registers[x as usize].wrapping_mul(5) as u16;
                }
                0x33 => {
                    self.memory[self.address as usize] = (self.registers[x as usize] / 100) as u8;
                    self.memory[self.address as usize + 1 as usize] =
                        ((self.registers[x as usize] % 100) / 10) as u8;
                    self.memory[self.address as usize + 2 as usize] =
                        (self.registers[x as usize] % 10) as u8;
                }
                0x55 => {
                    for reg in 0..(x+1){
                        self.memory[self.address as usize + reg as usize] = self.registers[reg as usize] as u8;
                    }
                }
                0x65 => {
                    for reg in 0..(x+1) {
                        self.registers[reg as usize] = self.memory[reg as usize + self.address as usize];
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn install_cpu(mut commands: Commands) {
    let mut cpu: Cpu = Cpu::new();
    cpu.load_sprites_into_memory();
    commands.spawn(cpu);
}

pub fn cpu_event_handler(
    mut ev_cpu: EventReader<display::InstallProgram>,
    mut query: Query<&mut Cpu>,
) {
    for program in ev_cpu.iter() {
        let path_wrapped = program.path.as_path();
        if let Some(path) = path_wrapped.to_str() {
            let mut cpu: &mut Cpu = &mut query.single_mut();
            cpu.paused = true;
            cpu.started = false;
            const OFFSET: usize = 0x200;
            if let Ok(mut f) = File::open(path) {
                match f.read(&mut cpu.memory[OFFSET..]) {
                    Ok(_) => {
                        cpu.pc = OFFSET as u16;
                        cpu.started = true;
                        cpu.paused = false;
                    }
                    Err(err) => println!("Error Reading File {err}"),
                }
            }
        }
    }
}

pub fn update_cpu(
    mut cpu_query: Query<&mut Cpu>,
    mut display_query: Query<&mut display::Display>,
    keyboard_query: Query<&keyboard::Keyboard>,
) {
    let cpu: &mut Cpu = &mut cpu_query.single_mut();
    if cpu.started && !cpu.paused {
        let mut display: &mut display::Display = &mut display_query.single_mut();
        let keyboard: &keyboard::Keyboard = &keyboard_query.single();
        let opcode: u16 = ((cpu.memory[cpu.pc as usize] as u16) << 8) as u16
            | (cpu.memory[(cpu.pc as usize) + 1 as usize]) as u16;
        cpu.execute_instruction(opcode, &mut display, &keyboard);
    }

    if cpu.started && !cpu.paused {
        cpu.update_timers();
    }
}
