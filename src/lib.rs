mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    iteration: u32,
    cells: Vec<u32>,
    modified_cells: Vec<usize>
}

#[wasm_bindgen]
impl World {
    /// Get index alongsize 1D cells array for a specified row and column.
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Get (row, col) 2D coords from 1D cell array index.
    fn get_coords(&self, idx: usize) -> [u32; 2] {
        [
            (idx as u32) / (self.width-1),
            (idx as u32) % self.width
        ]
    }

    /// Get the 1D indexes of the Von Neumann neighborhood of the cell at a
    /// given 1D index.
    fn get_von_neumann_indexes(&self, idx: usize) -> [usize; 4] {
        let w = self.width  as usize;
        let h = self.height as usize;
        [
            // Indexing behaves as if the world was torus-shaped
            if idx % w != 0   { idx - 1 } else { idx + (w-1) },
            if idx % w != w-1 { idx + 1 } else { idx - (w-1) },
            if idx / w != 0   { idx - w } else { idx + (h-1)*w },
            if idx / w != h-1 { idx + w } else { idx - (h-1)*w },
        ]
    }

    /// Ticks the world. Return `true` if the state of the world has changed
    /// after ticking, `false` otherwise.
    fn tick(&mut self) -> bool {
        let mut cells = self.cells.clone();
        let mut modified_cells: Vec<usize> = Vec::new();

        // for i in 0..self.cells.len() {
        for &i in self.modified_cells.iter() {
            let cell = self.cells[i];

            if cell / 4 > 0 {
                // Pop four grains of sand on current pile and add its index to
                // modified cells set
                cells[i] -= 4;
                modified_cells.push(i);
                // Add one grain of sand to each neighboor pile and add their
                // indexes to modified cells set
                for &j in self.get_von_neumann_indexes(i).iter() {
                    cells[j] += 1;
                    modified_cells.push(j);
                }
            }
        }

        if modified_cells.len() > 0 {
            modified_cells.sort();
            modified_cells.dedup();
            
            self.cells = cells;
            self.modified_cells = modified_cells;
            self.iteration += 1;
            return true
        }
        false
    }

    /// Ticks the world `steps` times or until it is in a stable state. Return
    /// `true` if all computed states were unstable, or `false` if a stable
    /// state were encoutered.
    pub fn compute_steps(&mut self, steps: u32) -> bool {
        (0..steps).all(|_| self.tick())
    }

    /// Create a new world of given width and height, full of zeroes
    pub fn new(width: u32, height: u32) -> World {
        utils::set_panic_hook(); // Enable nicer debug messages

        let cells = vec![0; (width * height) as usize];

        let mut world = World {
            width,
            height,
            iteration: 0,
            cells,
            modified_cells: (0..(width*height) as usize).collect()
        };

        let center = world.get_index(height/2, width/2);
        world.cells[center] = 10000000;

        // center = world.get_index(height/2+1, width/2+1);
        // world.cells[center] = 10000000;

        world
    }

    pub fn get_cell(&self, row: u32, col: u32) -> u32 {
        self.cells[self.get_index(row, col)]
    }

    pub fn set_cell(&mut self, row: u32, col: u32, val: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = val;
    }

    pub fn get_iteration(&self) -> u32 {
        self.iteration
    }

    pub fn get_cells_ptr(&self) -> *const u32 {
        self.cells.as_ptr()
    }
}

impl fmt::Display for World {
    /// Implements `to_string()` to allow easier display
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                write!(formatter, "{}", cell.to_string())?;
            }
            write!(formatter, "\n")?;
        }
        Ok(())
    }
}
