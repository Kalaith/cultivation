use crate::data::buildings::BuildingType;
use crate::data::loader::GameData;
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use macroquad::prelude::*;
use std::collections::HashMap;

const SLOT_SIZE: Vec2 = vec2(200.0, 80.0);
const SLOT_PADDING: f32 = 20.0;

pub struct SectBaseState {
    slots: HashMap<BuildingType, Rect>,
    selected_building: Option<BuildingType>,
    upgrade_button_rect: Rect,
    recruit_button_rect: Rect,
}

impl SectBaseState {
    pub fn new() -> Self {
        let mut slots = HashMap::new();
        let building_types = [
            BuildingType::SectHall,
            BuildingType::TrainingYard,
            BuildingType::LibraryPavilion,
            BuildingType::MissionBoard,
            BuildingType::SpiritGarden,
        ];

        let mut y = 100.0;
        for building_type in building_types {
            let x = 50.0; // Align to the left
            slots.insert(building_type, Rect::new(x, y, SLOT_SIZE.x, SLOT_SIZE.y));
            y += SLOT_SIZE.y + SLOT_PADDING;
        }

        Self {
            slots,
            selected_building: None,
            upgrade_button_rect: Rect::new(320.0, 400.0, 150.0, 40.0),
            recruit_button_rect: Rect::new(320.0, 350.0, 150.0, 40.0),
        }
    }

    pub fn update(&mut self) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToMainMenu);
        }
        if is_key_pressed(KeyCode::D) {
            return UpdateResult::new().with_transition(StateTransition::ToDiscipleRoster);
        }
        if is_key_pressed(KeyCode::M) {
            return UpdateResult::new().with_transition(StateTransition::ToWorldMap);
        }

        let mouse_pos = mouse_position().into();
        for (building_type, rect) in &self.slots {
            if rect.contains(mouse_pos) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_building = Some(building_type.clone());
                return UpdateResult::new();
            }
        }

        if let Some(selected) = &self.selected_building {
            if self.upgrade_button_rect.contains(mouse_pos) && is_mouse_button_pressed(MouseButton::Left) {
                return UpdateResult::new().with_action(Action::UpgradeBuilding(selected.clone()));
            }

            if *selected == BuildingType::SectHall
                && self.recruit_button_rect.contains(mouse_pos)
                && is_mouse_button_pressed(MouseButton::Left)
            {
                return UpdateResult::new().with_action(Action::RecruitDisciple);
            }
        }

        UpdateResult::new()
    }

    pub fn draw(&self, data: &GameData, spirit_stones: u32) {
        // Draw Header
        draw_text("SECT BASE", 20.0, 40.0, 40.0, WHITE);
        draw_text(
            "Press ESC for Main Menu | Press D for Disciples | Press M for Map",
            20.0,
            70.0,
            20.0,
            GRAY,
        );

        // Draw Resources
        let stone_text = format!("Spirit Stones: {}", spirit_stones);
        draw_text(&stone_text, screen_width() - 250.0, 40.0, 24.0, GOLD);

        self.draw_building_slots(data);
        self.draw_details_panel(data);
    }
}

impl SectBaseState {
    fn draw_building_slots(&self, data: &GameData) {
        let mouse_pos = mouse_position().into();
        for (building_type, rect) in &self.slots {
            let color = if rect.contains(mouse_pos) {
                LIGHTGRAY
            } else {
                DARKGRAY
            };
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);

            if let Some(building) = data.buildings.get(building_type) {
                let name = format!("{:?}", building.building_type);
                let level = format!("Lvl: {}", building.level);
                draw_text(&name, rect.x + 10.0, rect.y + 30.0, 24.0, WHITE);
                draw_text(&level, rect.x + 10.0, rect.y + 60.0, 20.0, LIGHTGRAY);
            } else {
                draw_text("[Empty Slot]", rect.x + 10.0, rect.y + 40.0, 20.0, GRAY);
            }

            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, GRAY);
        }
    }

//...
    fn draw_details_panel(&self, data: &GameData) {
        let panel_rect = Rect::new(300.0, 100.0, 400.0, 500.0);
//...
        if let Some(selected_type) = &self.selected_building {
            // --- Mission Board View ---
            if *selected_type == BuildingType::MissionBoard {
                draw_text("Available Missions", panel_rect.x + 20.0, panel_rect.y + 40.0, 30.0, WHITE);
                let mut y_offset = 80.0;
                for mission in &data.missions {
                    draw_text(&mission.description, panel_rect.x + 20.0, panel_rect.y + y_offset, 20.0, WHITE);
                    let danger_text = format!("Danger: {}", mission.danger_level);
                    draw_text(&danger_text, panel_rect.x + 20.0, panel_rect.y + y_offset + 25.0, 18.0, GRAY);
                    y_offset += 60.0;
                }
                return; // End early for this specific view
            }

            // --- Default Building View ---
            if let Some(building) = data.buildings.get(selected_type) {
//...
                let name = format!("{:?}", building.building_type);
                let level = format!("Level {}", building.level);
                let description = "A description of the building and its purpose would go here. It provides bonuses to the sect.";

                draw_text(
                    &name,
                    panel_rect.x + 20.0,
                    panel_rect.y + 40.0,
                    30.0,
                    WHITE,
                );
                draw_text(
                    &level,
                    panel_rect.x + 20.0,
                    panel_rect.y + 70.0,
                    24.0,
                    LIGHTGRAY,
                );

                // --- Upgrade Button ---
                let btn_rect = self.upgrade_button_rect;
                let mouse_pos = mouse_position().into();
                let btn_color = if btn_rect.contains(mouse_pos) { LIME } else { GREEN };
                draw_rectangle(btn_rect.x, btn_rect.y, btn_rect.w, btn_rect.h, btn_color);
                draw_text(
                    "Upgrade (50)",
                    btn_rect.x + 10.0,
                    btn_rect.y + 28.0,
                    24.0,
                    BLACK,
                );

                // --- Recruit Button (SectHall only) ---
                if building.building_type == BuildingType::SectHall {
                    let recruit_btn_rect = self.recruit_button_rect;
                    let recruit_btn_color = if recruit_btn_rect.contains(mouse_pos) {
                        LIGHTGRAY
                    } else {
                        GRAY
                    };
                    draw_rectangle(
                        recruit_btn_rect.x,
                        recruit_btn_rect.y,
                        recruit_btn_rect.w,
                        recruit_btn_rect.h,
                        recruit_btn_color,
                    );
                    draw_text(
                        "Recruit (Free)",
                        recruit_btn_rect.x + 10.0,
                        recruit_btn_rect.y + 28.0,
                        24.0,
                        BLACK,
                    );
                }
            } else {
                draw_text(
                    "Empty Slot",
                    panel_rect.x + 20.0,
                    panel_rect.y + 40.0,
                    30.0,
                    GRAY,
                );
            }
        } else {
            draw_text(
                "Select a building to see details",
                panel_rect.x + 20.0,
                panel_rect.y + 40.0,
                24.0,
                GRAY,
            );
        }
    }
}