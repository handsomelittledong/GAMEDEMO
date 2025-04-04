use crate::settings::TerrainType::*;
use lazy_static::lazy_static;
use macroquad::prelude::*;

pub const GAME_TITLE: &str = "LastLineDemo";

lazy_static! {
    pub static ref SCALE: Vec2 = vec2(screen_width() / 1280.0, screen_height() / 720.0);
}

pub const MAP_IMG_PATH:&str = "res/map/";

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

impl ToString for TerrainType {
    fn to_string(&self) -> String {
        match self{
            Grass => "grass".to_string(),
            Gravel => "gravel".to_string(),
            River => "river".to_string(),
        }
    }
}

impl From<TerrainType> for i32 {
    fn from(tile: TerrainType) -> Self {
        tile as i32
    }
}

pub const MAP_SIZE: (usize, usize) = (30, 30);
pub const MAP_TILE_SPACING: usize = 32; //for pixels

pub const MAP: [[i32; MAP_SIZE.0]; MAP_SIZE.1] = [[0; MAP_SIZE.0]; MAP_SIZE.1]; //  test

pub const COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

pub const CAMERA_SPEED: f32 = 180.0;
