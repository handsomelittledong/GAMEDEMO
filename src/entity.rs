use hecs::World;
use macroquad::prelude::*;
use crate::main_game::{Game, LayerMethod};

trait EntityMethod{ 
    fn update(&mut self){}
    fn draw(&self){}
}

struct Turret{
    texes:Texture2D,
    
    hp:f32,
    ap:f32,
}

//region BuildingLayer
pub struct BuildingLayer {
    world: World,
}

impl BuildingLayer {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}

impl LayerMethod for BuildingLayer {
    fn draw(&self, game: &Game) {
        // todo!()
    }

    fn update(&self, game: &Game) {
        if is_mouse_button_pressed(MouseButton::Left) {
            //todo!()
        }
    }
}
//endregion