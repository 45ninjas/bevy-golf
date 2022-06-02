use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod proc;
mod ball;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(proc::ProcPlugin)
        .add_plugin(ball::BallPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_startup_system(setup_scene)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Set the clear colour.
    // TODO: use skybox or gradient
    commands.insert_resource(ClearColor(Color::hex("668093").unwrap()));

    // Setup our lighting with slightly purple ambient and slightly orange directional light.
    commands.insert_resource(AmbientLight {
        color: Color::hex("CDB8E6").unwrap(),
        brightness: 0.1,
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::hex("FFF7D8").unwrap(),
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.2, 1.0, 0.2).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    // commands
    //     .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, -0.0, 0.0)))
    //     .insert(Collider::cuboid(200.0, 0.1, 200.0));
}
