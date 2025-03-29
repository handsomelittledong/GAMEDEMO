use hecs::World;
use macroquad::prelude::*;
use crate::main_game::{Game, LayerMethod};

trait EntityMethod{ 
    fn update(&mut self){}
    fn draw(&self){}
}

struct Turret{
    texes:Vec<Texture2D>,
    
    hp:f32,
    ap:f32,
}

impl EntityMethod for Turret{
    fn draw(&self) {
        todo!()
    }
    
    fn update(&mut self) {
        todo!()
    }
}

impl Turret{
    fn new()->Self{
        Self{
            texes:vec![],
            
            hp:1.0,
            ap:1.0
        }
    }
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