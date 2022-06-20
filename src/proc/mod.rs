use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use tile::*;
use tile_definitions::*;

mod dynamic_mesh;
pub mod tile_definitions;
pub mod tile;
// use self::mesh_maker::MeshMaker;
use self::dynamic_mesh::{DynamicMesh, DynamicMeshPlugin};

pub struct ProcPlugin;

impl Plugin for ProcPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TileDefinitions>();
        app.init_asset_loader::<TileDefinitionsLoader>();
        app.add_plugin(DynamicMeshPlugin);
        app.add_startup_system(add_ground);
        app.add_startup_system(add_walls);
        app.add_event::<UpdateGroundEvent>();
        app.add_system(update_ground);
        app.add_system(update_walls);
        app.add_system(reload_tile_defs);
        app.add_startup_system(add_tiles);
    }
}

#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct Wall;

fn add_tiles(mut commands: Commands, mut ev_update_ground: EventWriter<UpdateGroundEvent>) {
    commands.spawn().insert(Tile {
        position: IVec3::ZERO,
        rotation: Orientation::North,
        tile_type: 1
    });
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
        position: IVec3::new(0, 0, -1),
        rotation: Orientation::North,
        tile_type: 6,
    });
    commands.spawn().insert(Tile {
        position: IVec3::new(1, 0, -1),
        rotation: Orientation::North,
        tile_type: 1,
    });
    commands.spawn().insert(Tile {
        position: IVec3::new(2, 0, -1),
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
    let tile_defs: Handle<TileDefinitions> = asset_server.load("tiles.ron");

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
fn add_walls(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let tile_defs: Handle<TileDefinitions> = asset_server.load("tiles.ron");

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Torus {
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("E8B792").unwrap(),
                // base_color_texture: Some(asset_server.load("textures/checker-wood.png")),
                ..default()
            }),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.01, 0.5))
        .insert(DynamicMesh::new())
        .insert(Wall)
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

            // Clear our dynamic mesh.
            dynamic_mesh.clear();

            // TODO: Move this into it's own function.
            // Go over each tile in the world and add them to the dynamic_mesh.
            for tile in tile_query.iter() {
                // If the tile definition for this tile exists, add it's triangles to the mesh.
                if let Some(def) = defs.iter().find(|x| x.id == tile.tile_type) {
                    for triangle in def.triangles().unwrap_or_default() {
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

fn update_walls(
    mut ev_update_ground: EventReader<UpdateGroundEvent>,
    mut wall_query: Query<(&mut DynamicMesh, &Handle<TileDefinitions>), With<Wall>>,
    tile_query: Query<&Tile>,
    defs_asset: Res<Assets<TileDefinitions>>,
) {
    for _ in ev_update_ground.iter() {
        for (mut dynamic_mesh, tile_defs) in wall_query.iter_mut() {
            // Get the tile definitions.
            let defs = match defs_asset.get(tile_defs) {
                Some(defs) => &defs.0, // The defs asset exists.
                // TODO: Figure out how to wait until the asset has loaded (defer the event maybe?)
                None => continue, // The defs asset does not exist. Just continue.
            };

            let mut edges = Vec::new();
            let mut edges_count = Vec::new();

            // Go over each edge in each tile and add them to the list of edges.
            for tile in tile_query.iter() {
                if let Some(def) = defs.iter().find(|x| x.id == tile.tile_type) {
                    for edge_index in def.edges().unwrap_or_default() {
                        // Rotate each edge, while we're iterating add the position;
                        let new_edge = Edge(
                            (TILE_VERTS[rotate_index(edge_index[0], &tile.rotation) as usize]
                                + tile.position.as_vec3())
                                * TILE_BOUNDS,
                            (TILE_VERTS[rotate_index(edge_index[1], &tile.rotation) as usize]
                                + tile.position.as_vec3())
                                * TILE_BOUNDS,
                        );

                        match edges.iter().position(|x| new_edge.eq(x)) {
                            // If this edge already exists, increment our edge counter for this edge.
                            Some(index) => {
                                edges_count[index] = edges_count[index] + 1;
                            }
                            // Else add a new edge counter entry and the new edge.
                            None => {
                                // Add our edge.
                                edges.push(new_edge);
                                edges_count.push(1);
                            }
                        }
                    }
                }
            }

            // Clear our dynamic mesh.
            dynamic_mesh.clear();

            let mut total_edges = 0;
            for i in 0..edges.len() {
                let edge = &edges[i];
                let a = edge.0;
                let b = edge.1;
                const WALL_SIZE: f32 = 0.08;

                let up = Vec3::Y * WALL_SIZE;
                let btm = Vec3::Y * -TILE_BOUNDS.y;
                let inside = (a - b).normalize().cross(Vec3::Y) * WALL_SIZE;

                // If there's one edge (no duplicated edges) then place our edge.
                if edges_count[i] == 1 {
                    total_edges += 1;
                    // Outside wall
                    dynamic_mesh.insert_tri([b + up, a + up, a + btm]);
                    dynamic_mesh.insert_tri([a + btm, b + btm, b + up]);

                    // Top face
                    dynamic_mesh.insert_tri([b + up + inside, a + up, b + up]);
                    dynamic_mesh.insert_tri([b + up + inside, a + up + inside, a + up]);

                    // Inside face
                    dynamic_mesh.insert_tri([b + up + inside, b + btm + inside, a + btm + inside]);
                    dynamic_mesh.insert_tri([b + up + inside, a + btm + inside, a + up + inside]);
                }
            }
            println!("{:?} Edges total", total_edges);
        }
    }
}

fn reload_tile_defs(
    mut ev_update_ground: EventWriter<UpdateGroundEvent>,
    mut ev_assets: EventReader<AssetEvent<TileDefinitions>>,
) {
    for ev in ev_assets.iter() {
        match ev {
            AssetEvent::Created { handle: _ } => {}
            AssetEvent::Modified { handle: _ } => {
                ev_update_ground.send_default();
            }
            AssetEvent::Removed { handle: _ } => {}
        }
    }
}

#[derive(Default)]
struct UpdateGroundEvent;
