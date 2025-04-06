use crate::main_game::{Game, LayerMethod};
use crate::settings::*;
use macroquad::math::vec2;
use macroquad::prelude::{draw_texture, Vec2};
use macroquad::window::{screen_height, screen_width};

pub fn get_tile_id(world_pos: Vec2) -> (usize, usize) {
    let mut temp_x_id = (world_pos.x as usize) / MAP_TILE_SPACING;
    let mut temp_y_id = (world_pos.y as usize) / MAP_TILE_SPACING;
    if temp_x_id == MAP_SIZE.0 {
        temp_x_id = MAP_SIZE.0 - 1;
    }
    if temp_y_id == MAP_SIZE.1 {
        temp_y_id = MAP_SIZE.1 - 1;
    }
    (temp_x_id, temp_y_id)
}

//region MapLayer
pub struct MapLayer {}

impl MapLayer {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl LayerMethod for MapLayer {
    fn draw(&self, game: &Game) {
        let left_top_pos = game.camera2d.screen_to_world(vec2(0.0, 0.0));
        let left_top_tile_id = get_tile_id(left_top_pos);

        let right_bottom_pos = game
            .camera2d
            .screen_to_world(vec2(screen_width(), screen_height()));
        let right_bottom_tile_id = get_tile_id(right_bottom_pos);
        for x in left_top_tile_id.0..=right_bottom_tile_id.0 {
            for y in left_top_tile_id.1..=right_bottom_tile_id.1 {
                let temp = game
                    .texes
                    .get_map_tex(MAP[x][y].terrain_type,MAP[x][y].tex_id);
                draw_texture(
                    temp,
                    (x * MAP_TILE_SPACING) as f32,
                    (y * MAP_TILE_SPACING) as f32,
                    COLOR,
                );
            }
        }
    }
}
//endregion
