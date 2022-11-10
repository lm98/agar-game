use bevy::prelude::*;
use player::PlayerPlugin;

mod components;
mod player;

// region: --- Constants
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;
// endregion: --- Constants

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4,0.4,0.4)))
        .insert_resource(WindowDescriptor {
            title: "Agar".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

//bevy will inject arguments by type
fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}