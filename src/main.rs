use bevy::{prelude::*, asset::AssetServerSettings, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use bevy_rapier3d::prelude::*;
use bevy_prototype_debug_lines::*;
mod proc;
mod ball;
mod camera;

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(proc::ProcPlugin)
        .add_plugin(ball::BallPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Set the clear colour.
    // TODO: use skybox or gradient
    commands.insert_resource(ClearColor(Color::hex("668093").unwrap()));

    // Setup our lighting with slightly purple ambient and slightly orange directional light.
    commands.insert_resource(AmbientLight {
        color: Color::hex("CDB8E6").unwrap(),
        brightness: 0.2,
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
