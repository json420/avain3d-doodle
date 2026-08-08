use avian3d::prelude::*;
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.5),
        ColliderDensity(2.0),
    ));
}


fn main() {
    println!("Hello, world!");
}
