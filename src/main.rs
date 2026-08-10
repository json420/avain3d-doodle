use avian3d::prelude::*;
use bevy::prelude::*;

const ACCELERATION: f32 = 30.0; // m/s^2
const IMPULSE: f32 = 7.0; // m/s

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

    // The player
    commands
        .spawn((
            Player,
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            Mesh3d(meshes.add(Cuboid::from_length(1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(2.0, 2.5, 0.75),
            LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
            GravityScale(2.0),
            Friction::new(0.7),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 2.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(5.5, 3.5, 1.5)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

#[derive(Component)]
struct Player;

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
    let dy = down as i8 - up as i8; // Forward is negative Z
    let direction = Vec2::new(dx.into(), dy.into());
    if direction != Vec2::ZERO {
        commands.trigger(Action::Move(direction));
    }

    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(Action::Jump);
    }
}

// If you flip the order of the fn args to (time, on), this doesn't work. Should it?
fn handle_action(
    on: On<Action>,
    time: Res<Time>,
    mut query: Query<(
        &Player,
        &Transform,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    for (_player, transform, mut linear_velocity, mut angular_velocity) in &mut query {
        match on.event() {
            Action::Move(direction) => {
                println!("Action::Move({direction})");
                let factor = ACCELERATION * time.delta_secs();

                let dv = transform.local_z() * (direction.y * factor);
                linear_velocity.x += dv.x;
                linear_velocity.z += dv.z;
                if direction.x != 0.0 {
                    angular_velocity.y = direction.x * -3.0;
                }
            }
            Action::Jump => {
                println!("Action::Jump");
                linear_velocity.y = IMPULSE;
            }
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
