use crate::data::loader::GameData;
use crate::state::UpdateResult;

impl LibraryState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self) -> UpdateResult {
        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32) {
        // Draw logic here
    }
}
pub struct LibraryState;