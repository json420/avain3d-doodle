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
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .run();
}
