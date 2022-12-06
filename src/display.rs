use bevy::prelude::*;
use bevy::math::*;
use bevy_egui::{egui, EguiContext};
use std::path::{Path, PathBuf};
use rfd;

use crate::cpu;

const COLS: u8 = 64;
const ROWS: u8 = 32;
const SCREEN_SIZE: usize = COLS as usize * ROWS as usize;

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

pub fn render(mut egui_context: ResMut<EguiContext>, mut ev_program: EventWriter<InstallProgram>, query: Query<&Display>) {
   egui::TopBottomPanel::top("menu").show(egui_context.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Load ROM").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        ev_program.send(InstallProgram{path});
                    }
                }
            })
        })
   });

    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let display =  query.single();
            let rows = (ROWS as f32 * display.scale as f32) as f32;
            let cols = (COLS as f32 * display.scale as f32) as f32;
            let (response, painter) = ui.allocate_painter(egui::Vec2::new(cols, rows), egui::Sense::hover());
            for (index, pixel) in display.screen.iter().enumerate() {
                if *pixel == 1 {
                    let x = (index % COLS as usize) * display.scale as usize + response.rect.min.x as usize;
                    let y = (index / COLS as usize) * display.scale as usize + response.rect.min.y as usize;
                    let min = egui::Pos2 {x: x as f32, y: y as f32};
                    painter.rect_filled(
                        egui::Rect::from_min_size(min, egui::Vec2::splat(display.scale as f32)),
                        1.0,
                        egui::Color32::WHITE
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

