use std::fmt::Display;
use crate::settings::TerrainType::*;
use lazy_static::lazy_static;
use macroquad::prelude::*;

pub const GAME_TITLE: &str = "LastLineDemo";

lazy_static! {
    pub static ref SCALE: Vec2 = vec2(screen_width() / 1280.0, screen_height() / 720.0);
}

pub const MAP_IMG_PATH:&str = "res/map/";

#[derive(Clone, Copy)]
pub enum TerrainType {
    Grass,
    Gravel,
    Ground,
    River,
}

impl Display for TerrainType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Grass => "grass".to_string(),
            Gravel => "gravel".to_string(),
            River => "river".to_string(),
            Ground=>"ground".to_string(),
        };
        write!(f, "{}", str)
    }
}


#[derive(Copy, Clone)]
pub struct Tile{
    pub terrain_type: TerrainType,
    pub tex_id:usize,
}

impl Tile{
    const fn new(terrain_type: TerrainType,tex_id:usize)->Self{
        Tile{terrain_type,tex_id}
    }
}

macro_rules! tile {
    ($terrain:ident, $tex_id:expr) => {
        Tile::new(TerrainType::$terrain, $tex_id)
    };
}

pub const MAP_SIZE: (usize, usize) = (30, 30);
pub const MAP_TILE_SPACING: usize = 32; //for pixels

pub const  MAP: [[Tile; MAP_SIZE.0]; MAP_SIZE.1] = [[tile!(Ground,0); MAP_SIZE.0]; MAP_SIZE.1]; //  test

pub const COLOR: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

pub const CAMERA_SPEED: f32 = 180.0;
