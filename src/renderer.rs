use macroquad::prelude::*;
use crate::{cells, CELLS_HEIGHT, CELLS_WIDTH, VIRTUAL_HEIGHT, VIRTUAL_WIDTH};

pub struct Renderer {
    pub zoom: f32,
    pub position: Vec2,
    pub texture_vec: Vec<u8>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            zoom: 1.0,
            position: (0.0, 0.0).into(),
            texture_vec: vec![0; CELLS_WIDTH as usize * CELLS_HEIGHT as usize * 4],
        }
    }

    pub fn setup(&mut self) {
    }

    pub fn draw_cells(&mut self, cells: &cells::Cells) {
        // would like to use a shader for this so I can just pass the array but idk how to do that
        // so we have this... incredibly slow (2048^2 pixels at 30fps)
        let color_array = [0, 255];
        for (it, cell) in cells.cells.iter().enumerate() {
            if cells.previous_state_cells[it] == *cell {
                continue;
            }
            let base = it * 4;
            let c = *cell as usize;
            self.texture_vec[base..base+4].copy_from_slice(&[color_array[c], color_array[c], color_array[c], color_array[c]]);
        }

        let texture = Texture2D::from_rgba8(CELLS_WIDTH, CELLS_HEIGHT, &self.texture_vec);
        texture.set_filter(FilterMode::Nearest);

        clear_background(BLACK);

        let h = screen_height() as f32;
        let w = screen_width() as f32;
        let h_c = CELLS_HEIGHT as f32;
        let w_c = CELLS_WIDTH as f32;
        draw_texture_ex(&texture,
            w/2. + (self.position.x * self.zoom), // funny shenanigan
            h/2. + (self.position.y * self.zoom),
            WHITE, DrawTextureParams {
            dest_size: Some(vec2(w_c * self.zoom, h_c * self.zoom)),
            ..Default::default()
        });
    }
}

