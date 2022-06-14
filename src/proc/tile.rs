use serde::Deserialize;

use std::hash::{Hash, Hasher};

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    math::const_vec3,
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};

pub const TILE_BOUNDS: Vec3 = const_vec3!([1.0, 0.5, 1.0]);
pub const TILE_VERTS: &'static [Vec3] = &[
    const_vec3!([0.5, -1.0, -0.5]),  // 0
    const_vec3!([0.5, -1.0, 0.5]),   // 1
    const_vec3!([-0.5, -1.0, 0.5]),  // 2
    const_vec3!([-0.5, -1.0, -0.5]), // 3
    const_vec3!([0.5, 0.0, -0.5]),   // 4
    const_vec3!([0.5, 0.0, 0.5]),    // 5
    const_vec3!([-0.5, 0.0, 0.5]),   // 6
    const_vec3!([-0.5, 0.0, -0.5]),  // 7
];

#[derive(Copy, Clone, Debug, Deserialize, TypeUuid)]
#[uuid = "879067aa-3b4d-4144-aed2-a6f9ab701655"]
pub enum Orientation {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub fn rotate_index(index: u8, orientation: &Orientation) -> u8 {
    let rotations = *orientation as u8;
    if index >= 4 {
        4 + (rotations + index) % 4
    } else {
        (rotations + index) % 4
    }
}

pub fn index_to_lower(index: u8) -> u8 {
    index % 4
}

#[derive(Debug, Deserialize, TypeUuid, Component)]
#[uuid = "aa5fc0fb-722d-4d8f-b0cd-9526f1a0e75e"]
pub struct Tile {
    pub position: IVec3,
    pub rotation: Orientation,
    pub tile_type: u8,
}

#[derive(Debug)]
pub struct Edge<T>(pub T, pub T);
impl PartialEq for Edge<u8> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.1 == other.1 && self.0 == other.0)
    }
}
impl PartialEq for Edge<Vec3> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.1 == other.1 && self.0 == other.0)
    }
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "b43e6937-97e2-4fb9-9146-16f894bf814d"]
pub struct TileDefinition {
    pub id: u8,
    pub name: String,
    pub triangles: Vec<[u8; 3]>,

    #[serde(skip)]
    pub edges: Vec<Edge<u8>>,
}

impl Default for TileDefinition {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Error: Unknown"),
            triangles: Default::default(),
            edges: Default::default(),
        }
    }
}

impl TileDefinition {
    fn compute_edges(&mut self) {
        // Add the indices of all the edges on the triangle.
        self.edges.clear();
        for tri in self.triangles.iter() {
            for i in 0..2 {
                let edge = Edge(tri[i], tri[(i + 1) % 3]);

                if !self.edges.contains(&edge) {
                    self.edges.push(edge);
                }
            }
        }
    }
}

#[derive(Default, Debug, Deserialize, TypeUuid)]
#[uuid = "74e0d658-5507-4195-9222-dff94b6839f3"]
pub struct TileDefinitions(pub Vec<TileDefinition>);

#[derive(Default)]
pub struct TileDefinitionsLoader;

impl AssetLoader for TileDefinitionsLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut array = ron::de::from_bytes::<Vec<TileDefinition>>(bytes)?;
            for def in array.iter_mut() {
                def.compute_edges();
            }
            let asset = TileDefinitions(array);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
