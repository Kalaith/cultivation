use super::*;
use crate::data::missions::{Mission, MissionType};
use macroquad_toolkit::ui::draw_ui_text;

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

        if draw_button(
            Rect::new(rect.x + rect.w - 80.0, rect.y + 10.0, 60.0, 30.0),
            "Back",
            false,
        ) {
            self.view = SectView::Map;
            self.herb_planting_modal = None;
            self.disciple_assignment_modal = false;
            self.infusion_modal_open = false;
            return None;
        }

        let action_y = self.calculate_action_y(rect, &building, disciples);

        if let Some(res) = self.dispatch_building_type(
            rect,
            &building,
            &b_type,
            data,
            spirit_stones,
            herbs,
            inventory,
            unlocked_techs,
            ongoing_missions,
            completed_missions,
            completed_history,
            disciples,
            current_season,
            discovered_recipes,
            action_y,
        ) {
            return Some(res);
        }

        self.draw_building_modals(
            data,
            &b_type,
            spirit_stones,
            herbs,
            inventory,
            unlocked_techs,
            disciples,
            discovered_recipes,
        )
    }

    fn draw_building_header(
        &self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        disciples: &[Disciple],
    ) {
        let b_type = &building.building_type;
        draw_ui_text(
            &format!("{}", b_type),
            rect.x + 20.0,
            rect.y + 60.0,
            FONT_HEADER_SIZE,
            PRIMARY,
        );
        let d_y = rect.y + 100.0;
        draw_ui_text(
            &format!("Level: {}", building.level),
            rect.x + 20.0,
            d_y,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );
        draw_ui_text(
            &format!("Element: {:?}", building.element),
            rect.x + 20.0,
            d_y + 30.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
        let fs_color = if building.feng_shui_score > 0.0 {
            Color::new(0.2, 0.8, 0.2, 1.0)
        } else if building.feng_shui_score < 0.0 {
            Color::new(0.8, 0.2, 0.2, 1.0)
        } else {
            TEXT_SECONDARY
        };
        draw_ui_text(
            &format!("Feng Shui: {:.1}", building.feng_shui_score),
            rect.x + 20.0,
            d_y + 60.0,
            FONT_BODY_SIZE,
            fs_color,
        );
    }

    fn calculate_action_y(
        &self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        disciples: &[Disciple],
    ) -> f32 {
        let d_y = rect.y + 100.0;
        let mut action_y = d_y + 100.0;

        if building.building_type == BuildingType::SectHall {
            let capacity: u32 = self.calculate_population_capacity_from_buildings(disciples, &[]);
            let _ = capacity; // drawn below
            draw_ui_text(
                &format!(
                    "Population: {}/{}",
                    disciples.len(),
                    self.get_sect_capacity_display(building)
                ),
                rect.x + 20.0,
                d_y + 90.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            action_y = d_y + 130.0;
        }

        action_y
    }

    fn get_sect_capacity_display(
        &self,
        _building: &crate::data::buildings::Building,
    ) -> &'static str {
        // Capacity is calculated externally; just show placeholder
        "?"
    }

    fn calculate_population_capacity_from_buildings(
        &self,
        _disciples: &[Disciple],
        _buildings: &[crate::data::buildings::Building],
    ) -> u32 {
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
            return self.draw_mission_list(
                rect,
                data,
                ongoing_missions,
                completed_missions,
                completed_history,
                action_y,
            );
        }

        if matches!(b_type, BuildingType::HerbGarden | BuildingType::Greenhouse) {
            return self.draw_herb_building_details(
                rect,
                building,
                data,
                disciples,
                current_season,
                action_y,
            );
        }

        if *b_type == BuildingType::DryingPavilion {
            return self.draw_drying_pavilion_details(rect, building, data, inventory, action_y);
        }

        if *b_type == BuildingType::HerbStorage {
            return self.draw_herb_storage_details(rect, building, data, inventory, action_y);
        }

        self.draw_generic_building_actions(rect, b_type, action_y)
    }

    fn draw_ruined_building(
        &mut self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        b_type: &BuildingType,
        action_y: f32,
    ) -> Option<UpdateResult> {
        draw_ui_text(
            "(Ruined)",
            rect.x + 200.0,
            rect.y + 60.0,
            FONT_HEADER_SIZE,
            Color::new(0.8, 0.2, 0.2, 1.0),
        );
        let repair_cost = building.repair_cost;
        let repair_label = if *b_type == BuildingType::SectHall {
            format!("Restore ({} SS)", repair_cost)
        } else {
            format!("Repair ({} SS)", repair_cost)
        };
        if draw_button(
            Rect::new(rect.x + 20.0, action_y, 200.0, 40.0),
            &repair_label,
            false,
        ) {
            return Some(UpdateResult::new().with_action(Action::RepairBuilding(building.id)));
        }
        None
    }

    fn draw_generic_building_actions(
        &mut self,
        rect: Rect,
        b_type: &BuildingType,
        action_y: f32,
    ) -> Option<UpdateResult> {
        if draw_button(
            Rect::new(rect.x + 20.0, action_y, 150.0, 40.0),
            "Upgrade (50 SS)",
            false,
        ) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(b_type.clone())));
        }

        if *b_type == BuildingType::SectHall {
            if draw_button(
                Rect::new(rect.x + 180.0, action_y, 150.0, 40.0),
                "Recruit",
                false,
            ) {
                return Some(UpdateResult::new().with_action(Action::RecruitDisciple));
            }
            if draw_button(
                Rect::new(rect.x + 340.0, action_y, 150.0, 40.0),
                "Research / Tech",
                false,
            ) {
                self.tech_tree_open = true;
            }
        } else if matches!(
            b_type,
            BuildingType::AlchemyFurnace
                | BuildingType::ArtifactForge
                | BuildingType::Blacksmith
                | BuildingType::TalismanScriptorium
        ) {
            if draw_button(
                Rect::new(rect.x + 180.0, action_y, 150.0, 40.0),
                "Crafting",
                false,
            ) {
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

            let action = tech::draw_tech_tree_modal(
                &mut self.tech_tree_state,
                data,
                unlocked_techs,
                spirit_stones,
            );

            if tech::check_tech_tree_close(modal_x, modal_y, modal_w) {
                self.tech_tree_open = false;
            } else if let Some(action) = action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.crafting_modal_open {
            if let Some(res) = self.draw_crafting_modal(
                data,
                b_type,
                spirit_stones,
                herbs,
                inventory,
                disciples,
                discovered_recipes,
            ) {
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

        if draw_button(
            Rect::new(rect.x + 20.0, action_y, 140.0, 40.0),
            "Upgrade (50 SS)",
            false,
        ) {
            return Some(UpdateResult::new().with_action(Action::UpgradeBuilding(b_type.clone())));
        }

        if draw_button(
            Rect::new(rect.x + 170.0, action_y, 140.0, 40.0),
            "Assign Worker",
            false,
        ) {
            self.disciple_assignment_modal = true;
        }

        if *b_type == BuildingType::Greenhouse {
            if draw_button(
                Rect::new(rect.x + 320.0, action_y, 140.0, 40.0),
                "Set Infusion",
                false,
            ) {
                self.infusion_modal_open = true;
            }
        }

        let panel_y = action_y + 60.0;
        if *b_type == BuildingType::Greenhouse {
            herbs::draw_greenhouse_panel(
                building,
                data,
                disciples,
                current_season,
                rect.x + 20.0,
                panel_y,
                rect.w - 40.0,
            );
        } else {
            herbs::draw_herb_garden_panel(
                building,
                data,
                disciples,
                current_season,
                rect.x + 20.0,
                panel_y,
                rect.w - 40.0,
            );
        }

        let mut plot_btn_y = panel_y + 30.0;
        for (i, plot) in building.herb_plots.iter().enumerate() {
            if i >= building.get_max_herb_plots() {
                break;
            }
            if plot.growing.is_none() {
                if draw_button(
                    Rect::new(rect.x + rect.w - 120.0, plot_btn_y + 10.0, 80.0, 30.0),
                    "Plant",
                    false,
                ) {
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
            if result.close_modal {
                self.herb_planting_modal = None;
            }
            if let Some(action) = result.action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.disciple_assignment_modal {
            let result =
                herbs::draw_disciple_assignment_modal(building, disciples, &data.buildings);
            if result.close_modal {
                self.disciple_assignment_modal = false;
            }
            if let Some(action) = result.action {
                return Some(UpdateResult::new().with_action(action));
            }
        }

        if self.infusion_modal_open {
            let result = herbs::draw_infusion_modal(building);
            if result.close_modal {
                self.infusion_modal_open = false;
            }
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
        if draw_button(
            Rect::new(rect.x + 20.0, action_y, 150.0, 40.0),
            "Upgrade (50 SS)",
            false,
        ) {
            return Some(
                UpdateResult::new()
                    .with_action(Action::UpgradeBuilding(building.building_type.clone())),
            );
        }

        let panel_y = action_y + 60.0;
        if let Some(action) = herbs::draw_drying_pavilion_panel(
            building,
            data,
            inventory,
            rect.x + 20.0,
            panel_y,
            rect.w - 40.0,
        ) {
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
        if draw_button(
            Rect::new(rect.x + 20.0, action_y, 150.0, 40.0),
            "Upgrade (50 SS)",
            false,
        ) {
            return Some(
                UpdateResult::new()
                    .with_action(Action::UpgradeBuilding(building.building_type.clone())),
            );
        }

        let panel_y = action_y + 60.0;
        herbs::draw_herb_storage_panel(
            building,
            data,
            inventory,
            rect.x + 20.0,
            panel_y,
            rect.w - 40.0,
        );

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
        let list_rect = Rect::new(
            rect.x + 20.0,
            start_y,
            rect.w - 40.0,
            rect.h - start_y + rect.y - 95.0,
        );
        let card_h = 74.0;
        let card_gap = 10.0;
        let mut available_missions: Vec<&Mission> = Vec::new();
        let mut selected_available = false;
        let selected_desc = self.selected_mission.clone();
        let mouse = vec2(mouse_position().0, mouse_position().1);

        for mission in &data.missions {
            let is_ongoing = ongoing_missions
                .iter()
                .any(|m| m.mission.description == mission.description);
            let is_pending = completed_missions
                .iter()
                .any(|m| m.description == mission.description);
            let is_historically_complete = completed_history.contains(&mission.description);

            let available = if mission.repeatable {
                !is_ongoing && !is_pending
            } else {
                !is_ongoing && !is_pending && !is_historically_complete
            };

            if !available {
                continue;
            }
            available_missions.push(mission);
        }

        let total_h = available_missions.len() as f32 * (card_h + card_gap);
        if list_rect.contains(mouse.into()) {
            let wheel = mouse_wheel().1;
            if total_h > list_rect.h {
                self.mission_scroll -= wheel * 32.0;
                self.mission_scroll = self
                    .mission_scroll
                    .clamp(0.0, (total_h - list_rect.h).max(0.0));
            } else {
                self.mission_scroll = 0.0;
            }
        }

        draw_ui_text(
            "Mission Board",
            rect.x + 20.0,
            start_y - 16.0,
            FONT_HEADER_SIZE,
            PRIMARY,
        );

        let mut m_y = list_rect.y - self.mission_scroll;
        for mission in available_missions {
            let card_rect = Rect::new(list_rect.x, m_y, list_rect.w, card_h);
            let is_selected = selected_desc.as_deref() == Some(mission.description.as_str());
            if is_selected {
                selected_available = true;
            }

            if card_rect.y + card_h >= list_rect.y && card_rect.y <= list_rect.y + list_rect.h {
                self.draw_mission_card(card_rect, mission, is_selected);
            }

            if card_rect.contains(mouse.into()) && is_mouse_button_pressed(MouseButton::Left) {
                self.selected_mission = Some(mission.description.clone());
            }

            if card_rect.contains(mouse.into()) {
                draw_tooltip(
                    mouse,
                    &format!(
                        "{}\nRisk {} | {} ticks",
                        mission.description, mission.danger_level, mission.duration
                    ),
                );
            }

            m_y += card_h + card_gap;
        }

        if total_h > list_rect.h {
            let track_x = list_rect.x + list_rect.w - 7.0;
            let handle_h = (list_rect.h * list_rect.h / total_h).max(24.0);
            let max_offset = (total_h - list_rect.h).max(1.0);
            let handle_y =
                list_rect.y + (self.mission_scroll / max_offset) * (list_rect.h - handle_h);
            draw_rectangle(
                track_x,
                list_rect.y,
                3.0,
                list_rect.h,
                Color::new(0.0, 0.0, 0.0, 0.32),
            );
            draw_rectangle(
                track_x - 1.0,
                handle_y,
                5.0,
                handle_h,
                Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.62),
            );
        }

        if let Some(selected) = &self.selected_mission {
            if selected_available {
                let btn_rect =
                    Rect::new(rect.x + 20.0, rect.y + rect.h - 60.0, rect.w - 40.0, 40.0);
                if draw_button(btn_rect, "Send disciples beyond the gate", false) {
                    return Some(
                        UpdateResult::new().with_transition(StateTransition::ToMissionAssignment(
                            selected.clone(),
                        )),
                    );
                }
            } else {
                draw_ui_text(
                    "Selected mission unavailable.",
                    rect.x + 20.0,
                    rect.y + rect.h - 30.0,
                    FONT_SMALL_SIZE,
                    TEXT_SECONDARY,
                );
            }
        }
        None
    }

    fn draw_mission_card(&self, rect: Rect, mission: &Mission, selected: bool) {
        let hover = rect.contains(mouse_position().into());
        let alpha = if selected {
            0.74
        } else if hover {
            0.52
        } else {
            0.34
        };
        let accent = mission_type_color(&mission.mission_type);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.035, 0.028, 0.02, alpha),
        );
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if selected { 2.0 } else { 1.0 },
            Color::new(
                accent.r,
                accent.g,
                accent.b,
                if selected { 0.88 } else { 0.38 },
            ),
        );
        draw_rectangle(
            rect.x,
            rect.y,
            5.0,
            rect.h,
            Color::new(accent.r, accent.g, accent.b, 0.72),
        );

        let title = mission_title(&mission.description);
        draw_ui_text(
            title,
            rect.x + 16.0,
            rect.y + 24.0,
            FONT_BODY_SIZE,
            if selected {
                TEXT_PRIMARY
            } else {
                Color::new(TEXT_PRIMARY.r, TEXT_PRIMARY.g, TEXT_PRIMARY.b, 0.78)
            },
        );
        draw_ui_text(
            &format!(
                "{} path | Risk {} | {} ticks",
                mission_type_label(&mission.mission_type),
                mission.danger_level,
                mission.duration
            ),
            rect.x + 16.0,
            rect.y + 47.0,
            FONT_SMALL_SIZE,
            Color::new(
                TEXT_SECONDARY.r,
                TEXT_SECONDARY.g,
                TEXT_SECONDARY.b,
                if selected { 0.92 } else { 0.62 },
            ),
        );
        draw_ui_text(
            mission_spoils(&mission.mission_type),
            rect.x + rect.w - 205.0,
            rect.y + 47.0,
            FONT_SMALL_SIZE,
            Color::new(
                PRIMARY.r,
                PRIMARY.g,
                PRIMARY.b,
                if selected { 0.9 } else { 0.48 },
            ),
        );
    }
}

fn mission_title(description: &str) -> &str {
    description.trim_end_matches('.')
}

fn mission_type_label(mission_type: &MissionType) -> &'static str {
    match mission_type {
        MissionType::Exploration => "Scout",
        MissionType::ResourceGathering => "Gather",
        MissionType::MonsterSuppression => "Hunt",
        MissionType::Diplomacy => "Treaty",
        MissionType::RuinDelve => "Ruin",
    }
}

fn mission_spoils(mission_type: &MissionType) -> &'static str {
    match mission_type {
        MissionType::Exploration => "Spoils: rumors, herbs",
        MissionType::ResourceGathering => "Spoils: ore, stones",
        MissionType::MonsterSuppression => "Spoils: hides, prestige",
        MissionType::Diplomacy => "Spoils: favor, trade",
        MissionType::RuinDelve => "Spoils: relics, techniques",
    }
}

fn mission_type_color(mission_type: &MissionType) -> Color {
    match mission_type {
        MissionType::Exploration => SECONDARY,
        MissionType::ResourceGathering => PRIMARY,
        MissionType::MonsterSuppression => ACCENT,
        MissionType::Diplomacy => SUCCESS,
        MissionType::RuinDelve => WARNING,
    }
}
