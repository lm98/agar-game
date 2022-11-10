use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
        .add_startup_system(setup_system)
        // we want to execute this startup system after setup_system
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .run();
}

//bevy will inject arguments by type
fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn player_spawn_system(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}