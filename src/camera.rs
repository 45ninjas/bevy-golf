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

#[derive(Component, Debug)]
pub struct Smoother<T> {
    pub smoothness: f32,
    pub enabled: bool,
    pub last_value: T,
}

const CAMERA_OFFSET: Vec3 = const_vec3!([-10.0, 10.0, -10.0]);

fn add_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 1.5;
    camera.transform = Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera).insert(Smoother {
        smoothness: 4.0,
        enabled: true,
        last_value: CAMERA_OFFSET,
    });
}

fn camera_follow(
    mut transforms: ParamSet<(
        Query<(&mut Transform, Option<&mut Smoother<Vec3>>), With<Camera3d>>,
        Query<&Transform, With<CameraTarget>>,
    )>,
    time: Res<Time>,
) {
    // Get the middle of all the camera targets.
    let mut middle_target = Vec3::ZERO;
    for target_transform in transforms.p1().iter() {
        middle_target += target_transform.translation;
    }
    middle_target /= transforms.p1().iter().count() as f32;

    for (mut transform, smoother) in transforms.p0().iter_mut() {
        let target = middle_target + CAMERA_OFFSET;

        // TODO: Add smoothing.
        match smoother {
            Some(mut smoother) => {
                transform.translation = smoother.last_value.lerp(target, smoother.smoothness * time.delta_seconds());
                smoother.last_value = transform.translation;
            }
            None => transform.translation = target,
        }
    }
}

/// Gets the cursor's position in normalised coordinates (-1, 1)
pub fn cursor_normalised(window: &Window, origin: &WindowOrigin) -> Option<Vec2> {
    window.cursor_position().map(|position| match origin {
        WindowOrigin::Center => {
            (position / Vec2::new(window.width(), window.height()) - Vec2::new(0.5, 0.5))
                // -1.0 to 1.0
                * Vec2::new(2.0, 2.0)
                // Aspect ratio correction.
                * Vec2::new(window.width() / window.height(), 1.0)
        }
        // TODO: Support 0,0 at the bottom left.
        WindowOrigin::BottomLeft => todo!(),
    })
}

/// Convert the cursor's screen position to a camera ray for orthographic projection.
pub fn cursor_to_world_orthographic(
    camera_world: &Transform,
    camera_projection: &OrthographicProjection,
    window: &Window,
) -> Option<Ray> {
    // Get our cursor's position or return None if no cursor exists.
    let cursor_pos = cursor_normalised(window, &camera_projection.window_origin)?;

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
