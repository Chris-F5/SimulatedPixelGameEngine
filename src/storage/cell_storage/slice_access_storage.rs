use crate::components::cell_components::CellComponent;
use crate::storage::cell_storage::InnerCellStorage;
use crate::CHUNK_SIZE;

pub struct SliceAccessStorage<T>
where
    T: CellComponent,
{
    pub cells: [T; (CHUNK_SIZE * CHUNK_SIZE) as usize],
}

impl<T> Default for SliceAccessStorage<T>
where
    T: CellComponent,
{
    fn default() -> Self {
        SliceAccessStorage::<T> {
            cells: [Default::default(); (CHUNK_SIZE * CHUNK_SIZE) as usize],
        }
    }
}

impl<T> InnerCellStorage<T> for SliceAccessStorage<T>
where
    T: CellComponent,
{
    fn get_mut(&mut self, id: u32) -> &mut T {
        &mut self.cells[id as usize]
    }
    fn get(&self, id: u32) -> &T {
        &self.cells[id as usize]
    }
    fn insert(&mut self, id: u32, component: T) {
        self.cells[id as usize] = component;
    }
    fn remove(&mut self, id: u32) {
        self.cells[id as usize] = Default::default();
    }
}