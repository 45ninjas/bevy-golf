use bevy::{audio::AudioSink, prelude::*, render::camera::Camera3d, transform};
use bevy_prototype_debug_lines::*;
use bevy_rapier3d::prelude::*;

use crate::camera::{self, cursor_ndc, Ray};

const BALL_RADIUS: f32 = 0.035;
const MAX_POWER: f32 = 0.25;
const MIN_VELOCITY: f32 = 0.01;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_ball);
        app.add_startup_system(ball_sounds);
        app.add_system(charge_ball);
        app.add_system(fire_ball);
    }
}

#[derive(Component)]
pub struct Ball(Vec3);

#[derive(Component)]
pub struct ChargeAudio {
    sound: Handle<AudioSource>,
    last_charge: f32,
}

fn add_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: BALL_RADIUS,
                subdivisions: 2,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::ANTIQUE_WHITE,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping: 0.03,
            angular_damping: 8.0,
        })
        .insert(ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        })
        .insert(Ccd::enabled())
        .insert(Collider::ball(BALL_RADIUS))
        .insert(Ball(Vec3::ZERO))
        .insert(camera::CameraTarget)
        .insert(ChargeAudio {
            sound: asset_server.load("sounds/pluck.ogg"),
            last_charge: 0.0,
        });
}

pub struct BallSounds {
    pub fire_sound: Handle<AudioSource>,
}

fn ball_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_sounds = BallSounds {
        fire_sound: asset_server.load("sounds/fire.ogg"),
    };
    commands.insert_resource(ball_sounds);
}

fn fire_ball(
    mut balls: Query<(
        &mut ExternalForce,
        &Velocity,
        &Ball,
        &mut ChargeAudio,
        &Transform,
    )>,
    buttons: Res<Input<MouseButton>>,
    mut sounds: ResMut<BallSounds>,
    audio_sinks: Res<Assets<AudioSink>>,
    audio: Res<Audio>,
    mut lines: ResMut<DebugLines>,
) {
    for (mut force, velocity, ball, mut charge_audio, transform) in balls.iter_mut() {
        if (velocity.linvel.length_squared() < MIN_VELOCITY) {
            if (buttons.just_released(MouseButton::Left)) {
                force.force = ball.0 * MAX_POWER;
                audio.play_with_settings(
                    sounds.fire_sound.clone(),
                    PlaybackSettings {
                        repeat: false,
                        volume: ball.0.length(),
                        speed: 2.0 - ball.0.length(),
                    },
                );
                charge_audio.last_charge = f32::NEG_INFINITY;
            } else {
                force.force = Vec3::ZERO;
            }

            if (buttons.pressed(MouseButton::Left)) {
                lines.line(transform.translation, transform.translation + ball.0, 0.0);

                let charge = (ball.0.length() * 4.1).floor();

                if (charge_audio.last_charge != charge) {
                    charge_audio.last_charge = charge;
                    audio.play_with_settings(
                        charge_audio.sound.clone(),
                        PlaybackSettings {
                            repeat: false,
                            volume: 0.5,
                            speed: 1.0 + (charge * 0.5),
                        },
                    );
                }
            }
        } else {
            // Reset the force of this ball.
            force.force = Vec3::ZERO;
        }
    }
}

fn charge_ball(
    mut balls_query: Query<(&mut Ball, &Velocity, &Transform)>,
    camera_query: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    windows: Res<Windows>,
) {
    // Send a ray from screen into the world.
    // Get our ray.
    let window = windows
        .get_primary()
        .expect("No window to get cursor position from.");

    let (cam_transform, cam) = camera_query.single();
    let ray = Ray::from_screenspace(camera::cursor_ndc(window), cam, cam_transform);
    if ray.is_none() {
        return;
    }
    let ray = ray.unwrap();

    // Apply the appropriate forces to our balls based on our raycast.
    for (mut ball, velocity, trans) in balls_query.iter_mut() {
        // If our rigidbody isn't moving. Set it's force to zero.
        if velocity.linvel.length_squared() > MIN_VELOCITY {
            continue;
        }

        let dir = (ray.intersect_plane(Vec3::Y, trans.translation) - trans.translation);

        let power = dir.length().powi(2).clamp(0.0, MAX_POWER) / MAX_POWER;
        ball.0 = -dir.normalize() * power;
    }
}
