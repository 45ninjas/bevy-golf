use bevy::{prelude::*, render::camera::Camera3d};
use bevy_prototype_debug_lines::*;
use bevy_rapier3d::prelude::*;

use crate::camera;

const BALL_RADIUS: f32 = 0.035;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_ball);
        app.add_system(move_ball);
    }
}

#[derive(Component)]
pub struct Ball;

fn add_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
        .insert(Ball)
        .insert(camera::CameraTarget);
}

fn move_ball(
    mut balls_query: Query<(&mut ExternalForce, &Velocity, &Transform), With<Ball>>,
    mut camera_query: Query<(&Transform, &OrthographicProjection), With<Camera3d>>,
    mut lines: ResMut<DebugLines>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
) {
    // Get our ray.
    let window = windows
        .get_primary()
        .expect("No window to get cursor position from.");
    let (cam_transform, cam_projection) = camera_query.single();
    let ray = camera::cursor_to_world_orthographic(cam_transform, cam_projection, window);
    if ray.is_none() {
        return;
    }
    let ray = ray.unwrap();

    for (mut force, velocity, transform) in balls_query.iter_mut() {
        // If our rigidbody isn't moving. Set it's force to zero.
        if velocity.linvel.distance_squared(Vec3::ZERO) > 0.025 {
            force.force = Vec3::ZERO;
            continue;
        }

        let mut dir = ray.intersect_plane(Vec3::Y, transform.translation) - transform.translation;
        dir /= cam_projection.scale;
        dir = dir.clamp_length(0.01, 0.25);

        lines.line_colored(transform.translation, ray.intersect_plane(Vec3::Y, transform.translation), 0.0, Color::ORANGE_RED);

        if buttons.pressed(MouseButton::Left) {
            lines.line_colored(
                transform.translation,
                transform.translation - dir,
                0.0,
                Color::WHITE,
            );
        }

        if buttons.just_released(MouseButton::Left) {
            force.force = -dir;
            println!("Bam!");
        }
    }
}
