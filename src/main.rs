#[macro_use]
extern crate lazy_static;
extern crate rand;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_kira_audio::prelude::*;
use bevy::time::FixedTimestep;
use bevy_egui::EguiPlugin;
mod display;
mod keyboard;
mod speaker;
mod cpu;

fn main() {
    App::new()
        .add_event::<display::InstallProgram>()
        .add_plugins(DefaultPlugins.set( WindowPlugin {
                window: WindowDescriptor {
                title: "Chip8".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
                },
                ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(display::install_display)
        .add_startup_system(speaker::install_speaker)
        .add_startup_system(cpu::install_cpu)
        .add_startup_system(keyboard::install_keyboard)
        .add_system(cpu::cpu_event_handler)
        .add_system(display::render)
        .add_system(cpu::update_cpu)
        .run();
}

