use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

const COLS: u8 = 64;
const ROWS: u8 = 64;
const SCREEN_SIZE: usize = COLS as usize * ROWS as usize;

#[derive(Component)]
pub struct Display {
    scale: u8,
    screen: Vec<u8>,
}

impl Display {
    pub fn new(scale: u8) -> Self {
        Self {
            scale: scale,
            screen: vec![0; SCREEN_SIZE],
        }
    }

    fn set_pixel(&mut self, x: u8, y: u8) -> bool {
        let col = if x > COLS { x % COLS } else { x };
        let row = if y > ROWS { y % ROWS } else { y };
        let pixel_location = col + (row * COLS);
        self.screen[pixel_location as usize] ^= 1;
        !(self.screen[pixel_location as usize] as usize == 0)
    }

    fn clear(&mut self) {
        self.screen = vec![0; SCREEN_SIZE];
    }
}

pub fn install_display(mut commands: Commands) {
    commands.spawn(Display::new(10));
}

pub fn render(mut egui_context: ResMut<EguiContext>, _query: Query<&Display>) {
//    egui::Window::new("Chip8")
//        .resizable(true)
//        .show(egui_context.ctx_mut(), |ui| {
//            ui.horizontal(|ui| ui.label("Emulator"));
//        });

    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.label("Emulator");
    });
}
