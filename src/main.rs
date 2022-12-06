#[macro_use]
extern crate lazy_static;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_kira_audio::prelude::*;
use bevy_egui::EguiPlugin;
mod display;
mod keyboard;
mod speaker;
mod cpu;

const TIMESTEP_PER_SECOND: f64 = 1.0;

fn main() {
    App::new()
        .add_event::<display::InstallProgram>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(display::install_display)
        .add_startup_system(keyboard::install_keyboard)
        .add_startup_system(speaker::install_speaker)
        .add_startup_system(cpu::install_cpu)
        .add_system(display::render)
        .add_system(display::test_render)
        .add_system(keyboard::handle_keyboard)
        .add_system(cpu::cpu_event_handler)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::steps_per_second(TIMESTEP_PER_SECOND))
            .with_system(cpu::update_cpu)
        )
        .run();
}

