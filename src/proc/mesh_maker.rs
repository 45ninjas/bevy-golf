use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::tile::*;

/// Contains the vertices, uvs and triangles used for building meshes.
#[derive(Default)]
pub struct MeshMaker {
    verts: Vec<Vertex>,
    triangles: Vec<u32>,
}

impl MeshMaker {
    /// Clears the mesh maker.
    pub fn clear(&mut self) {
        self.verts.clear();
        self.triangles.clear();
    }

    /// Inserts a triangle.
    pub fn insert_tri(&mut self, positions: [Vec3; 3]) {
        let normal = (positions[1] - positions[0])
            .cross(positions[2] - positions[0])
            .normalize();

        for position in positions {
            let vertex = Vertex {
                normal: normal,
                position: position,
            };
            let index = self.insert(vertex);
            self.triangles.push(index);
        }
    }

    /// Inserts a vertex and returns it's index.
    pub fn insert(&mut self, vertex: Vertex) -> u32 {
        let index = self.verts.iter().position(|&x| x == vertex);
        match index {
            Some(index) => index as u32,
            None => {
                self.verts.push(vertex);
                self.verts.len() as u32 - 1
            }
        }
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
            // normals.push([0.0, 0.0, 1.0]);
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
