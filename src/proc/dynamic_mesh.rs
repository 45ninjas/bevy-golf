use bevy::{prelude::*, render::mesh::Indices};
use bevy_rapier3d::prelude::*;

pub struct DynamicMeshPlugin;
impl Plugin for DynamicMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mesh_on_changed);
        app.add_system(collider_on_changed);
    }
}

/// Contains the vertices, uvs and triangles used for building meshes.
#[derive(Component, Default)]
pub struct DynamicMesh {
    triangles: Vec<[Vertex; 3]>,
}

impl DynamicMesh {
    pub fn new() -> DynamicMesh {
        DynamicMesh {
            ..Default::default()
        }
    }
    /// Clears the mesh maker.
    pub fn clear(&mut self) {
        self.triangles.clear();
    }

    /// Gets the total triangle count.
    pub fn tri_count(&self) -> usize {
        self.triangles.len()
    }

    /// Inserts a triangle.
    pub fn insert_tri(&mut self, positions: [Vec3; 3]) {
        let normal = (positions[1] - positions[0])
            .cross(positions[2] - positions[0])
            .normalize();

        let mut tri: [Vertex; 3] = [Vertex { ..default() }; 3];

        for i in 0..3 {
            tri[i] = Vertex {
                normal: normal,
                position: positions[i],
                uv: Vec2::ZERO,
            };
        }
        self.triangles.push(tri);
    }

    /// Reduce the vertices and triangles with different rules.
    pub fn distil(&self, rule: &CompareRule) -> (Vec<[u32; 3]>, Vec<Vertex>) {

        // Our new 'mesh'
        let mut triangles = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();

        // Go over each vertex in each triangle
        for triangle in self.triangles.iter() {
            let mut indices = [0; 3];
            for i in 0..3 {

                // Get the index of an existing or similar vertex based on our rule.
                let index = vertices.iter().position(|&x| triangle[i].compare(&x, rule));

                // Set the index of the current vertex to the existing or new index.
                indices[i] = match index {
                    Some(index) => index as u32,
                    None => {
                        vertices.push(triangle[i]);
                        vertices.len() as u32 - 1
                    }
                };
            }
            triangles.push(indices);
        }

        (triangles, vertices)
    }

    /// Update an existing mesh.
    pub fn update_mesh(&self, mesh: &mut Mesh) {
        let (tris, verts) = self.distil(&CompareRule::Mesh);
        // Migrate our Vec3 position and normal data to [f32; 3] and generate our uvs.
        // TODO: learn how to do this more elegantly like how indices/triangles works.
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        for v in verts.iter() {
            vertices.push([v.position.x, v.position.y, v.position.z]);
            normals.push([v.normal.x, v.normal.y, v.normal.z]);
            // uvs.push([v.position.x, v.position.z]);
            uvs.push([v.uv.x, v.uv.y]);
        }

        let mut triangles = Vec::new();

        for tri in tris.iter() {
            triangles.push(tri[0]);
            triangles.push(tri[1]);
            triangles.push(tri[2]);
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        // Set our triangles.
        let indices = Indices::U32(triangles);
        mesh.set_indices(Some(indices));

        println!(
            "Updated a mesh with {:?} tris and {:?} verts.",
            tris.len(),
            verts.len()
        );
    }

    pub fn collider(&self) -> Collider {
        let (tris, verts) = self.distil(&CompareRule::Collider);
        let mut vertices = Vec::new();
        for v in verts.iter() {
            vertices.push(v.position);
        }
        println!(
            "Updated a collider with {:?} tris and {:?} verts.",
            tris.len(),
            vertices.len()
        );
        Collider::trimesh(vertices, tris)
    }
}

//// Updates a Mesh when the DynamicMesh has been changed.
fn mesh_on_changed(
    mut assets: ResMut<Assets<Mesh>>,
    query: Query<(&Handle<Mesh>, &DynamicMesh), Changed<DynamicMesh>>,
) {
    for (handle, dynamic) in query.iter() {
        if let Some(mesh) = assets.get_mut(handle) {
            dynamic.update_mesh(mesh);
        }
    }
}

/// Replaces an existing collider with one derived from our changed DynamicMesh.
fn collider_on_changed(
    mut commands: Commands,
    query: Query<(Entity, &DynamicMesh, &Collider), Changed<DynamicMesh>>,
) {
    for (ent, dynamic, _) in query.iter() {
        if dynamic.tri_count() > 0 {
            commands
                .entity(ent)
                .remove::<Collider>()
                .insert(dynamic.collider());
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

pub enum CompareRule {
    Mesh,
    MeshNoUV,
    Collider,
}

impl Vertex {
    fn compare(&self, other: &Vertex, rule: &CompareRule) -> bool {
        const MAX_ABS_DIFF: f32 = 0.01;

        match rule {
            CompareRule::Mesh => self.eq(other),
            CompareRule::Collider => self.position.abs_diff_eq(other.position, MAX_ABS_DIFF),
            CompareRule::MeshNoUV => {
                self.position.abs_diff_eq(other.position, MAX_ABS_DIFF)
                    && self.normal.abs_diff_eq(other.normal, MAX_ABS_DIFF)
            }
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        // How different can the normals and positions be?
        const MAX_ABS_DIFF: f32 = 0.01;

        self.position.abs_diff_eq(other.position, MAX_ABS_DIFF)
            && self.normal.abs_diff_eq(other.normal, MAX_ABS_DIFF)
            && self.uv.abs_diff_eq(other.uv, MAX_ABS_DIFF)
    }
}
