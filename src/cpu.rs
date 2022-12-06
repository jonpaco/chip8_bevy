use crate::display;
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

    fn update_timers(&self) {}

    fn execute_instruction(&self, _opcode: u16) {}
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
                }
                Err(e) => {
                    print!("{}", e);
                }
            }
        }
    }
}

pub fn update_cpu(query: Query<&Cpu>) {
    let cpu = query.single();
    for _ in 0..cpu.speed {
        if !cpu.paused {
            let opcode: u16 = ((cpu.memory[cpu.pc as usize] as u16) << 8) as u16
                | (cpu.memory[(cpu.pc as usize) + 1 as usize]) as u16;
            cpu.execute_instruction(opcode);
        }
    }

    if !cpu.paused {
        cpu.update_timers();
    }
}
