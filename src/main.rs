#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_egui::EguiPlugin;
mod display;
mod keyboard;
mod speaker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(display::install_display)
        .add_startup_system(keyboard::install_keyboard)
        .add_startup_system(speaker::install_speaker)
        .add_system(display::render)
        .add_system(display::test_render)
        .add_system(keyboard::handle_keyboard)
        .run();
}

