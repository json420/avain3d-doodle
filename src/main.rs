use avian3d::prelude::*;
use bevy::prelude::*;

const LINEAR_ACCELERATION: f32 = 30.0; // m/s^2
const MAX_LINEAR_SPEED: f32 = 30.0; // m/s
const ANGULAR_ACCELERATION: f32 = 6.5; // radians/s^2
const MAX_ANGULAR_SPEED: f32 = 2.5; // radians/s
const JUMP_IMPULSE: f32 = 7.0; // m/s
const USER: f32 = 1.3; // m [Size of the player block]

#[derive(Component)]
struct Player;

#[derive(Resource, Deref, DerefMut, Debug)]
struct PlayerInput {
    #[deref]
    throttle: f32,
    steering: f32,
    jump: bool,
}

#[derive(Event)]
struct ResetWorld;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(PlayerInput {
            throttle: 0.0,
            steering: 0.0,
            jump: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input, update_player).chain())
        .add_observer(reset_world)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(200.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(200.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    let colors = [
        Color::srgb_u8(124, 144, 255),
        Color::srgb_u8(240, 255, 124),
        Color::srgb_u8(124, 255, 144),
        Color::srgb_u8(255, 144, 124),
        Color::srgb_u8(124, 144, 255),
        Color::srgb_u8(240, 255, 124),
        Color::srgb_u8(124, 255, 144),
        Color::srgb_u8(255, 144, 124),
    ];
    for i in -2..3_i32 {
        for j in -2..3_i32 {
            for k in 0..8 {
                commands.spawn((
                    RigidBody::Dynamic,
                    Collider::cuboid(1.0, 1.0, 1.0),
                    Mesh3d(meshes.add(Cuboid::from_length(1.0))),
                    MeshMaterial3d(materials.add(colors[k])),
                    Transform::from_xyz(i as f32 * 8.0, 1.0 + k as f32 * 1.05, j as f32 * 8.0),
                ));
            }
        }
    }

    for i in -1..2 {
        for j in -1..2 {
            commands.spawn((
                PointLight {
                    shadow_maps_enabled: true,
                    ..default()
                },
                Transform::from_xyz(i as f32 * 30.0, 10.0, j as f32 * 30.0),
            ));
        }
    }

    // The player
    commands
        .spawn((
            Player,
            RigidBody::Dynamic,
            Collider::cuboid(USER, USER, USER),
            ColliderDensity(4.0),
            Mesh3d(meshes.add(Cuboid::from_length(USER))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, 13.0, 0.0),
            LockedAxes::ROTATION_LOCKED,
            MaxLinearSpeed(MAX_LINEAR_SPEED),
            LinearDamping(0.5),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 3.0, 13.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });
}

fn keyboard_input(keyboard: Res<ButtonInput<KeyCode>>, mut input: ResMut<PlayerInput>) {
    let up = keyboard.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) as i8;
    let down = keyboard.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) as i8;
    let left = keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) as i8;
    let right = keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) as i8;
    input.throttle = (up - down).into();
    input.steering = (right - left).into();
    if keyboard.just_pressed(KeyCode::Space) {
        input.jump = true;
    }
}

fn update_player(
    time: Res<Time>,
    mut input: ResMut<PlayerInput>,
    mut query: Query<(
        &Player,
        &mut Transform,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    for (_player, mut transform, mut linear_velocity, mut angular_velocity) in &mut query {
        let dt = time.delta_secs();
        if input.throttle == 0.0 {
            linear_velocity.x *= 0.9;
            linear_velocity.z *= 0.9;
        } else {
            let delta_v = -transform.local_z() * input.throttle * LINEAR_ACCELERATION * dt;
            linear_velocity.x += delta_v.x;
            linear_velocity.z += delta_v.z;
        }

        if input.steering == 0.0 {
            angular_velocity.y *= 0.7;
        } else {
            angular_velocity.y += -input.steering * ANGULAR_ACCELERATION * dt;
            angular_velocity.y = angular_velocity
                .y
                .clamp(-MAX_ANGULAR_SPEED, MAX_ANGULAR_SPEED);
        }

        if input.jump {
            input.jump = false; // Clear the jump command
            linear_velocity.y = JUMP_IMPULSE;
        }

        if transform.translation.y < -10.0 {
            println!("fell off world");
            transform.translation.x = 0.0;
            transform.translation.y = 9.0;
            transform.translation.z = 0.0;
        }
    }
}

fn reset_world(on: On<ResetWorld>) {
    println!("here");
}
