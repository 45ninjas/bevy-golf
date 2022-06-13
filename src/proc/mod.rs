use bevy::{asset::AssetLoader, prelude::*};
use bevy_rapier3d::prelude::*;

use tile::*;

mod dynamic_mesh;
mod tile;
// use self::mesh_maker::MeshMaker;
use self::dynamic_mesh::{DynamicMesh, DynamicMeshPlugin};

pub struct ProcPlugin;

impl Plugin for ProcPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TileDefinitions>();
        app.init_asset_loader::<TileDefinitionsLoader>();
        app.add_plugin(DynamicMeshPlugin);
        app.add_startup_system(add_ground);
        app.add_event::<UpdateGroundEvent>();
        app.add_system(update_ground);
        app.add_startup_system(add_tiles);
    }
}

#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct Wall;

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
    let tile_defs: Handle<TileDefinitions> = asset_server.load("data/tiles.ron");

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
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(DynamicMesh::new())
        .insert(Ground)
        .insert(tile_defs.clone());
}

// TODO: Track added/changed tiles instead of firing events.
/// Updates the ground's dynamic mesh.
fn update_ground(
    mut ev_update_ground: EventReader<UpdateGroundEvent>,
    mut ground_query: Query<(&mut DynamicMesh, &Handle<TileDefinitions>), With<Ground>>,
    tile_query: Query<&Tile>,
    defs_asset: Res<Assets<TileDefinitions>>,
) {
    for _ in ev_update_ground.iter() {
        for (mut dynamic_mesh, tile_defs) in ground_query.iter_mut() {
            // Get the tile definitions.
            let defs = match defs_asset.get(tile_defs) {
                Some(defs) => &defs.0, // The defs asset exists.
                // TODO: Figure out how to wait until the asset has loaded (defer the event maybe?)
                None => continue, // The defs asset does not exist. Just continue.
            };

            // TODO: Move this into it's own function.
            // Go over each tile in the world and add them to the dynamic_mesh.
            for tile in tile_query.iter() {
                // If the tile definition for this tile exists, add it's triangles to the mesh.
                if let Some(def) = defs.iter().find(|x| x.id == tile.tile_type) {
                    for triangle in &def.triangles {
                        let mut positions: [Vec3; 3] = [Vec3::ZERO; 3];

                        // Rotate each index. Also, while we're iterating add the position.
                        for i in 0..3 {
                            let vert = TILE_VERTS
                                [tile::rotate_index(triangle[i], &tile.rotation) as usize];
                            positions[i] = (vert + tile.position.as_vec3()) * TILE_BOUNDS;
                        }

                        // Add our 3 positions to make a triangle.
                        dynamic_mesh.insert_tri(positions);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct UpdateGroundEvent;