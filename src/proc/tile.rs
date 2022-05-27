use bevy::{math::const_vec3, prelude::*};

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

#[derive(Copy, Clone)]
pub enum Orientation {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Component)]
pub struct Tile {
    pub position: IVec3,
    pub rotation: Orientation,
    pub tris: Vec<[u8; 3]>,
}