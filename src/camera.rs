use bevy::{math::const_vec3, prelude::*, render::camera::Camera3d};
use bevy_rapier3d::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_camera);
        app.add_system(camera_follow);
    }
}

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct Smoother {
    smoothness: f32,
    enabled: bool,
}

const CAMERA_OFFSET: Vec3 = const_vec3!([-10.0, 10.0, -10.0]);

fn add_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 2.0;
    camera.transform = Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera);
}

fn camera_follow(
    mut transforms: ParamSet<(
        Query<(&mut Transform, Option<&mut Smoother>), With<Camera3d>>,
        Query<&Transform, With<CameraTarget>>,
    )>,
    time: Res<Time>,
) {
    // Get the middle of all the camera targets.
    let targets_query = transforms.p1();
    let mut middle_target = Vec3::ZERO;
    for target_transform in targets_query.iter() {
        middle_target += target_transform.translation;
    }
    middle_target /= targets_query.iter().count() as f32;

    // Update our camera's position.
    let mut camera_query = transforms.p0();
    let (mut camera_transform, mut smoother) = camera_query
        .get_single_mut()
        .expect("A singular Camera3d doesn't exist.");

    let target = middle_target + CAMERA_OFFSET;

    // TODO: Add smoothing.
    match smoother {
        Some(_) => todo!(),
        None => camera_transform.translation = target,
    }
}
