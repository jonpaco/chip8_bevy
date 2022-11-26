use bevy::prelude::*;
use bevy::math::*;
use bevy_egui::{egui, EguiContext};

const COLS: u8 = 64;
const ROWS: u8 = 32;
const SCREEN_SIZE: usize = COLS as usize * ROWS as usize;

#[derive(Component)]
pub struct Display {
    scale: u8,
    screen: Vec<u8>,
    test: bool,
}

impl Display {
    pub fn new(scale: u8) -> Self {
        Self {
            scale: scale,
            screen: vec![0; SCREEN_SIZE],
            test: false,
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

pub fn render(mut egui_context: ResMut<EguiContext>, query: Query<&Display>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let display =  query.single();
            for (index, pixel) in display.screen.iter().enumerate() {
                if *pixel == 1 {
                    let x = (index % COLS as usize) * display.scale as usize;
                    let y = (index / COLS as usize) * display.scale as usize;
                    let min = egui::Pos2 {x: x as f32, y: y as f32};
                    let rect = ui.painter_at(egui::Rect::from_min_size(min, egui::Vec2::splat(display.scale as f32)));
                    rect.rect_filled(
                        egui::Rect::EVERYTHING,
                        1.0,
                        egui::Color32::WHITE,
                    );
                }
            }
        });
    });
}

pub fn test_render(mut query: Query<&mut Display>) {
    let mut display = query.single_mut();
    if display.test {
        display.set_pixel(0,0);
        display.set_pixel(5,2);
        display.set_pixel(0,2);
        display.test = false;
    }
}

