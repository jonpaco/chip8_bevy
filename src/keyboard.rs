use std::collections::HashMap;

use bevy::prelude::*;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;

lazy_static! {
static ref KEYMAP: HashMap<KeyCode, u8> = {
        HashMap::from([
            (KeyCode::Key1, 0x1),
            (KeyCode::Key2, 0x2),
            (KeyCode::Key3, 0x3),
            (KeyCode::Key4, 0xc),
            (KeyCode::Q, 0x4),
            (KeyCode::W, 0x5),
            (KeyCode::E, 0x6),
            (KeyCode::R, 0xD),
            (KeyCode::A, 0x7),
            (KeyCode::S, 0x8),
            (KeyCode::D, 0x9),
            (KeyCode::F, 0xE),
            (KeyCode::Z, 0xA),
            (KeyCode::X, 0x0),
            (KeyCode::C, 0xB),
            (KeyCode::V, 0xF),
        ])
    };
}

#[derive(Component)]
pub struct Keyboard {
    keylist: Vec<ButtonState>,
}

impl Keyboard {
    fn new() -> Self {
        Self {
            keylist: vec![ButtonState::Released; KEYMAP.keys().len()],
        }
    }

    pub fn is_key_pressed(&self, btn: u8) -> bool {
        return self.keylist[btn as usize] == ButtonState::Pressed;
    }
}

pub fn install_keyboard(mut commands: Commands) {
    commands.spawn(Keyboard::new());
}

pub fn handle_keyboard(mut key_evr: EventReader<KeyboardInput>, mut query: Query<&mut Keyboard>) {
    for ev in key_evr.iter() {
        if let Some(keycode) = ev.key_code {
        let mut board = query.single_mut();
            if let Some(index) = KEYMAP.get(&keycode) {
                board.keylist[*index as usize] = ev.state;
            }
        }

    }
}
