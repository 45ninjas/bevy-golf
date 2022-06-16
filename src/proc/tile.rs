use std::vec;

use serde::Deserialize;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
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
#[derive(Debug, Deserialize, TypeUuid, Component)]
#[uuid = "aa5fc0fb-722d-4d8f-b0cd-9526f1a0e75e"]
pub struct Tile {
    pub position: IVec3,
    pub rotation: Orientation,
    pub tile_type: u8,
}

#[derive(Debug)]
pub struct Edge(pub Vec3, pub Vec3);
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        const MAX_ABS_DIFF: f32 = 0.01;
        (self.0.abs_diff_eq(other.0, MAX_ABS_DIFF) && self.1.abs_diff_eq(other.1, MAX_ABS_DIFF))
            || (self.0.abs_diff_eq(other.1, MAX_ABS_DIFF)
                && self.1.abs_diff_eq(other.0, MAX_ABS_DIFF))
    }
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "b43e6937-97e2-4fb9-9146-16f894bf814d"]
pub struct TileDefinition {
    pub id: u8,
    pub perimeter: Vec<u8>,
    pub name: String,
}

impl TileDefinition {
    pub fn triangles(&self) -> Option<Vec<[u8; 3]>> {
        // None when there's not enough points in the perimeter to create a triangle.
        if self.perimeter.len() < 3 {
            return None;
        }

        let mut triangles = vec![[self.perimeter[0], self.perimeter[1], self.perimeter[2]]];

        // Create the 2nd triangle with the first point of the first triangle.
        if self.perimeter.len() == 4 {
            triangles.push([self.perimeter[0], self.perimeter[2], self.perimeter[3]]);
        }

        Some(triangles)
    }

    pub fn edges(&self) -> Option<Vec<[u8; 2]>> {
        // None when there's not enough points in the perimeter to create an edge.
        if self.perimeter.is_empty() {
            return None;
        }

        let mut edges = Vec::new();

        // Add each pair.
        for i in 1..self.perimeter.len() {
            edges.push([self.perimeter[i - 1], self.perimeter[i]]);
        }
        // Add the edge that completes the loop.
        edges.push([self.perimeter[self.perimeter.len() - 1], self.perimeter[0]]);

        Some(edges)
    }
}

impl Default for TileDefinition {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Error: Unknown"),
            perimeter: Default::default(),
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
            let array = ron::de::from_bytes::<Vec<TileDefinition>>(bytes)?;
            let asset = TileDefinitions(array);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
