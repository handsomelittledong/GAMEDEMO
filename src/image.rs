use crate::settings::{TerrainType, MAP_IMG_PATH};
use macroquad::prelude::FilterMode::Nearest;
use macroquad::prelude::{load_texture, Texture2D};
use std::collections::HashMap;
use std::fs::read_dir;
use std::io;

pub struct Textures {
    terrain_texes: HashMap<String, Texture2D>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            terrain_texes: HashMap::<String, Texture2D>::new(),
        }
    }

    pub async fn load_all_map_tex(&mut self) -> io::Result<()> {
        for files in read_dir(MAP_IMG_PATH.to_string())? {
            let file_path = format!("{}{}", MAP_IMG_PATH, files?.file_name().to_str().unwrap());
            let tex = load_texture(file_path.as_str())
                .await
                .expect("cannot load map images");
            self.terrain_texes.insert(file_path, tex);
        }

        for img in self.terrain_texes.iter() {
            img.1.set_filter(Nearest);
        }

        Ok(())
    }

    pub fn get_map_tex(&self, terrain_type: TerrainType, id: usize) -> &Texture2D {
        let file_path = format!(
            "{}{}{}{}",
            MAP_IMG_PATH,
            terrain_type.to_string(),
            id.to_string(),
            ".png"
        );
        self.terrain_texes.get(file_path.as_str()).unwrap()
    }
}
