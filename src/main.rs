use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod proc;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(proc::ProcPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene (mut commands: Commands) {

    // Set the clear colour.
    // TODO: use skybox or gradient
    commands.insert_resource(ClearColor(Color::hex("668093").unwrap()));


    // Setup our lighting with slightly purple ambient and slightly orange directional light.
    commands.insert_resource(AmbientLight {
        color: Color::hex("CDB8E6").unwrap(),
        brightness: 0.1
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::hex("FFF7D8").unwrap(),
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.2, 0.1, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Create the camera.
    // TODO: move creation of camera into camera module.
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 3.0;
    camera.transform = Transform::from_xyz(-10.0, 10.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera);
}