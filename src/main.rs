use bevy::prelude::*;
use components::{Velocity, Movable};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod player;
mod enemy;

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
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup_system)
        .add_system(movable_system)
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

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
            || translation.y < -win_size.h / 2. - MARGIN
            || translation.x > win_size.w / 2. + MARGIN
            || translation.x < -win_size.w / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
} 