use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::{Orientation, Tile, TILE_BOUNDS, TILE_VERTS};

#[derive(Default)]
/// Contains the vertices, uvs and triangles used for building meshes.
pub struct MeshMaker {
    pub verts: Vec<Vertex>,
    pub triangles: Vec<u32>,
}

impl MeshMaker {
    /// Inserts a tile into the mesh maker apply rotations and removing duplicated verts.
    pub fn insert_tile(&mut self, tile: &Tile) {
        // Rotate our triangles then iterate over each triangle.
        for tri in orient_indices(&tile.tris, &tile.rotation).iter() {
            // Calculate the surface normal for this triangle.
            let a = TILE_VERTS[tri[0] as usize] * TILE_BOUNDS;
            let b = TILE_VERTS[tri[1] as usize] * TILE_BOUNDS;
            let c = TILE_VERTS[tri[2] as usize] * TILE_BOUNDS;
            let normal = (b - a).cross(c - a).normalize();

            for index in tri {
                let vertex = Vertex {
                    normal: normal,
                    position: (tile.position.as_vec3() + TILE_VERTS[*index as usize]) * TILE_BOUNDS,
                };
                // If this is a new vert, add it.
                if !self.verts.contains(&vertex) {
                    self.verts.push(vertex);
                }

                // Get the index of our vert from verts and push to triangles.
                let index = self.verts.iter().position(|&x| x == vertex).unwrap();
                let index = u32::try_from(index).unwrap();

                self.triangles.push(index);
            }
        }
    }

    /// Create a new mesh from this mesh maker.
    pub fn as_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        self.update_mesh(&mut mesh);
        mesh
    }

    /// Update an existing mesh.
    pub fn update_mesh(&self, mesh: &mut Mesh) {
        // Migrate our Vec3 position and normal data to [f32; 3] and generate our uvs.
        // TODO: learn how to do this more elegantly like how indices/triangles works.
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for v in self.verts.iter() {
            vertices.push([v.position.x, v.position.y, v.position.z]);
            normals.push([v.normal.x, v.normal.y, v.normal.z]);
            uvs.push([v.position.x, v.position.z]);
        }

        // Log to confirm that our duplicate remove process works.
        // println!("Total Vertices: {:?}", vertices.len());

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        // Set our triangles.
        let indices = Indices::U32(self.triangles.clone());
        mesh.set_indices(Some(indices));
    }
}

/// Rotate triangles
fn orient_indices(indices: &Vec<[u8; 3]>, orientation: &Orientation) -> Vec<[u8; 3]> {
    // To rotate triangles we can increment each index on all triangles.
    // To prevent bottom indices becoming top indices and vice versa we
    // need to repeat the top and bottom indices separately

    let mut new_indices = indices.clone();

    // Iterate for the amount of rotation operations required.
    for _ in 0..(*orientation as u8) {
        // Get every index and increment wrap each index keeping the upper and lower indices separated.
        for index in new_indices.iter_mut().flatten() {
            if *index == 3 {
                *index = 0;
            } else if *index == 7 {
                *index = 4;
            } else {
                *index += 1;
            }
        }
    }
    new_indices
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
