use crate::{
    components::cell_components::CellColor,
    storage::cell_storage::{Join, ReadCellStorage},
};
use shred::System;

pub struct DrawSystem;

impl<'a> System<'a> for DrawSystem {
    type SystemData = ReadCellStorage<'a, CellColor>;
    fn run(&mut self, cell_colors: Self::SystemData) {
        for ((x, y), cell_color) in (&cell_colors).join() {
            println!(
                "(x: {}, y: {}) (r: {}, g: {}, b: {})",
                x, y, cell_color.r, cell_color.g, cell_color.b
            )
        }
    }
}