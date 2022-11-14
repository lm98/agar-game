use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::{Player, Velocity, Movable};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_keyboard_event_system);
        
    }
}

fn player_spawn_system(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    })
    .insert(Player)
    .insert(Velocity {x: 0., y:0.})
    .insert(Movable { auto_despawn: false });
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };

        velocity.y = if kb.pressed(KeyCode::Up) {
            1.
        } else if kb.pressed(KeyCode::Down) {
            -1.
        } else {
            0.
        }
    }
}