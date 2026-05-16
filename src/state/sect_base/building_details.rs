use super::*;

impl SectBaseState {
    pub(super) fn draw_building_details(
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
        completed_history: &[String],
        disciples: &[Disciple],
        current_season: &Season,
        discovered_recipes: &[String],
    ) -> Option<UpdateResult> {
        let building = data.buildings.iter().find(|b| b.id == id)?.clone();
        let b_type = building.building_type.clone();

        self.draw_building_header(rect, &building, disciples);

        if draw_button(Rect::new(rect.x + rect.w - 80.0, rect.y + 10.0, 60.0, 30.0), "Back", false) {
            self.view = SectView::Map;
            self.herb_planting_modal = None;
            self.disciple_assignment_modal = false;
            self.infusion_modal_open = false;
            return None;
        }

        let action_y = self.calculate_action_y(rect, &building, disciples);

        if let Some(res) = self.dispatch_building_type(
            rect, &building, &b_type, data, spirit_stones, herbs, inventory,
            unlocked_techs, ongoing_missions, completed_missions, completed_history,
            disciples, current_season, discovered_recipes, action_y,
        ) {
            return Some(res);
        }

        self.draw_building_modals(data, &b_type, spirit_stones, herbs, inventory, unlocked_techs, disciples, discovered_recipes)
    }

    fn draw_building_header(&self, rect: Rect, building: &crate::data::buildings::Building, disciples: &[Disciple]) {
        let b_type = &building.building_type;
        draw_text(&format!("{}", b_type), rect.x + 20.0, rect.y + 60.0, FONT_HEADER_SIZE, PRIMARY);
        let d_y = rect.y + 100.0;
        draw_text(&format!("Level: {}", building.level), rect.x + 20.0, d_y, FONT_BODY_SIZE, TEXT_PRIMARY);
        draw_text(&format!("Element: {:?}", building.element), rect.x + 20.0, d_y + 30.0, FONT_BODY_SIZE, TEXT_SECONDARY);
        let fs_color = if building.feng_shui_score > 0.0 {
            Color::new(0.2, 0.8, 0.2, 1.0)
        } else if building.feng_shui_score < 0.0 {
            Color::new(0.8, 0.2, 0.2, 1.0)
        } else {
            TEXT_SECONDARY
        };
        draw_text(&format!("Feng Shui: {:.1}", building.feng_shui_score), rect.x + 20.0, d_y + 60.0, FONT_BODY_SIZE, fs_color);
    }

    fn calculate_action_y(&self, rect: Rect, building: &crate::data::buildings::Building, disciples: &[Disciple]) -> f32 {
        let d_y = rect.y + 100.0;
        let mut action_y = d_y + 100.0;

        if building.building_type == BuildingType::SectHall {
            let capacity: u32 = self.calculate_population_capacity_from_buildings(disciples, &[]);
            let _ = capacity; // drawn below
            draw_text(
                &format!("Population: {}/{}", disciples.len(), self.get_sect_capacity_display(building)),
                rect.x + 20.0, d_y + 90.0, FONT_BODY_SIZE, TEXT_SECONDARY,
            );
            action_y = d_y + 130.0;
        }

        action_y
    }

    fn get_sect_capacity_display(&self, _building: &crate::data::buildings::Building) -> &'static str {
        // Capacity is calculated externally; just show placeholder
        "?"
    }

    fn calculate_population_capacity_from_buildings(&self, _disciples: &[Disciple], _buildings: &[crate::data::buildings::Building]) -> u32 {
        0 // Placeholder - actual capacity computed elsewhere
    }

    fn dispatch_building_type(
        &mut self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        b_type: &BuildingType,
        data: &mut GameData,
        spirit_stones: u32,
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>,
        unlocked_techs: &[String],
        ongoing_missions: &[OngoingMission],
        completed_missions: &[MissionOutcome],
        completed_history: &[String],
        disciples: &[Disciple],
        current_season: &Season,
        discovered_recipes: &[String],
        action_y: f32,
    ) -> Option<UpdateResult> {
        if building.status == BuildingStatus::Ruined {
            return self.draw_ruined_building(rect, building, b_type, action_y);
        }

        if *b_type == BuildingType::MissionBoard {
            return self.draw_mission_list(rect, data, ongoing_missions, completed_missions, completed_history, action_y);
        }

        if matches!(b_type, BuildingType::HerbGarden | BuildingType::Greenhouse) {
            return self.draw_herb_building_details(rect, building, data, disciples, current_season, action_y);
        }

        if *b_type == BuildingType::DryingPavilion {
            return self.draw_drying_pavilion_details(rect, building, data, inventory, action_y);
        }

        if *b_type == BuildingType::HerbStorage {
            return self.draw_herb_storage_details(rect, building, data, inventory, action_y);
        }

        self.draw_generic_building_actions(rect, b_type, action_y)
    }

    fn draw_ruined_building(&mut self, rect: Rect, building: &crate::data::buildings::Building, b_type: &BuildingType, action_y: f32) -> Option<UpdateResult> {
        draw_text("(Ruined)", rect.x + 200.0, rect.y + 60.0, FONT_HEADER_SIZE, Color::new(0.8, 0.2, 0.2, 1.0));
        let repair_cost = building.repair_cost;
        let repair_label = if *b_type == BuildingType::SectHall {
            format!("Restore ({} SS)", repair_cost)
        } else {
            format!("Repair ({} SS)", repair_cost)
        };
        if draw_button(Rect::new(rect.x + 20.0, action_y, 200.0, 40.0), &repair_label, false) {
            return Some(UpdateResult::new().with_action(Action::RepairBuilding(building.id)));
        }
        None
    }

    fn draw_generic_building_actions(&mut self, rect: Rect, b_type: &BuildingType, action_y: f32) -> Option<UpdateResult> {
        if draw_button(Rect::new(rect.x + 20.0, action_y, 150.0, 40.0), "Upgrade (50 SS)", false) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(b_type.clone())));
        }

        if *b_type == BuildingType::SectHall {
            if draw_button(Rect::new(rect.x + 180.0, action_y, 150.0, 40.0), "Recruit", false) {
                return Some(UpdateResult::new().with_action(Action::RecruitDisciple));
            }
            if draw_button(Rect::new(rect.x + 340.0, action_y, 150.0, 40.0), "Research / Tech", false) {
                self.tech_tree_open = true;
            }
        } else if matches!(b_type, BuildingType::AlchemyFurnace | BuildingType::ArtifactForge | BuildingType::Blacksmith | BuildingType::TalismanScriptorium) {
            if draw_button(Rect::new(rect.x + 180.0, action_y, 150.0, 40.0), "Crafting", false) {
                self.crafting_modal_open = true;
            }
        }

        None
    }

    fn draw_building_modals(
        &mut self,
        data: &GameData,
        b_type: &BuildingType,
        spirit_stones: u32,
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>,
        unlocked_techs: &[String],
        disciples: &[Disciple],
        discovered_recipes: &[String],
    ) -> Option<UpdateResult> {
        if self.tech_tree_open {
            let (screen_w, screen_h) = (screen_width(), screen_height());
            let modal_w = 650.0;
            let modal_x = (screen_w - modal_w) / 2.0;
            let modal_y = (screen_h - 550.0) / 2.0;

            let action = tech::draw_tech_tree_modal(&mut self.tech_tree_state, data, unlocked_techs, spirit_stones);

            if tech::check_tech_tree_close(modal_x, modal_y, modal_w) {
                self.tech_tree_open = false;
            } else if let Some(action) = action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.crafting_modal_open {
            if let Some(res) = self.draw_crafting_modal(data, b_type, spirit_stones, herbs, inventory, disciples, discovered_recipes) {
                return Some(res);
            }
        }

        None
    }

    fn draw_herb_building_details(
        &mut self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        data: &GameData,
        disciples: &[Disciple],
        current_season: &Season,
        action_y: f32,
    ) -> Option<UpdateResult> {
        let b_type = &building.building_type;

        if draw_button(Rect::new(rect.x + 20.0, action_y, 140.0, 40.0), "Upgrade (50 SS)", false) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(b_type.clone())));
        }

        if draw_button(Rect::new(rect.x + 170.0, action_y, 140.0, 40.0), "Assign Worker", false) {
            self.disciple_assignment_modal = true;
        }

        if *b_type == BuildingType::Greenhouse {
            if draw_button(Rect::new(rect.x + 320.0, action_y, 140.0, 40.0), "Set Infusion", false) {
                self.infusion_modal_open = true;
            }
        }

        let panel_y = action_y + 60.0;
        if *b_type == BuildingType::Greenhouse {
            herbs::draw_greenhouse_panel(building, data, disciples, current_season, rect.x + 20.0, panel_y, rect.w - 40.0);
        } else {
            herbs::draw_herb_garden_panel(building, data, disciples, current_season, rect.x + 20.0, panel_y, rect.w - 40.0);
        }

        let mut plot_btn_y = panel_y + 30.0;
        for (i, plot) in building.herb_plots.iter().enumerate() {
            if i >= building.get_max_herb_plots() { break; }
            if plot.growing.is_none() {
                if draw_button(Rect::new(rect.x + rect.w - 120.0, plot_btn_y + 10.0, 80.0, 30.0), "Plant", false) {
                    self.herb_planting_modal = Some(i);
                }
            }
            plot_btn_y += 65.0;
        }

        self.draw_herb_modals(building, data, disciples, current_season)
    }

    fn draw_herb_modals(
        &mut self,
        building: &crate::data::buildings::Building,
        data: &GameData,
        disciples: &[Disciple],
        current_season: &Season,
    ) -> Option<UpdateResult> {
        if let Some(plot_idx) = self.herb_planting_modal {
            let result = herbs::draw_herb_planting_modal(building, data, current_season, plot_idx);
            if result.close_modal { self.herb_planting_modal = None; }
            if let Some(action) = result.action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.disciple_assignment_modal {
            let result = herbs::draw_disciple_assignment_modal(building, disciples, &data.buildings);
            if result.close_modal { self.disciple_assignment_modal = false; }
            if let Some(action) = result.action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.infusion_modal_open {
            let result = herbs::draw_infusion_modal(building);
            if result.close_modal { self.infusion_modal_open = false; }
            if let Some(action) = result.action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        None
    }

    fn draw_drying_pavilion_details(
        &mut self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        data: &GameData,
        inventory: &std::collections::HashMap<String, u32>,
        action_y: f32,
    ) -> Option<UpdateResult> {
        if draw_button(Rect::new(rect.x + 20.0, action_y, 150.0, 40.0), "Upgrade (50 SS)", false) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(building.building_type.clone())));
        }

        let panel_y = action_y + 60.0;
        if let Some(action) = herbs::draw_drying_pavilion_panel(building, data, inventory, rect.x + 20.0, panel_y, rect.w - 40.0) {
            return Some(UpdateResult::new().with_action(action));
        }

        None
    }

    fn draw_herb_storage_details(
        &mut self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        data: &GameData,
        inventory: &std::collections::HashMap<String, u32>,
        action_y: f32,
    ) -> Option<UpdateResult> {
        if draw_button(Rect::new(rect.x + 20.0, action_y, 150.0, 40.0), "Upgrade (50 SS)", false) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(building.building_type.clone())));
        }

        let panel_y = action_y + 60.0;
        herbs::draw_herb_storage_panel(building, data, inventory, rect.x + 20.0, panel_y, rect.w - 40.0);

        None
    }

    pub(super) fn draw_mission_list(
        &mut self,
        rect: Rect,
        data: &GameData,
        ongoing_missions: &[OngoingMission],
        completed_missions: &[MissionOutcome],
        completed_history: &[String],
        start_y: f32,
    ) -> Option<UpdateResult> {
        let mut m_y = start_y;
        let mut selected_available = false;
        let selected_desc = self.selected_mission.clone();
        let mouse = vec2(mouse_position().0, mouse_position().1);

        for mission in &data.missions {
            let is_ongoing = ongoing_missions.iter().any(|m| m.mission.description == mission.description);
            let is_pending = completed_missions.iter().any(|m| m.description == mission.description);
            let is_historically_complete = completed_history.contains(&mission.description);

            let available = if mission.repeatable {
                !is_ongoing && !is_pending
            } else {
                !is_ongoing && !is_pending && !is_historically_complete
            };

            if !available { continue; }

            let is_selected = selected_desc.as_deref() == Some(mission.description.as_str());
            let raw_label = mission.description.clone();
            let label = if raw_label.len() > 60 {
                format!("{}...", &raw_label[..57])
            } else {
                raw_label.clone()
            };
            let btn_rect = Rect::new(rect.x + 20.0, m_y, rect.w - 40.0, 42.0);

            if is_selected {
                if draw_button(btn_rect, &label, true) {
                    self.selected_mission = Some(mission.description.clone());
                }
                selected_available = true;
            } else if draw_button_muted(btn_rect, &label, false) {
                self.selected_mission = Some(mission.description.clone());
            }

            if btn_rect.contains(mouse.into()) && raw_label.len() > label.len() {
                draw_tooltip(mouse, &raw_label);
            }

            m_y += 50.0;
        }

        if let Some(selected) = &self.selected_mission {
            if selected_available {
                let btn_rect = Rect::new(rect.x + 20.0, rect.y + rect.h - 60.0, rect.w - 40.0, 40.0);
                if draw_button(btn_rect, "Assign selected mission", false) {
                    return Some(UpdateResult::new().with_transition(StateTransition::ToMissionAssignment(selected.clone())));
                }
            } else {
                draw_text("Selected mission unavailable.", rect.x + 20.0, rect.y + rect.h - 30.0, FONT_SMALL_SIZE, TEXT_SECONDARY);
            }
        }
        None
    }
}
