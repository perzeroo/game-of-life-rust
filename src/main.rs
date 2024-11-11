mod renderer;
mod game;
mod cells;
mod state;

use macroquad::prelude::*;

pub static VIRTUAL_WIDTH: f32 = 1280.0;
pub static VIRTUAL_HEIGHT: f32 = 720.0;
pub static CELLS_WIDTH: u16 = 1024;
pub static CELLS_HEIGHT: u16 = 1024;

const CROSSHAIR_SIZE: f32 = 16.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "Game of life".to_owned(),
        fullscreen: false,
        window_width: VIRTUAL_WIDTH as i32,
        window_height: VIRTUAL_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game: game::Game = game::Game::new();
    game.setup();

    loop {
        game.handle_input();
        game.tick();
        let w = screen_width() / 2.;
        let h = screen_height() / 2.;
        const COLOR: Color = Color::new(0., 0.5, 1., 0.5);
        draw_line(w - CROSSHAIR_SIZE, h, w + CROSSHAIR_SIZE, h, 2.0, COLOR);
        draw_line(w, h - CROSSHAIR_SIZE, w, h + CROSSHAIR_SIZE, 2.0, COLOR);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 35., 70., BLUE);
        next_frame().await 
    }
}
