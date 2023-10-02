use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct IsJumping(bool);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Jump(f32);

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                color: Color::rgba(1.0, 1.0, 0.47, 1.0),
                range: 10.0,
                intensity: 4000.0,
                outer_angle: 0.5,
                inner_angle: 0.4,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.25, -0.3),
            ..default()
        },
        Name::new("Flashlight"),
    );

    let player = (
        SceneBundle {
            scene: assets.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ActiveEvents::COLLISION_EVENTS,
        Player,
        Name::new("Player"),
        ThirdPersonCameraTarget,
        Speed(2.5),
        Jump(5.0),
        IsJumping(false),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Collider::cylinder(0.5, 0.25),
        RigidBody::Dynamic,
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}

fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Speed, &Jump, &mut IsJumping), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut collision_evr: EventReader<CollisionEvent>,
) {
    for (mut player_transform, player_speed, jump, mut is_jumping) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        // stop jumping when collision is detected
        for ev in collision_evr.iter() {
            if let CollisionEvent::Started(handle1, handle2, handle3) = ev {
                println!("{:?}", handle1);
                println!("{:?}", handle2);
                println!("{:?}", handle3);
                is_jumping.0 = false;
            }
        }

        // jump
        if keys.just_pressed(KeyCode::F) || is_jumping.0 {
            player_transform.translation.y += jump.0 * time.delta_seconds();
            is_jumping.0 = true;
        }

        // rotate player to face direction he is currently moving
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}
