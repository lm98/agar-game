use bevy::prelude::*;
use player::PlayerPlugin;

mod components;
mod player;

// region: --- Constants
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// region: --- Resources
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
// endregion: --- Resources

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
fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    let win_size = WinSize { w: win_w, h: win_h };
    commands.insert_resource(win_size);
}