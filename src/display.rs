use bevy::math::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use rfd;
use std::path::PathBuf;

pub const COLS: u8 = 64;
pub const ROWS: u8 = 32;
pub const SCREEN_SIZE: usize = COLS as usize * ROWS as usize;

#[derive(Component)]
pub struct Display {
    scale: u8,
    screen: Vec<u8>,
    test: bool,
}

pub struct InstallProgram {
    pub path: PathBuf,
}

impl Display {
    pub fn new(scale: u8) -> Self {
        Self {
            scale: scale,
            screen: vec![0; SCREEN_SIZE],
            test: true,
        }
    }

    pub fn set_pixel(&mut self, x: u8, y: u8) -> bool {
        let pixel_location = x as u16 + (y as u16 * COLS as u16) & 0x7FF;
        self.screen[pixel_location as usize] ^= 1;
        self.screen[pixel_location as usize] as usize == 0
    }

    pub fn is_offscreen(&self, x: u8, y: u8) -> bool {
        if x >= COLS || y >= ROWS {
            return true;
        }
        return false;
    }

    pub fn clear(&mut self) {
        self.screen = vec![0; SCREEN_SIZE];
    }
}

pub fn install_display(mut commands: Commands) {
    commands.spawn(Display::new(10));
}

pub fn render(
    mut egui_context: ResMut<EguiContext>,
    mut ev_program: EventWriter<InstallProgram>,
    mut query: Query<&mut Display>,
) {
    egui::Window::new("Emulator").show(egui_context.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Load ROM").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let mut display = query.single_mut();
                        display.clear();
                        ev_program.send(InstallProgram { path });
                    }
                }
            })
        });

        let display = query.single();
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::hover());
        for (index, pixel) in display.screen.iter().enumerate() {
            let x = (index % COLS as usize) * display.scale as usize + response.rect.min.x as usize;
            let y = (index / COLS as usize) * display.scale as usize + response.rect.min.y as usize;
            let min = egui::Pos2 {
                x: x as f32,
                y: y as f32,
            };
            if *pixel == 1 {
                painter.rect_filled(
                    egui::Rect::from_min_size(min, egui::Vec2::splat(display.scale as f32)),
                    1.0,
                    egui::Color32::WHITE,
                );
            } else {
                painter.rect_filled(
                    egui::Rect::from_min_size(min, egui::Vec2::splat(display.scale as f32)),
                    1.0,
                    egui::Color32::BLACK,
                );
            }
        }
    });
}

pub fn test_render(mut query: Query<&mut Display>) {
    let mut display = query.single_mut();
    if display.test {
        for pixel in display.screen.iter_mut() {
            *pixel = 1;
        }
        display.test = false;
    }
}
