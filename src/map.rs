use macroquad::math::vec2;
use macroquad::miniquad::FilterMode;
use crate::settings::*;
use macroquad::prelude::{draw_texture_ex, screen_height, screen_width, Vec2};
use crate::main_game::{Game, LayerMethod};

pub fn get_tile_id(world_pos: Vec2) -> (usize, usize) {
    let mut temp_x_id = (world_pos.x as usize) / MAP_TILE_SPACING;
    let mut temp_y_id = (world_pos.y as usize) / MAP_TILE_SPACING;
    if temp_x_id == MAP_SIZE.w {
        temp_x_id = MAP_SIZE.w - 1;
    }
    if temp_y_id == MAP_SIZE.h {
        temp_y_id = MAP_SIZE.h - 1;
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
                    .get_map_tex(TerrainType::try_from(MAP[x][y]).unwrap());
                temp.set_filter(FilterMode::Nearest);
                draw_texture_ex(
                    temp,
                    (x * MAP_TILE_SPACING) as f32,
                    (y * MAP_TILE_SPACING) as f32,
                    COLOR,
                    DRAW_TEXTURE_PARAMS,
                );
            }
        }
    }
}
//endregion

