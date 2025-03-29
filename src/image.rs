use crate::settings::{TerrainType, MAP_IMG_PATH};
use macroquad::prelude::{load_texture, Texture2D};

pub struct Textures {
    terrain_texes: Vec<Texture2D>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            terrain_texes: Vec::<Texture2D>::new(),
        }
    }

    pub async fn load_all_map_tex(&mut self) {
        for img in MAP_IMG_PATH {
            self.terrain_texes.push(load_texture(img).await.unwrap())
        }
    }

    pub fn get_map_tex(&self, id: TerrainType) -> &Texture2D {
        self.terrain_texes.get(id as usize).unwrap()
    }
}
