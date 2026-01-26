use crate::data::buildings::{BuildingStatus, BuildingType};
use crate::data::elements::Element;
use crate::data::loader::GameData;
use crate::data::grid::Grid;
use crate::data::missions::{MissionOutcome, OngoingMission};
use crate::engine::actions::Action;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

#[derive(Clone, PartialEq)]
enum SectView {
    Map,
    BuildingDetails(u64), // Using ID
}

pub struct SectBaseState {
    view: SectView,
    settings_open: bool,
    feng_shui_overlay_active: bool,
    crafting_modal_open: bool,
    tech_tree_open: bool,
    placement_mode: Option<crate::data::buildings::BuildingType>,
    hovered_tile: Option<(i32, i32)>,
}

impl SectBaseState {
    pub fn new() -> Self {
        Self {
            view: SectView::Map,
            settings_open: false,
            feng_shui_overlay_active: false,
            crafting_modal_open: false,
            tech_tree_open: false,
            placement_mode: None,
            hovered_tile: None,
        }
    }

    /// Update handling immediate mode UI
    pub fn update(
        &mut self, 
        data: &mut GameData, 
        grid: &mut Grid, 
        spirit_stones: u32, 
        herbs: u32, 
        influence: u32, 
        relics: u32, 
        inventory: &std::collections::HashMap<String, u32>, 
        unlocked_techs: &[String], 
        event_log: &[String], 
        ongoing_missions: &[OngoingMission], 
        completed_missions: &[MissionOutcome], 
        completed_history: &[String]
    ) -> UpdateResult {
        
        // --- 1. Global Input & Navigation ---
        if let Some(res) = self.handle_global_input() {
            return res;
        }

        // --- 2. Layout & Setup ---
        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 60.0;
        let left_panel_w = 250.0;
        let right_panel_w = 250.0;
        let center_w = screen_w - left_panel_w - right_panel_w - 20.0;
        
        // --- 3. Draw Header ---
        self.draw_header(screen_w, header_h, spirit_stones, herbs, influence, relics);

        // --- 4. Draw Left Panel (Buildings & Navigation) ---
        if let Some(res) = self.draw_left_panel(header_h, screen_h, left_panel_w, data) {
            return res;
        }

        // --- 5. Draw Center Panel (Main Content) ---
        let center_rect = Rect::new(left_panel_w, header_h, center_w, screen_h - header_h);
        if let Some(res) = self.draw_center_panel(
            center_rect, 
            data, 
            grid, 
            spirit_stones, 
            herbs,
            inventory, 
            unlocked_techs, 
            ongoing_missions, 
            completed_missions, 
            completed_history
        ) {
            return res;
        }

        // --- 6. Draw Right Panel (Event Log) ---
        self.draw_right_panel(header_h, screen_h, left_panel_w, center_w, right_panel_w, event_log);

        // --- 7. Modals (Settings, etc) ---
        if self.settings_open {
            if let Some(res) = self.draw_settings_modal(screen_w, screen_h) {
                return res;
            }
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _grid: &Grid, _spirit_stones: u32) {
        // Drawing handled in update for immediate mode
    }

    // --- Helper Functions ---

    fn handle_global_input(&mut self) -> Option<UpdateResult> {
        if is_key_pressed(KeyCode::Escape) {
            if self.settings_open {
                self.settings_open = false;
            } else if self.tech_tree_open {
                self.tech_tree_open = false;
            } else if self.crafting_modal_open {
                // If in details view, might close modal, but if in map view (construction), also close
                self.crafting_modal_open = false;
            } else if self.placement_mode.is_some() {
                self.placement_mode = None;
            } else if let SectView::BuildingDetails(_) = self.view {
                self.view = SectView::Map; // Go back to map
            } else {
                return Some(UpdateResult::new().with_transition(StateTransition::ToMainMenu));
            }
        }
        None
    }

    fn draw_header(&mut self, screen_w: f32, header_h: f32, spirit_stones: u32, herbs: u32, influence: u32, relics: u32) {
        draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
        draw_text("SECT MANAGEMENT", 20.0, 40.0, FONT_TITLE_SIZE, PRIMARY);
        
        let res_text = format!("SS: {}  Herbs: {}  Infl: {}  Relics: {}", spirit_stones, herbs, influence, relics);
        let res_dims = measure_text(&res_text, None, FONT_HEADER_SIZE as u16, 1.0);
        
        draw_text(&res_text, screen_w - res_dims.width - 60.0, 40.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);

        // Cog Button (Settings)
        if draw_button(Rect::new(screen_w - 50.0, 10.0, 40.0, 40.0), "O", false) {
            self.settings_open = !self.settings_open;
        }
    }

    fn draw_left_panel(&mut self, header_h: f32, screen_h: f32, width: f32, data: &GameData) -> Option<UpdateResult> {
        let rect = Rect::new(0.0, header_h, width, screen_h - header_h);
        draw_panel(rect, Some("Buildings"));

        // List constructed buildings
        let mut btn_y = rect.y + 40.0;
        for building in &data.buildings {
            let status_str = match building.status {
                BuildingStatus::Active => "",
                BuildingStatus::Ruined => " (Ruined)",
                BuildingStatus::Constructing => " (Building...)",
            };
            let label = format!("{}{}", building.building_type, status_str);

            if draw_button(Rect::new(rect.x + 10.0, btn_y, width - 20.0, 35.0), &label, false) {
                self.view = SectView::BuildingDetails(building.id);
            }
            btn_y += 40.0;

            // Prevent overflow
            if btn_y > rect.y + rect.h - 200.0 {
                draw_text("...", rect.x + 10.0, btn_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                break;
            }
        }

        // Navigation buttons at bottom
        let nav_y_start = rect.y + rect.h - 170.0;
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start, width - 20.0, 40.0), "Disciples", false) {
             return Some(UpdateResult::new().with_transition(StateTransition::ToDiscipleRoster));
        }
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start + 50.0, width - 20.0, 40.0), "World Map", false) {
             return Some(UpdateResult::new().with_transition(StateTransition::ToWorldMap));
        }
        if draw_button(Rect::new(rect.x + 10.0, nav_y_start + 100.0, width - 20.0, 40.0), "Construction", false) {
             self.view = SectView::Map;
             self.crafting_modal_open = true;
        }
        None
    }

    fn draw_center_panel(
        &mut self, 
        rect: Rect, 
        data: &mut GameData, 
        grid: &mut Grid, 
        spirit_stones: u32, 
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>, 
        unlocked_techs: &[String], 
        ongoing_missions: &[OngoingMission], 
        completed_missions: &[MissionOutcome], 
        completed_history: &[String]
    ) -> Option<UpdateResult> {
        match self.view {
            SectView::Map => self.draw_map_view(rect, data, grid, spirit_stones, unlocked_techs),
            SectView::BuildingDetails(id) => self.draw_building_details(
                rect, 
                id, 
                data, 
                spirit_stones, 
                herbs,
                inventory, 
                unlocked_techs, 
                ongoing_missions, 
                completed_missions, 
                completed_history
            ),
        }
    }

    fn draw_map_view(&mut self, rect: Rect, data: &mut GameData, grid: &mut Grid, _spirit_stones: u32, unlocked_techs: &[String]) -> Option<UpdateResult> {
        let has_geomancy = unlocked_techs.contains(&"geomancy".to_string());

        let header_str = if has_geomancy {
            if self.feng_shui_overlay_active {
                "Sect Map (Feng Shui ON - Press 'F' to toggle)"
            } else {
                "Sect Map (Press 'F' for Feng Shui)"
            }
        } else {
             "Sect Map"
        };
        draw_panel(rect, Some(header_str));

        // Toggle Overlay
        if has_geomancy && is_key_pressed(KeyCode::F) {
            self.feng_shui_overlay_active = !self.feng_shui_overlay_active;
            if self.feng_shui_overlay_active {
                crate::engine::feng_shui::update_feng_shui_map(grid, &mut data.buildings);
            }
        }

        // Ensure overlay is off if tech is locked (e.g. if cheat/reset happened?)
        if !has_geomancy && self.feng_shui_overlay_active {
            self.feng_shui_overlay_active = false;
        }

        // Reset hovered tile for this frame
        self.hovered_tile = None;

        // Draw Map Tiles
        let tile_size = 25.0;
        let map_start_x = rect.x + (rect.w - (grid.width as f32 * tile_size)) / 2.0;
        let map_start_y = rect.y + 50.0;

        self.draw_placement_preview(rect, map_start_x, map_start_y);

        for y in 0..grid.height {
            for x in 0..grid.width {
                let tx = map_start_x + x as f32 * tile_size;
                let ty = map_start_y + y as f32 * tile_size;
                let tile_rect = Rect::new(tx, ty, tile_size, tile_size);

                // Get tile data for element coloring
                if let Some(tile) = grid.get_tile(x, y) {
                    // Base grass color
                    draw_rectangle(tx, ty, tile_size, tile_size, Color::new(0.1, 0.4, 0.1, 1.0));

                    if self.feng_shui_overlay_active {
                        // Overlay: vivid element coloring
                        let strength = tile.element_strength.get(&tile.dominant_element).copied().unwrap_or(1.0);
                        let intensity = (strength / 2.0).clamp(0.3, 0.8);
                        let mut elem_color = tile.dominant_element.color();
                        elem_color.a = intensity;
                        draw_rectangle(tx, ty, tile_size, tile_size, elem_color);
                    } else {
                        // Normal: subtle tint
                        let mut tint = tile.dominant_element.color();
                        tint.a = 0.15;
                        draw_rectangle(tx, ty, tile_size, tile_size, tint);
                    }

                    draw_rectangle_lines(tx, ty, tile_size, tile_size, 1.0, Color::new(0.2, 0.5, 0.2, 0.5));

                    // Track hovered tile for tooltip
                    if tile_rect.contains(mouse_position().into()) && self.placement_mode.is_none() {
                        self.hovered_tile = Some((x, y));
                    }
                }

                // Placement Interaction
                if let Some(res) = self.handle_placement_click(tile_rect, x as i32, y as i32) {
                    return Some(res);
                }
            }
        }

        // Draw Buildings
        for building in data.buildings.iter() {
             let tx = map_start_x + building.x as f32 * tile_size;
             let ty = map_start_y + building.y as f32 * tile_size;
             let b_rect = Rect::new(tx, ty, tile_size, tile_size);

             // Element-based building coloring
             let base_color = if building.status == BuildingStatus::Ruined {
                 Color::new(0.5, 0.5, 0.5, 1.0)
             } else if building.element != Element::None {
                 building.element.color()
             } else {
                 BUTTON_NORMAL
             };

             draw_rectangle(tx + 2.0, ty + 2.0, tile_size - 4.0, tile_size - 4.0, base_color);

             // Feng Shui overlay: show score as colored border
             if self.feng_shui_overlay_active {
                 let border_color = if building.feng_shui_score >= 10.0 {
                     FENG_SHUI_EXCELLENT
                 } else if building.feng_shui_score > 0.0 {
                     FENG_SHUI_POSITIVE
                 } else if building.feng_shui_score < 0.0 {
                     FENG_SHUI_NEGATIVE
                 } else {
                     FENG_SHUI_NEUTRAL
                 };
                 draw_rectangle_lines(tx + 1.0, ty + 1.0, tile_size - 2.0, tile_size - 2.0, 3.0, border_color);

                 // Score number
                 let score_text = format!("{:+.0}", building.feng_shui_score);
                 draw_text(&score_text, tx + 2.0, ty + tile_size - 2.0, 12.0, TEXT_PRIMARY);
             }

             // Interaction: Click to view details
             if self.placement_mode.is_none() && b_rect.contains(mouse_position().into()) {
                 draw_rectangle_lines(tx, ty, tile_size, tile_size, 2.0, Color::new(1.0, 1.0, 0.0, 0.8));
                 if is_mouse_button_pressed(MouseButton::Left) {
                     self.view = SectView::BuildingDetails(building.id);
                 }
             }
        }

        // Draw legend for overlay mode
        if self.feng_shui_overlay_active {
            let legend_y = rect.y + rect.h - 80.0;
            draw_text("Elements:", rect.x + 10.0, legend_y, FONT_SMALL_SIZE, TEXT_PRIMARY);
            let elements = [
                (Element::Metal, "Met"),
                (Element::Wood, "Wod"),
                (Element::Water, "Wat"),
                (Element::Fire, "Fir"),
                (Element::Earth, "Ear"),
            ];
            let mut lx = rect.x + 80.0;
            for (elem, name) in elements {
                draw_rectangle(lx, legend_y - 12.0, 14.0, 14.0, elem.color());
                draw_text(name, lx + 18.0, legend_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                lx += 55.0;
            }
        }

        // Draw tooltip for hovered tile
        if let Some((hx, hy)) = self.hovered_tile {
            if let Some(tile) = grid.get_tile(hx, hy) {
                let building = data.buildings.iter().find(|b| b.x == hx && b.y == hy);

                let mut lines = vec![
                    format!("Tile ({}, {})", hx, hy),
                    format!("Dominant: {:?}", tile.dominant_element),
                ];

                if self.feng_shui_overlay_active {
                    for (elem, str_val) in &tile.element_strength {
                        if *str_val > 0.1 {
                            lines.push(format!("  {:?}: {:.1}", elem, str_val));
                        }
                    }
                }

                if let Some(b) = building {
                    lines.push(format!("{} (Lv{})", b.building_type, b.level));
                    lines.push(format!("Feng Shui: {:+.0}", b.feng_shui_score));
                }

                // Draw tooltip near mouse
                let (mx, my) = mouse_position();
                draw_tooltip_box(mx + 15.0, my + 15.0, &lines);
            }
        }

        // Construction Modal (reusing crafting_modal_open)
        if self.crafting_modal_open {
             return self.draw_construction_modal(data, unlocked_techs);
        }

        None
    }

    fn draw_placement_preview(&mut self, rect: Rect, _map_x: f32, _map_y: f32) {
        if let Some(place_type) = &self.placement_mode {
             draw_text(&format!("Placing: {:?} (Click to Build, RMB/Esc to Cancel)", place_type), rect.x + 20.0, rect.y + rect.h - 40.0, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
             
             if is_mouse_button_pressed(MouseButton::Right) {
                 self.placement_mode = None;
             }
        }
    }

    fn handle_placement_click(&mut self, tile_rect: Rect, x: i32, y: i32) -> Option<UpdateResult> {
        // Fix E0506: Clone place_type before potential mutation of self.placement_mode
        let place_type = if let Some(pt) = &self.placement_mode {
            pt.clone()
        } else {
            return None;
        };

        if tile_rect.contains(mouse_position().into()) {
            // Draw highlight via helper or here? self is borrowed mut so drawing is tricky if helper needs struct state?
            // Actually macroquad draw calls don't need self usually, but here we are inside a loop in run_loop context
            // Just draw:
            draw_rectangle(tile_rect.x, tile_rect.y, tile_rect.w, tile_rect.h, Color::new(0.0, 1.0, 0.0, 0.5));
            
            if is_mouse_button_pressed(MouseButton::Left) {
                self.placement_mode = None;
                return Some(UpdateResult::new().with_action(Action::ConstructBuilding(place_type, x, y)));
            }
        }
        None
    }

    fn draw_building_details(
        &mut self, 
        rect: Rect, 
        id: u64, 
        data: &mut GameData, 
        spirit_stones: u32,
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>,
        unlocked_techs: &[String],
        ongoing_missions: &[OngoingMission], 
        completed_missions: &[MissionOutcome], 
        completed_history: &[String]
    ) -> Option<UpdateResult> {
        
        let building = data.buildings.iter().find(|b| b.id == id)?; // If not found, do nothing
        let b_type = &building.building_type;
        
        draw_text(&format!("{:?}", b_type), rect.x + 20.0, rect.y + 60.0, FONT_HEADER_SIZE, PRIMARY);
        let d_y = rect.y + 100.0;
        
        draw_text(&format!("Level: {}", building.level), rect.x + 20.0, d_y, FONT_BODY_SIZE, TEXT_PRIMARY);
        draw_text(&format!("Element: {:?}", building.element), rect.x + 20.0, d_y + 30.0, FONT_BODY_SIZE, TEXT_SECONDARY);
        draw_text(&format!("Feng Shui: {:.1}", building.feng_shui_score), rect.x + 20.0, d_y + 60.0, FONT_BODY_SIZE, 
            if building.feng_shui_score > 0.0 { Color::new(0.2, 0.8, 0.2, 1.0) } 
            else if building.feng_shui_score < 0.0 { Color::new(0.8, 0.2, 0.2, 1.0) } 
            else { TEXT_SECONDARY }
        );

        // Close Button for Details
        if draw_button(Rect::new(rect.x + rect.w - 80.0, rect.y + 10.0, 60.0, 30.0), "Back", false) {
            self.view = SectView::Map;
            return None;
        }

        // Specialized Building Actions
        let action_y = d_y + 100.0;
        
        if building.status == BuildingStatus::Ruined {
            draw_text("(Ruined)", rect.x + 200.0, rect.y + 60.0, FONT_HEADER_SIZE, Color::new(0.8, 0.2, 0.2, 1.0));
            if draw_button(Rect::new(rect.x + 20.0, action_y, 200.0, 40.0), "Repair (50 SS)", false) {
                 return Some(UpdateResult::new().with_action(Action::RepairBuilding(building.id)));
            }
        } else if *b_type == BuildingType::MissionBoard {
            if let Some(res) = self.draw_mission_list(rect, data, ongoing_missions, completed_missions, completed_history, action_y) {
                return Some(res);
            }
        } else {
             // Upgrade Button
             if draw_button(Rect::new(rect.x + 20.0, action_y, 150.0, 40.0), "Upgrade (50 SS)", false) {
                  return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(b_type.clone())));
             }

             // Specific Buttons
             if *b_type == BuildingType::SectHall {
                  if draw_button(Rect::new(rect.x + 180.0, action_y, 150.0, 40.0), "Recruit", false) {
                      return Some(UpdateResult::new().with_action(Action::RecruitDisciple));
                  }
                  if draw_button(Rect::new(rect.x + 340.0, action_y, 150.0, 40.0), "Research / Tech", false) {
                      self.tech_tree_open = true;
                  }
             } else if matches!(b_type, BuildingType::AlchemyFurnace | BuildingType::ArtifactForge) {
                  if draw_button(Rect::new(rect.x + 180.0, action_y, 150.0, 40.0), "Crafting", false) {
                       self.crafting_modal_open = true;
                  }
             }
        }

        // Modals overlaying details
        if self.tech_tree_open {
             if let Some(res) = self.draw_tech_tree_modal(data, unlocked_techs) {
                 return Some(res);
             }
        }

        if self.crafting_modal_open {
             if let Some(res) = self.draw_crafting_modal(data, b_type, spirit_stones, herbs, inventory) {
                 return Some(res);
             }
        }

        None
    }

    fn draw_mission_list(
        &self, 
        rect: Rect, 
        data: &GameData, 
        ongoing_missions: &[OngoingMission], 
        completed_missions: &[MissionOutcome], 
        completed_history: &[String],
        start_y: f32
    ) -> Option<UpdateResult> {
         let mut m_y = start_y;
         for mission in &data.missions {
             let is_ongoing = ongoing_missions.iter().any(|m| m.mission.description == mission.description);
             let is_pending = completed_missions.iter().any(|m| m.description == mission.description);
             let is_historically_complete = completed_history.contains(&mission.description);
             
             let available = if mission.repeatable {
                 !is_ongoing && !is_pending
             } else {
                 !is_ongoing && !is_pending && !is_historically_complete
             };

             if available {
                 if draw_button(Rect::new(rect.x + 20.0, m_y, rect.w - 40.0, 35.0), &format!("Mission: {}", mission.description), false) {
                      return Some(UpdateResult::new().with_transition(StateTransition::ToMissionAssignment(mission.description.clone())));
                 }
                 m_y += 40.0;
             }
         }
         None
    }

    fn draw_construction_modal(&mut self, data: &GameData, unlocked_techs: &[String]) -> Option<UpdateResult> {
         let (screen_w, screen_h) = (screen_width(), screen_height());
         draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

         let modal_w = 400.0;
         let modal_h = 500.0;
         let modal_x = (screen_w - modal_w) / 2.0;
         let modal_y = (screen_h - modal_h) / 2.0;

         draw_panel(Rect::new(modal_x, modal_y, modal_w, modal_h), Some("Construction Blueprints"));

         if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "Close", false) {
             self.crafting_modal_open = false;
         }

         let mut b_y = modal_y + 50.0;

         // Use loaded definitions
         // Sort by cost for consistent display
         let mut build_opts: Vec<_> = data.building_definitions.values().collect();
         build_opts.sort_by_key(|a| a.cost);

         for def in build_opts {
             let req_tech = def.tech_required.clone().unwrap_or_default();
             let tech_unlocked = unlocked_techs.contains(&req_tech) || req_tech.is_empty();

             // Skip unique buildings that already exist
             let already_built = def.unique && data.buildings.iter().any(|b| b.building_type == def.building_type);

             if tech_unlocked && !already_built {
                 if draw_button(Rect::new(modal_x + 20.0, b_y, modal_w - 40.0, 40.0), &format!("{} ({} SS)", def.name, def.cost), false) {
                     self.crafting_modal_open = false;
                     self.placement_mode = Some(def.building_type.clone());
                 }
                 b_y += 50.0;
             }
         }
         None
    }

    fn draw_tech_tree_modal(&mut self, data: &GameData, unlocked_techs: &[String]) -> Option<UpdateResult> {
        let (screen_w, screen_h) = (screen_width(), screen_height());
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        
        let modal_w = 600.0;
        let modal_h = 600.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;
        
        draw_panel(Rect::new(modal_x, modal_y, modal_w, modal_h), Some("Sect Knowledge Tree"));
        
        if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "Close", false) {
            self.tech_tree_open = false;
        }
        
        let mut t_y = modal_y + 50.0;
        for tech in data.techs.values() {
            let unlocked = unlocked_techs.contains(&tech.id);
            let can_unlock = !unlocked && tech.prerequisites.iter().all(|p| unlocked_techs.contains(p));
            
            let color = if unlocked { Color::new(0.5, 1.0, 0.5, 1.0) } else if can_unlock { Color::new(1.0, 1.0, 0.5, 1.0) } else { Color::new(0.5, 0.5, 0.5, 1.0) };
            
            draw_text(&tech.name, modal_x + 20.0, t_y, FONT_HEADER_SIZE, color);
            draw_text(&format!("Cost: {} SS", tech.cost_spirit_stones), modal_x + 300.0, t_y, FONT_BODY_SIZE, TEXT_SECONDARY);
            t_y += 25.0;
            draw_text(&tech.description, modal_x + 20.0, t_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
            t_y += 25.0;
            
            if can_unlock {
                if draw_button(Rect::new(modal_x + 450.0, t_y - 40.0, 120.0, 30.0), "Research", false) {
                     return Some(UpdateResult::new().with_action(Action::ResearchTech(tech.id.clone())));
                }
            } else if unlocked {
                draw_text("(Learned)", modal_x + 450.0, t_y - 35.0, FONT_SMALL_SIZE, TEXT_HIGHLIGHT);
            } else {
                draw_text("Locked", modal_x + 450.0, t_y - 35.0, FONT_SMALL_SIZE, TEXT_SECONDARY);
            }
            
            t_y += 20.0;
            draw_line(modal_x + 10.0, t_y, modal_x + modal_w - 10.0, t_y, 1.0, SECONDARY);
            t_y += 10.0;
        }
        None
    }

    fn draw_crafting_modal(&mut self, data: &GameData, b_type: &BuildingType, spirit_stones: u32, herbs: u32, inventory: &std::collections::HashMap<String, u32>) -> Option<UpdateResult> {
        let (screen_w, screen_h) = (screen_width(), screen_height());
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        
        let modal_w = 500.0;
        let modal_h = 600.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;
        
        draw_panel(Rect::new(modal_x, modal_y, modal_w, modal_h), Some("Crafting Menu"));
        
        if draw_button(Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0), "Close", false) {
            self.crafting_modal_open = false;
        }
        
        let mut r_y = modal_y + 50.0;
        let recipes: Vec<_> = data.recipes.iter().filter(|r| r.required_building == *b_type).collect();
        
        if recipes.is_empty() {
            draw_text("No recipes available.", modal_x + 20.0, r_y, FONT_BODY_SIZE, TEXT_SECONDARY);
        } else {
            for recipe in recipes {
                let mut can_craft = true;
                let mut ing_text = String::new();
                for (ing, amt) in &recipe.ingredients {
                    let has = match ing.as_str() {
                        "spirit_stones" => spirit_stones,
                        "herbs" => herbs,
                        _ => *inventory.get(ing).unwrap_or(&0),
                    };
                    
                    if has < *amt {
                        can_craft = false;
                    }
                    
                    if !ing_text.is_empty() { ing_text.push_str(", "); }
                    ing_text.push_str(&format!("{}x {} ({})", amt, ing, has));
                }
                
                // Dim button if not craftable
                // Actually, draw_button doesn't support disabled state easily without custom check
                // We'll just append (LOCKED) or change text color if we could, 
                // but for now, rely on logic execution check or user visual feedback?
                let label = if can_craft {
                    format!("{} ({})", recipe.name, ing_text)
                } else {
                    format!("{} (Missing: {})", recipe.name, ing_text)
                };

                // Visual hint: We can't change button color easily with this API wrapper unless we modify it, 
                // but we can just not respond to click if !can_craft? Or let the action fail?
                // Letting action fail is safe. But Good UI should show.
                // We will assume standard button color for now.
                
                if draw_button(Rect::new(modal_x + 20.0, r_y, modal_w - 40.0, 50.0), &label, false) {
                     if can_craft {
                         return Some(UpdateResult::new().with_action(Action::CraftItem(recipe.id.clone())));
                     }
                }
                r_y += 60.0;
            }
        }
        None
    }

    fn draw_right_panel(&self, header_h: f32, screen_h: f32, left_w: f32, center_w: f32, width: f32, event_log: &[String]) {
        let rect = Rect::new(left_w + center_w, header_h, width, screen_h - header_h);
        draw_panel(rect, Some("Event Log"));
        
        let mut log_y = rect.y + 50.0;
        let max_width = width - 20.0;
        
        for event in event_log.iter().rev().take(20) {
            let words: Vec<&str> = event.split_whitespace().collect();
            let mut current_line = String::new();
            
            for word in words {
                let test_line = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };
                
                if test_line.len() as f32 * 7.0 > max_width { 
                     draw_text(&current_line, rect.x + 10.0, log_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                     log_y += 18.0;
                     current_line = word.to_string();
                } else {
                     current_line = test_line;
                }
            }
            if !current_line.is_empty() {
                draw_text(&current_line, rect.x + 10.0, log_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                log_y += 18.0;
            }
            log_y += 4.0;
            if log_y > screen_h - 20.0 { break; }
        }
    }

    fn draw_settings_modal(&mut self, screen_w: f32, screen_h: f32) -> Option<UpdateResult> {
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));
        
        let modal_w = 300.0;
        let modal_h = 250.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;
        let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);
        
        draw_panel(modal_rect, Some("Settings"));
        
        if draw_button(Rect::new(modal_x + 50.0, modal_y + 60.0, 200.0, 40.0), "Save Game", false) {
             return Some(UpdateResult::new().with_action(Action::SaveGame));
        }

        if draw_button(Rect::new(modal_x + 50.0, modal_y + 120.0, 200.0, 40.0), "Exit to Menu", false) {
             return Some(UpdateResult::new().with_transition(StateTransition::ToMainMenu));
        }
        
        if draw_button(Rect::new(modal_x + 50.0, modal_y + 180.0, 200.0, 40.0), "Close", false) {
            self.settings_open = false;
        }
        None
    }
}