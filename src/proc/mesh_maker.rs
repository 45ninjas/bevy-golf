use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::{Orientation, Tile, FLOOR_UV, TILE_BOUNDS, TILE_VERTS};

#[derive(Default)]
pub struct MeshMaker {
    pub verts: Vec<Vertex>,
    pub uvs: Vec<Vec2>,
    pub triangles: Vec<u32>,
}

impl MeshMaker {
    pub fn insert_tile(&mut self, tile: &Tile) {
        // Rotate our tris.
        let tris = orient_tris(&tile.tris, &tile.rotation);
        for tri in tris.iter() {
            // Calculate the surface normal from our triangle.
            let a = TILE_VERTS[tri[0] as usize] * TILE_BOUNDS;
            let b = TILE_VERTS[tri[1] as usize] * TILE_BOUNDS;
            let c = TILE_VERTS[tri[2] as usize] * TILE_BOUNDS;
            let normal = (b - a).cross(c - a).normalize();

            // Write our new verts.
            for v in tri {
                let vert = Vertex {
                    normal: normal,
                    position: (tile.position.as_vec3() + TILE_VERTS[*v as usize]) * TILE_BOUNDS,
                };
                // If this is a new vert, add it.
                if !self.verts.contains(&vert) {
                    self.verts.push(vert);
                    self.uvs.push(Vec2::new(vert.position.x, vert.position.z));
                }

                // Get the index of our vert from verts and push to triangles.
                let index = self.verts.iter().position(|&x| x == vert).unwrap();
                let index = u32::try_from(index).unwrap();

                self.triangles.push(index);
            }
        }
    }
    pub fn as_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        self.update_mesh(&mut mesh);
        mesh
    }
    pub fn update_mesh(&self, mesh: &mut Mesh) {
        // Create our triangles, vertices, uvs and normals.
        let indices = Indices::U32(self.triangles.clone());
        let mut vertices = Vec::new();
        let mut normals = Vec::new();

        // Migrate our data from mesh maker (because I don't know of a better way yet)
        for v in self.verts.iter() {
            vertices.push([v.position.x, v.position.y, v.position.z]);
            normals.push([v.normal.x, v.normal.y, v.normal.z]);
        }

        let mut uvs = Vec::new();
        for uv in self.uvs.iter() {
            uvs.push([uv.x, uv.y]);
        }

        // Log to confirm that our duplicate remove process works.
        println!("Total Vertices: {:?}", vertices.len());

        // Actually create the mesh.

        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }
}

fn rotate_tris(tris: &mut Vec<[u8; 3]>) {
    for tri in tris.iter_mut() {
        for vert in tri.iter_mut() {
            if *vert == 3 {
                *vert = 0;
            } else if *vert == 7 {
                *vert = 4;
            } else {
                *vert += 1;
            }
        }
    }
}
fn orient_tris(tris: &Vec<[u8; 3]>, orientation: &Orientation) -> Vec<[u8; 3]> {
    let mut new_tris = tris.clone();
    for _ in 0..(*orientation as u8) {
        for tri in new_tris.iter_mut() {
            for vert in tri.iter_mut() {
                if *vert == 3 {
                    *vert = 0;
                } else if *vert == 7 {
                    *vert = 4;
                } else {
                    *vert += 1;
                }
            }
        }
    }
    new_tris
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        // How different can the normals and positions be?
        const MAX_ABS_DIFF: f32 = 0.01;

        self.position.abs_diff_eq(other.position, MAX_ABS_DIFF)
            && self.normal.abs_diff_eq(other.normal, MAX_ABS_DIFF)
    }
}
