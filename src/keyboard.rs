use std::collections::HashMap;

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use crate::cpu;

static ONE: u8 = 1;
static TWO: u8 = 2;
static THREE: u8 = 3;
static FOUR: u8 = 0xC;
static Q: u8 = 0x4;
static W: u8 = 0x5;
static E: u8 = 0x6;
static R: u8 = 0xD;
static A: u8 = 0x7;
static S: u8 = 0x8;
static D: u8 = 0x9;
static F: u8 = 0xE;
static Z: u8 = 0xA;
static X: u8 = 0x0;
static C: u8 = 0xB;
static V: u8 = 0xF;

lazy_static! {
    pub static ref KEYMAP: HashMap<KeyCode, u8> = {
        HashMap::from([
            (KeyCode::Key1, ONE),
            (KeyCode::Key2, TWO),
            (KeyCode::Key3, THREE),
            (KeyCode::Key4, FOUR),
            (KeyCode::Q, Q),
            (KeyCode::W, W),
            (KeyCode::E, E),
            (KeyCode::R, R),
            (KeyCode::A, A),
            (KeyCode::S, S),
            (KeyCode::D, D),
            (KeyCode::F, F),
            (KeyCode::Z, Z),
            (KeyCode::X, X),
            (KeyCode::C, C),
            (KeyCode::V, V),
        ])
    };
}

#[derive(Component)]
pub struct Keyboard {
    key_list: HashMap<u8, bool>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key_list: HashMap::from([
                (ONE, false),
                (TWO, false),
                (THREE, false),
                (FOUR, false),
                (Q, false),
                (W, false),
                (E, false),
                (R, false),
                (A, false),
                (S, false),
                (D, false),
                (F, false),
                (Z, false),
                (X, false),
                (C, false),
                (V, false),
            ]),
        }
    }
}

pub fn is_key_pressed(input: &Res<Input<KeyCode>>, x: u8) -> bool {
    match KEYMAP.iter().find_map(|(&key, &value)| { if value == x && input.any_pressed([key]) {Some(value)} else {None}}) {
        Some(_) => {true},
        None => {false}
    }
}

pub fn keyboard_events(
    mut keyboard: Query<&mut Keyboard>,
    mut cpu: Query<&mut cpu::Cpu>,
    mut key_ver: EventReader<KeyboardInput>,
) {
    let mut keyboard_map = keyboard.single_mut();
    let mut cpu_comp = cpu.single_mut();
    for ev in key_ver.iter() {
        if let Some(keycode) = ev.key_code {
            if KEYMAP.contains_key(&keycode) {
                let code: u8 = KEYMAP[&keycode];
                *keyboard_map
                    .key_list
                    .entry(code)
                    .or_insert(ev.state == ButtonState::Pressed) = ev.state == ButtonState::Pressed;

                if cpu_comp.is_paused() && ev.state == ButtonState::Pressed {
                   cpu_comp.clear_pause(code); 
                }
            }
        }
    }
}

pub fn install_keyboard(mut commands: Commands) {
    let mut keyboard: Keyboard = Keyboard::new();
    commands.spawn(keyboard);
}
