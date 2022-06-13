use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::camera;

const BALL_RADIUS: f32 = 0.05;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_ball);
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
        .insert(Ccd::enabled())
        .insert(Collider::ball(BALL_RADIUS))
        .insert(Ball)
        .insert(camera::CameraTarget);
}
