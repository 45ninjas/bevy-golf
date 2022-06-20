use bevy::{
    math::const_vec3,
    prelude::*,
    render::camera::{Camera3d, CameraProjection, WindowOrigin},
};

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
const CAMERA_OFFSET: Vec3 = const_vec3!([-7.0, 7.0, -7.0]);

fn add_camera(mut commands: Commands) {
    // let mut camera = OrthographicCameraBundle::new_3d();
    // camera.orthographic_projection.scale = 2.0;
    let mut camera = PerspectiveCameraBundle::new_3d();
    camera.perspective_projection.fov = f32::to_radians(15.0);
    camera.transform = Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera).insert(Smoother {
        smoothness: 6.0,
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
    // Our target position is the middle of all Camera targets ignoring the Y axis.
    let mut target_pos = Vec3::ZERO;
    for target_transform in transforms.p1().iter() {
        target_pos += target_transform.translation;
    }
    target_pos /= transforms.p1().iter().count() as f32;
    target_pos.y = 0.0;
    target_pos += CAMERA_OFFSET;

    for (mut transform, smoother) in transforms.p0().iter_mut() {
        match smoother {
            Some(mut smoother) => {
                if smoother.enabled {
                    transform.translation = smoother
                        .last_value
                        .lerp(target_pos, smoother.smoothness * time.delta_seconds());
                    smoother.last_value = transform.translation;
                } else {
                    transform.translation = target_pos;
                }
            }
            None => transform.translation = target_pos,
        }
    }
}

/// Gets the cursor's position in normalised device coordinates (-1, 1)
pub fn cursor_ndc(window: &Window) -> Option<Vec2> {
    let cursor = window.cursor_position()?;
    let size = Vec2::new(window.width(), window.height());
    Some(cursor / size * 2.0 - Vec2::ONE)
}

/// The ray
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

/// Find the intersection point of this ray and an infinite plane.
impl Ray {
    /// Finds where this plane intersects on a plane.
    pub fn intersect_plane(&self, normal: Vec3, origin: Vec3) -> Vec3 {
        // TODO: Check for impossible conditions.
        let origin = origin - self.origin;
        let distance = origin.dot(normal) / self.direction.dot(normal);
        self.origin + self.direction * distance
    }

    /// Creates a ray from a normalised device coordinate.
    /// Mostly copied from the bevy_mod_raycast crate.
    /// ref: https://docs.rs/bevy_mod_raycast/0.5.0/src/bevy_mod_raycast/primitives.rs.html#180
    pub fn from_screenspace(
        cursor_ndc: Option<Vec2>,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Option<Self> {
        let view = camera_transform.compute_matrix();

        // Return none if there's no cursor.
        cursor_ndc?;
        let cursor_ndc = cursor_ndc.unwrap();

        let projection = camera.projection_matrix;

        // 2D Normalized device coordinate cursor position from (-1, -1) to (1, 1)
        // let cursor_ndc = (cursor_ndc / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
        let ndc_to_world: Mat4 = view * projection.inverse();
        let world_to_ndc = projection * view;
        let is_orthographic = projection.w_axis[3] == 1.0;

        // Calculate the camera's near plane using the projection matrix
        let projection = projection.to_cols_array_2d();
        let camera_near = (2.0 * projection[3][2]) / (2.0 * projection[2][2] - 2.0);

        // Compute the cursor position at the near plane. The bevy camera looks at -Z.
        let ndc_near = world_to_ndc.transform_point3(-Vec3::Z * camera_near).z;
        let cursor_pos_near = ndc_to_world.transform_point3(cursor_ndc.extend(ndc_near));

        // Compute the ray's direction depending on the projection used.
        let ray_direction = match is_orthographic {
            true => view.transform_vector3(-Vec3::Z), // All screenspace rays are parallel in ortho
            false => cursor_pos_near - camera_transform.translation, // Direction from camera to cursor
        };

        Some(Ray {
            origin: cursor_pos_near,
            direction: ray_direction,
        })
    }
}
