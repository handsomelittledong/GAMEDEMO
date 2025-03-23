use crate::settings::TerrainType::*;
use macroquad::prelude::*;

pub const GAME_TITLE: &str = "LastLineDemo";
pub const MAP_IMG_PATH: [&str; 2] = ["res/grass.png", "res/gravel2.png"];

pub enum TerrainType {
    Grass = 0,
    Gravel = 1,
    River = 2,
}

impl TryFrom<i32> for TerrainType {
    type Error = &'static str;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Grass),
            1 => Ok(Gravel),
            2 => Ok(River),
            _ => Err("out of range"),
        }
    }
}

impl From<TerrainType> for i32 {
    fn from(tile: TerrainType) -> Self {
        tile as i32
    }
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.w as f32
            && self.x + self.w as f32 > other.x
            && self.y < other.y + other.h as f32
            && self.y + self.h as f32 > other.y
    }
}

pub const MAP_SIZE: Rect = Rect {
    x: 0.0,
    y: 0.0,
    w: 30,
    h: 30,
};
pub const MAP_TILE_SPACING: usize = 64; //for pixels

pub const MAP: [[i32; MAP_SIZE.w]; MAP_SIZE.h] = [[0; MAP_SIZE.w]; MAP_SIZE.h]; //  test

pub const COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

pub const DRAW_TEXTURE_PARAMS: DrawTextureParams = DrawTextureParams {
    dest_size: Some(vec2(MAP_TILE_SPACING as f32, MAP_TILE_SPACING as f32)),
    source: None,
    rotation: 0.0,
    flip_x: false,
    flip_y: false,
    pivot: None,
};

pub const CAMERA_SPEED: f32 = 180.0;
