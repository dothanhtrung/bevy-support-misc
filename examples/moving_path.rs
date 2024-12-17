use bevy::prelude::*;
use bevy_support_misc::transform::{Movement, TransformSupportPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TransformSupportPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    let circle = meshes.add(Circle::new(10.0));
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Mesh2d(circle),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Movement {
            velocity: 0.5,
            des: vec![
                Vec3::new(200., 200., 0.),
                Vec3::new(-200., 200., 0.),
                Vec3::new(200., -200., 0.),
                Vec3::new(-200., -200., 0.),
            ],
        },
    ));
}
