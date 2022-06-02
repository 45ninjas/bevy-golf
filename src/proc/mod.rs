use bevy::{asset::AssetLoader, prelude::*};
use bevy_rapier3d::prelude::*;

use tile::*;

mod mesh_maker;
mod tile;
use self::mesh_maker::MeshMaker;

pub struct ProcPlugin;

impl Plugin for ProcPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TileDefinitions>();
        app.init_resource::<MeshMaker>();
        app.init_asset_loader::<TileDefinitionsLoader>();
        app.add_startup_system(add_ground);
        app.add_event::<UpdateGroundEvent>();
        app.add_system(generate_ground);
        app.add_startup_system(add_tiles);
    }
}

#[derive(Component)]
pub struct Ground;

fn add_tiles(mut commands: Commands, mut ev_update_ground: EventWriter<UpdateGroundEvent>) {
    commands.spawn().insert(Tile {
        position: IVec3::new(2, 1, 0),
        rotation: Orientation::North,
        tile_type: 1,
    });
    commands.spawn().insert(Tile {
        position: IVec3::new(1, 1, 0),
        rotation: Orientation::North,
        tile_type: 3,
    });
    commands.spawn().insert(Tile {
        position: IVec3::new(0, 0, 0),
        rotation: Orientation::North,
        tile_type: 1,
    });
    commands.spawn().insert(Tile {
        position: IVec3::new(-1, 0, 0),
        rotation: Orientation::West,
        tile_type: 4,
    });

    commands.spawn().insert(Tile {
        position: IVec3::new(-1, 0, -1),
        rotation: Orientation::East,
        tile_type: 3,
    });

    commands.spawn().insert(Tile {
        position: IVec3::new(-1, -1, -2),
        rotation: Orientation::East,
        tile_type: 4,
    });

    commands.spawn().insert(Tile {
        position: IVec3::new(-2, -1, -2),
        rotation: Orientation::North,
        tile_type: 2,
    });

    ev_update_ground.send_default();
}

fn add_ground(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let tile_def_handle: Handle<TileDefinitions> = asset_server.load("data/tiles.ron");
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Torus {
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("1A7525").unwrap(),
                // base_color_texture: Some(asset_server.load("textures/checker-grass.png")),
                ..default()
            }),
            ..default()
        })
        .insert(Ground)
        .insert(Collider::trimesh(vec![Vec3::X, Vec3::ZERO, Vec3::Z, Vec3::X + Vec3::Z], vec![[0, 1, 2], [0, 3, 2]]))
        .insert(tile_def_handle);
}

#[derive(Default)]
struct UpdateGroundEvent;

/// TODO: Defer this event until the TileDefinitions have loaded.
fn generate_ground(
    mut ev_update_ground: EventReader<UpdateGroundEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ground_query: Query<(&Handle<Mesh>, &Handle<TileDefinitions>, &mut Collider), With<Ground>>,
    tile_query: Query<&Tile>,
    mut mesh_maker: ResMut<MeshMaker>,
    defs_asset: ResMut<Assets<TileDefinitions>>,
) {
    let (mesh_handle, defs_handle, mut collider) = ground_query
        .get_single_mut()
        .expect("A singular Ground doesn't exist.");

    let mesh = meshes
        .get_mut(mesh_handle)
        .expect("Ground has no Mesh to update.");

    let defs = if let Some(defs) = defs_asset.get(defs_handle) {
        defs
    } else {
        return;
    };

    for _ in ev_update_ground.iter() {
        for tile in tile_query.iter() {
            // If the tile definition for this tile exists, add it's triangles to the mesh.
            if let Some(def) = defs.0.iter().find(|x| x.id == tile.tile_type) {
                for triangle in &def.triangles {
                    let mut positions: [Vec3; 3] = [Vec3::ZERO; 3];

                    // Rotate each index. Also, while we're iterating add the position.
                    for i in 0..3 {
                        let vert =
                            TILE_VERTS[tile::rotate_index(triangle[i], &tile.rotation) as usize];

                        positions[i] = (vert + tile.position.as_vec3()) * TILE_BOUNDS;
                    }

                    // Add our 3 positions to make a triangle.
                    mesh_maker.insert_tri(positions);
                }
            }
        }
        mesh_maker.update_mesh(mesh);
        let mut trimesh = mesh_maker.trimesh();
        let mut tm = collider.as_trimesh().expect("Not a trimesh!");
        tm.raw = &trimesh;
        collider
    }
}
