use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::components::{Velocity, Movable};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(20.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    })
    //.insert(Player)
    .insert(Velocity {x: 0., y:0.})
    .insert(Movable { auto_despawn: false });
}