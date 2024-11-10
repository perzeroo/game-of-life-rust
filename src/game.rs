use macroquad::prelude::*;
use crate::cells;
use crate::renderer;
use crate::VIRTUAL_WIDTH;
use crate::VIRTUAL_HEIGHT;
use crate::CELLS_HEIGHT;
use crate::CELLS_WIDTH;


pub struct Game {
    pub renderer: renderer::Renderer,
    pub cells: cells::Cells,
}

impl Game {
    pub fn new() -> Self {
        Game {
            renderer: renderer::Renderer::new(),
            cells: cells::Cells::new(CELLS_WIDTH, CELLS_HEIGHT),
        }
    }

    pub fn setup(&mut self) {
        self.renderer.setup();
        self.cells.randomize();
    }

    pub fn handle_input(&mut self) {
        let (_, mouse_wheel_y) = mouse_wheel();
        let m_wheel_y_d = mouse_wheel_y / 1200.0 * (1.0 * self.renderer.zoom * self.renderer.zoom);
        self.renderer.zoom += m_wheel_y_d;
        self.renderer.zoom = self.renderer.zoom.clamp(0.5, 7.5);
        if is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let mut mp = mouse_delta_position();
            mp.x *= VIRTUAL_WIDTH/2.;
            mp.y *= VIRTUAL_HEIGHT/2.;
            self.renderer.position -= mp / self.renderer.zoom;
        }

        if is_key_pressed(KeyCode::R) {
            self.cells.randomize();
        }
    }

    pub fn tick(&mut self) {
        self.cells.tick();
        self.renderer.draw_cells(&self.cells);
    }
}
