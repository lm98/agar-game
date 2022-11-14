use rand::prelude::*;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{components::{Velocity, Movable}, WinSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    win_size: Res<WinSize>,
){
    for _ in 1..5 {
        let mut rng = rand::thread_rng();
        let w_span = win_size.w / 2. -100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);

        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(rng.gen_range(20.0..70.0)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform {
                translation: Vec3::new(x,y,10.),
                ..Default::default()
            },
            ..Default::default()
        })
        //.insert(Player)
        .insert(Velocity {x: 0., y:0.})
        .insert(Movable { auto_despawn: false });
    }
}