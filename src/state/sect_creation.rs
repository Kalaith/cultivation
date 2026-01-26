use crate::data::loader::GameData;
use crate::engine::actions::Action;
use crate::state::UpdateResult;
use crate::ui::theme::*;
use macroquad::prelude::*;

pub struct SectCreationState {
    pub input_buffer: String,
}

impl SectCreationState {
    pub fn new() -> Self {
        Self {
            input_buffer: "Test".to_string(),
        }
    }

    pub fn update(&mut self) -> UpdateResult {
        // Simple text input
        // Macroquad doesn't have a great built-in text input event for chars yet without some work, 
        // using simulated input for now or `get_char_pressed()`.
        
        while let Some(c) = get_char_pressed() {
            if c.is_alphanumeric() || c == ' ' {
                if self.input_buffer.len() < 20 {
                    self.input_buffer.push(c);
                }
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.input_buffer.pop();
        }

        if is_key_pressed(KeyCode::Enter) {
            if !self.input_buffer.is_empty() {
                return UpdateResult::new().with_action(Action::StartNewGame(self.input_buffer.clone()));
            }
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData) {
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.1, 0.1, 0.1, 1.0));

        let title_text = "FOUND A NEW SECT";
        let title_dims = measure_text(title_text, None, FONT_TITLE_SIZE as u16, 1.0);
        draw_text(title_text, (screen_width() - title_dims.width) / 2.0, 100.0, FONT_TITLE_SIZE, TEXT_HIGHLIGHT);

        draw_text("Enter Sect Name:", screen_width() / 2.0 - 100.0, 200.0, FONT_HEADER_SIZE, TEXT_PRIMARY);
        
        // Input Box
        let box_x = screen_width() / 2.0 - 150.0;
        let box_y = 230.0;
        draw_rectangle(box_x, box_y, 300.0, 40.0, BUTTON_NORMAL);
        draw_text(&self.input_buffer, box_x + 10.0, box_y + 30.0, FONT_BODY_SIZE, TEXT_HIGHLIGHT);
        
        // Blink cursor
        if (get_time() * 2.0) as i32 % 2 == 0 {
            let text_dims = measure_text(&self.input_buffer, None, FONT_BODY_SIZE as u16, 1.0);
            draw_line(box_x + 10.0 + text_dims.width, box_y + 5.0, box_x + 10.0 + text_dims.width, box_y + 35.0, 2.0, TEXT_HIGHLIGHT);
        }

        let instruction = "Press ENTER to Begin";
        let inst_dims = measure_text(instruction, None, FONT_BODY_SIZE as u16, 1.0);
        draw_text(instruction, (screen_width() - inst_dims.width) / 2.0, 350.0, FONT_BODY_SIZE, TEXT_SECONDARY);
    }
}
