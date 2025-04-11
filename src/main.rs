// #![windows_subsystem = "windows"]

mod entity;
mod image;
mod main_game;
mod map;
mod settings;

use macroquad::prelude::*;

use main_game::Game;
use settings::*;

use std::ptr;
use winapi::shared::windef::HWND;
use winapi::um::imm::HIMC;
use winapi::um::winuser::GetForegroundWindow;

#[link(name = "imm32")]
extern "system" {
    fn ImmAssociateContext(hwnd: HWND, h_imc: HIMC) -> HIMC;
}

fn window_conf() -> Conf {
    Conf {
        window_title: GAME_TITLE.to_string(),
        fullscreen: true,
        high_dpi: true,

        ..Default::default()
    }
}

fn disable_input_method() {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return;
        }
        ImmAssociateContext(hwnd, ptr::null_mut());
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    disable_input_method();
    let mut game = Game::new();

    game.run().await;
}
