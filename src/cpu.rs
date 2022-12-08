use crate::display;
use crate::keyboard;
use bevy::prelude::*;
const REGISTER_SIZE: usize = 16;

#[derive(Component)]
pub struct Cpu {
    memory: Vec<u8>,
    registers: [u8; REGISTER_SIZE as usize],
    address: u8,
    delay_timer: u32,
    sound_timer: u32,
    pc: u32,
    stack: Vec<u32>,
    paused: bool,
    speed: u32,
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
            paused: false,
            speed: 10,
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
            self.delay_timer -= 0;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 0;
        }
    }

    fn execute_instruction(
        &mut self,
        opcode: u16,
        display: &mut display::Display,
        keyboard: &mut keyboard::Keyboard,
    ) {
        self.pc += 2;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => display.clear(),
                0x00EE => match self.stack.pop() {
                    Some(instr) => self.pc = instr,
                    None => {}
                },
                _ => {}
            },
            0x1000 => self.pc = (opcode as u16 & 0xFFF as u16) as u32,
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = (opcode as u16 & 0xFFF as u16) as u32;
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
                if self.registers[y as usize] == self.registers[y as usize] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                self.registers[x as usize] = (opcode & 0xFF) as u8;
            }
            0x7000 => {
                self.registers[x as usize] += (opcode & 0xFF) as u8;
            }
            0x8000 => match opcode & 0xF {
                0x0 => self.registers[x as usize] = self.registers[y as usize],
                0x1 => self.registers[x as usize] |= self.registers[y as usize],
                0x2 => self.registers[x as usize] &= self.registers[y as usize],
                0x3 => self.registers[x as usize] ^= self.registers[y as usize],
                0x4 => {
                    self.registers[x as usize] += self.registers[y as usize];
                    let sum = self.registers[x as usize];
                    self.registers[0xF] = 0;
                    if sum as u8 > 0xFF as u8 {
                        self.registers[0xF] = 1;
                    }
                }
                0x5 => {
                    self.registers[0xF] = 0;
                    if self.registers[x as usize] > self.registers[y as usize] {
                        self.registers[0xF] = 1;
                    }
                    self.registers[x as usize] -= self.registers[y as usize];
                }
                0x6 => {
                    self.registers[0xF] = self.registers[x as usize] & 0x1;
                    self.registers[x as usize] >>= 1;
                }
                0x7 => {
                    self.registers[0xF] = 0;

                    if self.registers[y as usize] > self.registers[x as usize] {
                        self.registers[0xF] = 1;
                    }

                    self.registers[x as usize] =
                        self.registers[y as usize] - self.registers[x as usize];
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
                self.address = (opcode & 0xFFF) as u8;
            }
            0xB000 => {
                self.address = (opcode & 0xFFF) as u8 + self.registers[0];
            }
            0xC000 => {
                let rand: u8 = rand::random();
                self.registers[x as usize] = rand & (opcode & 0xFF) as u8;
            }
            0xD000 => {
                let width = 8;
                let height = opcode & 0xF;
                self.registers[0xF] = 0;
                for row in 1..height {
                    let mut sprite = self.memory[(self.address + row as u8) as usize];
                    for col in 1..width {
                        if sprite & 0x80 > 0 {
                            if display.set_pixel(
                                self.registers[x as usize] + col as u8,
                                self.registers[y as usize] + row as u8,
                            ) {
                                self.registers[0xF] = 1;
                            }
                        }

                        sprite <<= 1;
                    }
                }
            }
            0xE000 => match opcode & 0xFF {
                0x9E => {
                    if keyboard.is_key_pressed(self.registers[x as usize]) {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    if !keyboard.is_key_pressed(self.registers[x as usize]) {
                        self.pc += 2;
                    }
                }
                _ => {}
            },
            0xF000 => match opcode & 0xFF {
                0x07 => {}
                0x0A => {}
                0x15 => {}
                0x18 => {}
                0x1E => {}
                0x29 => {}
                0x33 => {}
                0x55 => {}
                0x65 => {}
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
            match std::fs::read(path) {
                Ok(bytes) => {
                    let mut cpu = query.single_mut();
                    const OFFSET: usize = 0x200;
                    let memory = cpu.memory.as_mut_ptr() as usize + OFFSET;
                    let len = std::cmp::min(cpu.memory.len() - OFFSET, bytes.len());
                    unsafe {
                        std::ptr::copy_nonoverlapping(bytes.as_ptr(), memory as *mut u8, len);
                    }
                    cpu.pc = 0;
                }
                Err(e) => {
                    print!("{}", e);
                }
            }
        }
    }
}

pub fn update_cpu(
    mut cpu_query: Query<&mut Cpu>,
    mut display_query: Query<&mut display::Display>,
    mut keyboard_query: Query<&mut keyboard::Keyboard>,
) {
    //let (mut cpu, mut display, mut keyboard) = query.single_mut();
    let mut cpu = cpu_query.single_mut();
    let mut display = display_query.single_mut();
    let mut keyboard = keyboard_query.single_mut();
    for _ in 0..cpu.speed {
        if !cpu.paused {
            let opcode: u16 = ((cpu.memory[cpu.pc as usize] as u16) << 8) as u16
                | (cpu.memory[(cpu.pc as usize) + 1 as usize]) as u16;
            cpu.execute_instruction(opcode, &mut display, &mut keyboard);
        }
    }

    if !cpu.paused {
        cpu.update_timers();
    }
}
