use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod display;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(display::install_display)
        .add_system(display::render)
        .add_system(display::test_render)
        .run();
}

