use crate::entity::BuildingLayer;
use crate::image::Textures;
use crate::map::MapLayer;
use crate::settings::*;
use macroquad::prelude::*;
use std::process::exit;

pub trait LayerMethod {
    fn update(&self, _game: &Game) {
        ()
    }

    fn draw(&self, _game: &Game) {}
}

//region Game
pub struct Game {
    pub camera2d: Camera2D,

    pub texes: Textures,
    pub layers: Vec<Box<dyn LayerMethod>>,

    delay: f32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            camera2d: Camera2D {
                target: vec2(
                    (MAP_SIZE.0 * MAP_TILE_SPACING) as f32 / 2.0,
                    (MAP_SIZE.1 * MAP_TILE_SPACING) as f32 / 2.0,
                ),
                zoom: vec2(
                    5.0 / screen_width() * SCALE.x,
                    5.0 / screen_height() * SCALE.y,
                ),
                ..Default::default()
            },
            texes: Textures::new(),
            layers: vec![],

            delay: 0.0,
        }
    }

    pub async fn run(&mut self) {
        let screen_visible_bound = self.get_screen_visible_bound();

        self.texes.load_all_map_tex().await.unwrap();

        self.layers.push(Box::new(MapLayer::new()));
        self.layers.push(Box::new(BuildingLayer::new()));

        loop {
            self.update(screen_visible_bound);

            set_camera(&self.camera2d);

            for layer in &self.layers {
                layer.update(&self);
                layer.draw(&self);
            }
            next_frame().await;
        }
    }

    fn update(&mut self, screen_visible_bound: Vec2) {
        self.delay = get_frame_time();

        for key in get_keys_down() {
            match key {
                KeyCode::A => self.camera2d.target.x -= CAMERA_SPEED * self.delay,
                KeyCode::D => self.camera2d.target.x += CAMERA_SPEED * self.delay,
                KeyCode::W => self.camera2d.target.y -= CAMERA_SPEED * self.delay,
                KeyCode::S => self.camera2d.target.y += CAMERA_SPEED * self.delay,
                KeyCode::Escape => exit(0),
                _ => (),
            }
        }

        self.camera2d.target.y = clamp(
            self.camera2d.target.y,
            (screen_visible_bound.y / 2.0),
            (MAP_SIZE.1 * MAP_TILE_SPACING) as f32 - screen_visible_bound.y / 2.0,
        );

        self.camera2d.target.x = clamp(
            self.camera2d.target.x,
            (screen_visible_bound.x / 2.0),
            (MAP_SIZE.0 * MAP_TILE_SPACING) as f32 - screen_visible_bound.x / 2.0,
        );
    }

    fn get_screen_visible_bound(&self) -> Vec2 {
        let left_top_pos = self.camera2d.screen_to_world(vec2(0.0, 0.0));
        let right_bottom_pos = self
            .camera2d
            .screen_to_world(vec2(screen_width(), screen_height()));
        right_bottom_pos - left_top_pos
    }
}

//endregion
