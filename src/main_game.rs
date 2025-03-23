use crate::image::Textures;
use crate::map::MapLayer;
use crate::settings::*;
use macroquad::prelude::*;
use std::process::exit;
use crate::entity::BuildingLayer;

pub trait LayerMethod { 
    fn update(&self, _game: &Game) {()}
    
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
                    (MAP_SIZE.w * MAP_TILE_SPACING) as f32 / 2.0,
                    (MAP_SIZE.h * MAP_TILE_SPACING) as f32 / 2.0,
                ),
                zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
                ..Default::default()
            },
            texes: Textures::new(),
            layers: vec![],

            delay: 0.0,
        }
    }

    pub async fn run(&mut self) {
        self.texes.load_all_map_tex().await;

        self.layers.push(Box::new(MapLayer::new()));
        self.layers.push(Box::new(BuildingLayer::new()));

        loop {
            self.update();

            set_camera(&self.camera2d);

            for layer in &self.layers {
                layer.update(&self);
                layer.draw(&self);
            }
            next_frame().await;
        }
    }

    fn update(&mut self) {
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

        if self.camera2d.target.y > (MAP_SIZE.h * MAP_TILE_SPACING) as f32 - screen_height() / 2.0 {
            self.camera2d.target.y = (MAP_SIZE.h * MAP_TILE_SPACING) as f32 - screen_height() / 2.0;
        } else if self.camera2d.target.y < screen_height() / 2.0 {
            self.camera2d.target.y = screen_height() / 2.0;
        }
        if self.camera2d.target.x > (MAP_SIZE.w * MAP_TILE_SPACING) as f32 - screen_width() / 2.0 {
            self.camera2d.target.x = (MAP_SIZE.w * MAP_TILE_SPACING) as f32 - screen_width() / 2.0;
        } else if self.camera2d.target.x < screen_width() / 2.0 {
            self.camera2d.target.x = screen_width() / 2.0;
        }
    }
}

//endregion
