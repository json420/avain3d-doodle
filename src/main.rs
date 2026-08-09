use avian3d::prelude::*;
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(400.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(400.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        Mesh3d(meshes.add(Capsule3d::new(0.4, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(2.0, 7.5, 0.75),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(5.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, -11.5, 17.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(240, 255, 124))),
        Transform::from_xyz(0.0, 8.0, 0.0),
    ));
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(0.0, 42.0, 0.0)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 255, 144))),
        Transform::from_xyz(0.0, 12.0, 0.0),
    ));
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(0.0, -69.0, 10.0)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 144, 124))),
        Transform::from_xyz(0.0, 16.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 1.9, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    println!("done");
}

#[derive(Event, Debug)]
enum Action {
    Move(Vec2),
    Jump,
}

fn keyboard_input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    let up = keyboard.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let dx = right as i8 - left as i8;
    let dy = up as i8 - down as i8;
    let direction = Vec2::new(dx.into(), dy.into()).clamp_length_max(1.0);
    if direction != Vec2::ZERO {
        commands.trigger(Action::Move(direction));
    }

    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(Action::Jump);
    }
}

fn handle_action(on: On<Action>) {
    match on.event() {
        Action::Move(direction) => {
            println!("Action::Move({direction})");
        }
        Action::Jump => {
            println!("Action::Jump");
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .add_observer(handle_action)
        .run();
}
