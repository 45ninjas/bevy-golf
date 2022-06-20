use serde::Deserialize;

use bevy::{math::const_vec3, prelude::*, reflect::TypeUuid};

pub const TILE_BOUNDS: Vec3 = const_vec3!([1.0, 0.5, 1.0]);
pub const TILE_VERTS: &[Vec3] = &[
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