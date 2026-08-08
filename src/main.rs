use avian3d::prelude::*;
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::sphere(0.5),
        ColliderDensity(2.0),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));
    commands.spawn((RigidBody::Static, Collider::cuboid(5.0, 0.5, 5.0)));
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-1.0, -2.5, -1.5), Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(20.0, 10.0, 20.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    println!("done");
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .run();
}
