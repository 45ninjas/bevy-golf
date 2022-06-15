use bevy::{
    math::const_vec3,
    prelude::*,
    render::camera::{Camera3d, WindowOrigin},
};
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
    camera.orthographic_projection.scale = 1.5;
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

/// Gets the cursor's position in normalised coordinates (-1, 1)
pub fn cursor_normalised(window: &Window, origin: &WindowOrigin) -> Option<Vec2> {
    match window.cursor_position() {
        Some(position) => Some(match origin {
            WindowOrigin::Center => {
                (position / Vec2::new(window.width(), window.height()) - Vec2::new(0.5, 0.5))
                // -1.0 to 1.0
                * Vec2::new(2.0, 2.0)
                // Aspect ratio correction.
                * Vec2::new(window.width() / window.height(), 1.0)
            }
            // TODO: Support 0,0 at the bottom left.
            WindowOrigin::BottomLeft => todo!(),
        }),
        None => None,
    }
}

/// Convert the cursor's screen position to a camera ray for orthographic projection.
pub fn cursor_to_world_orthographic(
    camera_world: &Transform,
    camera_projection: &OrthographicProjection,
    window: &Window,
) -> Option<Ray> {
    // Get our cursor's position or return None if no cursor exists.
    let cursor = cursor_normalised(window, &camera_projection.window_origin);
    if cursor.is_none() {
        return None;
    }
    let cursor_pos = cursor.unwrap();

    let mut position = camera_world.right() * cursor_pos.x * camera_projection.scale;
    position += camera_world.up() * cursor_pos.y * camera_projection.scale;

    Some(Ray {
        origin: camera_world.translation + position,
        direction: camera_world.forward(),
    })
}

/// The ray 
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

/// Find the intersection point of this ray and an infinite plane.
impl Ray {
    pub fn intersect_plane(&self, normal: Vec3, origin: Vec3) -> Vec3 {
        let origin = origin - self.origin;
        let distance = origin.dot(normal) / self.direction.dot(normal);
        self.origin + self.direction * distance
    }
}