use bevy::{math::const_vec3, prelude::*};

mod mesh_maker;
use self::mesh_maker::MeshMaker;

const TILE_BOUNDS: Vec3 = const_vec3!([1.0, 0.5, 1.0]);
const TILE_VERTS: &'static [Vec3] = &[
    const_vec3!([0.5, -1.0, -0.5]),  // 0
    const_vec3!([0.5, -1.0, 0.5]),   // 1
    const_vec3!([-0.5, -1.0, 0.5]),  // 2
    const_vec3!([-0.5, -1.0, -0.5]), // 3
    const_vec3!([0.5, 0.0, -0.5]),   // 4
    const_vec3!([0.5, 0.0, 0.5]),    // 5
    const_vec3!([-0.5, 0.0, 0.5]),   // 6
    const_vec3!([-0.5, 0.0, -0.5]),  // 7
];

#[derive(Copy, Clone)]
pub enum Orientation {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Component)]
pub struct Tile {
    pub position: IVec3,
    pub rotation: Orientation,
    pub tris: Vec<[u8; 3]>,
}

#[derive(Component)]
pub struct Ground;

fn add_tiles(mut commands: Commands, mut ev_update_ground: EventWriter<UpdateGroundEvent>) {
    commands.spawn().insert(Tile {
        position: IVec3::new(2, 1, 0),
        tris: vec![[4, 6, 5], [4, 7, 6]],
        rotation: Orientation::North,
    });
    commands.spawn().insert(Tile {
        position: IVec3::new(1, 1, 0),
        tris: vec![[4, 2, 5], [4, 3, 2]],
        rotation: Orientation::North,
    });
    commands.spawn().insert(Tile {
        position: IVec3::new(0, 0, 0),
        tris: vec![[4, 6, 5], [4, 7, 6]],
        rotation: Orientation::North,
    });
    commands.spawn().insert(Tile {
        position: IVec3::new(-1, 0, 0),
        tris: vec![[4, 6, 5]],
        rotation: Orientation::West,
    });

    commands.spawn().insert(Tile {
        position: IVec3::new(-1, 0, -1),
        tris: vec![[4, 2, 5], [4, 3, 2]],
        rotation: Orientation::East,
    });

    commands.spawn().insert(Tile {
        position: IVec3::new(-1, -1, -2),
        tris: vec![[4, 6, 5]],
        rotation: Orientation::East,
    });

    commands.spawn().insert(Tile {
        position: IVec3::new(-2, -1, -2),
        tris: vec![[4, 6, 5], [4, 7, 6]],
        rotation: Orientation::North,
    });

    ev_update_ground.send_default();
}

fn add_ground(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("1A7525").unwrap(),
                // base_color_texture: Some(asset_server.load("textures/checker-grass.png")),
                ..default()
            }),
            ..default()
        })
        .insert(Ground);
}

#[derive(Default)]
struct UpdateGroundEvent;

fn generate_ground(
    mut ev_update_ground: EventReader<UpdateGroundEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    tile_query: Query<&Tile>,
    ground_query: Query<&Handle<Mesh>, With<Ground>>,
) {
    for _ in ev_update_ground.iter() {
        let handle = ground_query
            .get_single()
            .expect("Error: could not find a singular ground.");
        let mesh = meshes.get_mut(handle).expect("No Mesh found to update.");

        let mut maker = MeshMaker {
            ..Default::default()
        };
        for tile in tile_query.iter() {
            maker.insert_tile(&tile);
        }

        maker.update_mesh(mesh);
    }
}

pub struct ProcPlugin;

impl Plugin for ProcPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_ground);
        app.add_event::<UpdateGroundEvent>();
        app.add_system(generate_ground);
        app.add_startup_system(add_tiles);
    }
}
