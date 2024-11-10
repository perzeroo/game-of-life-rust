use std::{u32, usize};
use macroquad::rand::rand;

#[derive(Clone)]
pub struct Cells {
    pub cells: Vec<u8>,
    pub previous_state_cells: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl Cells {
    pub fn new(width: u16, height: u16) -> Self {
        Cells {
            cells: vec![0; width as usize * height as usize],
            previous_state_cells: vec![0; width as usize * height as usize], 
            width,
            height,
        }
    }

    pub fn randomize(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = if rand() > u32::MAX/2 {0} else {1}
        }
    }

    pub fn tick(&mut self) { 
        let mut new_cells = vec![0; self.width as usize * self.height as usize];
        let mut copy_cells = self.cells.clone();
        self.previous_state_cells = copy_cells.to_vec();
        
        let check_1 = self.cells.len() - self.width as usize;
        let check_2 = self.width as usize;
        for (it, cell) in copy_cells.iter_mut().enumerate() {
            let mut ret_val: u8 = 0;
            // this will crash in debug as it underflows
            // checks the neighbors above
            if it > check_2 {
                for i in 0..3 {
                    let kt = it - self.width as usize - 1 + i;
                    if kt >= self.cells.len() {
                        continue;
                    }
                    ret_val += self.cells[kt];
                }
            }
            // checks neighbors at same level
            if (it - 1) < self.cells.len() {
                ret_val += self.cells[it-1];
            }
            if (it + 1) < self.cells.len() {
                ret_val += self.cells[it+1];
            }
            if ret_val > 3 {
                continue;
            }
            // checks neighbors below
            if it <= check_1 {
                for i in 0..3 {
                    let kt = it + self.width as usize - 1 + i;
                    if kt >= self.cells.len() {
                        continue;
                    }
                    ret_val += self.cells[kt];
                }
            }
            // we can just check for when the cell stays alive
            if *cell == 1 {
                if ret_val == 2 || ret_val == 3 {
                    new_cells[it] = 1;
                }
            } else {
                if ret_val == 3 {
                    new_cells[it] = 1;
                }
            }
        }
        
        self.cells = new_cells;
    }
}
