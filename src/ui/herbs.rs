//! Spirit herb and apothecary UI components
//!
//! This module provides reusable UI components for sect herb gardens,
//! preserving jars, and alchemy prep while keeping game logic separate.

use crate::data::buildings::{Building, BuildingType};
use crate::data::disciples::{Disciple, DiscipleRank};
use crate::data::elements::Element;
use crate::data::herbs::Season;
use crate::data::loader::GameData;
use crate::engine::actions::Action;
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::draw_ui_text;

pub use crate::ui::herb_widgets::draw_season_indicator;

/// Result from herb UI interactions
pub struct HerbUiResult {
    pub action: Option<Action>,
    pub close_modal: bool,
}

impl HerbUiResult {
    pub fn none() -> Self {
        Self {
            action: None,
            close_modal: false,
        }
    }

    pub fn with_action(action: Action) -> Self {
        Self {
            action: Some(action),
            close_modal: true,
        }
    }

    pub fn close() -> Self {
        Self {
            action: None,
            close_modal: true,
        }
    }
}

/// Draw the herb garden panel showing plots and their status
pub fn draw_herb_garden_panel(
    building: &Building,
    data: &GameData,
    disciples: &[Disciple],
    current_season: &Season,
    x: f32,
    y: f32,
    width: f32,
) -> f32 {
    let mut cur_y = y;

    draw_ui_text(
        &format!("Mountain Season: {}", current_season),
        x,
        cur_y,
        FONT_BODY_SIZE,
        SECONDARY,
    );
    cur_y += 30.0;

    // Assigned worker display
    let worker_name = building
        .assigned_disciple
        .and_then(|id| disciples.iter().find(|d| d.id == id))
        .map(|d| d.name.as_str())
        .unwrap_or("None");

    let worker_color = if building.assigned_disciple.is_some() {
        SUCCESS
    } else {
        TEXT_SECONDARY
    };
    draw_ui_text(
        &format!("Garden Attendant: {}", worker_name),
        x,
        cur_y,
        FONT_BODY_SIZE,
        worker_color,
    );
    cur_y += 35.0;

    draw_ink_divider(x, cur_y - 8.0, width - 20.0);
    draw_ui_text(
        "Spirit Herb Terraces",
        x,
        cur_y + 14.0,
        FONT_HEADER_SIZE,
        TEXT_HIGHLIGHT,
    );
    cur_y += 30.0;

    let max_plots = building.get_max_herb_plots();
    for (i, plot) in building.herb_plots.iter().enumerate() {
        if i >= max_plots {
            break;
        }

        let plot_rect = Rect::new(x, cur_y, width - 20.0, 60.0);
        draw_rectangle(
            plot_rect.x,
            plot_rect.y,
            plot_rect.w,
            plot_rect.h,
            Color::new(0.1, 0.15, 0.1, 0.8),
        );
        draw_rectangle(
            plot_rect.x,
            plot_rect.y,
            6.0,
            plot_rect.h,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.45),
        );
        draw_rectangle_lines(
            plot_rect.x,
            plot_rect.y,
            plot_rect.w,
            plot_rect.h,
            1.0,
            PANEL_BORDER,
        );

        if let Some(ref growing) = plot.growing {
            // Get herb info
            let herb_name = data
                .herbs
                .get(&growing.herb_id)
                .map(|h| h.name.as_str())
                .unwrap_or(&growing.herb_id);

            let herb_element = data
                .herbs
                .get(&growing.herb_id)
                .map(|h| h.element.clone())
                .unwrap_or(Element::None);

            // Draw herb name with element color
            draw_ui_text(
                &format!("Terrace {}: {}", i + 1, herb_name),
                x + 10.0,
                cur_y + 25.0,
                FONT_BODY_SIZE,
                herb_element.color(),
            );

            if growing.is_mature() {
                // Ready to harvest
                if building.assigned_disciple.is_some() {
                    draw_ui_text(
                        "Ripe - attendant harvesting",
                        x + 10.0,
                        cur_y + 50.0,
                        FONT_SMALL_SIZE,
                        SUCCESS,
                    );
                } else {
                    draw_ui_text(
                        "Ripe - no attendant assigned",
                        x + 10.0,
                        cur_y + 50.0,
                        FONT_SMALL_SIZE,
                        FAILURE,
                    );
                }
            } else {
                // Show growth progress
                let total_time = data
                    .herbs
                    .get(&growing.herb_id)
                    .map(|h| h.grow_time_ticks)
                    .unwrap_or(100);
                let progress = 1.0 - (growing.ticks_remaining as f32 / total_time as f32);

                draw_progress_bar(
                    Rect::new(x + 10.0, cur_y + 40.0, width - 50.0, 15.0),
                    progress,
                    SECONDARY,
                );
                draw_ui_text(
                    &format!("{:.0}%", progress * 100.0),
                    x + width - 35.0,
                    cur_y + 52.0,
                    FONT_SMALL_SIZE,
                    TEXT_SECONDARY,
                );
            }
        } else {
            // Empty plot
            draw_ui_text(
                &format!("Terrace {}: vacant spirit soil", i + 1),
                x + 10.0,
                cur_y + 35.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
        }

        cur_y += 65.0;
    }

    // Show locked plots
    for i in building.herb_plots.len()..max_plots {
        draw_ui_text(
            &format!("Terrace {}: restore the garden to unlock", i + 1),
            x,
            cur_y + 20.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        cur_y += 30.0;
    }

    cur_y
}

/// Draw the herb planting modal
pub fn draw_herb_planting_modal(
    building: &Building,
    data: &GameData,
    current_season: &Season,
    selected_plot: usize,
) -> HerbUiResult {
    let (screen_w, screen_h) = (screen_width(), screen_height());
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

    let modal_w = 450.0;
    let modal_h = 500.0;
    let modal_x = (screen_w - modal_w) / 2.0;
    let modal_y = (screen_h - modal_h) / 2.0;

    draw_panel(
        Rect::new(modal_x, modal_y, modal_w, modal_h),
        Some(&format!("Sow Spirit Herb - Terrace {}", selected_plot + 1)),
    );

    if draw_button(
        Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
        "X",
        false,
    ) {
        return HerbUiResult::close();
    }

    let max_tier = building.get_max_herb_tier();
    let mut h_y = modal_y + 55.0;

    draw_ui_text(
        &format!(
            "Seasonal omen: {} | Garden grade: tier {}",
            current_season, max_tier
        ),
        modal_x + 20.0,
        h_y,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    h_y += 30.0;

    // List plantable herbs
    let mut found_any = false;
    for herb in data.herbs.values() {
        // Check tier restriction
        if herb.tier > max_tier {
            continue;
        }

        // Check season (greenhouse with infusion can bypass)
        let in_season = herb.grow_seasons.contains(current_season);
        let has_infusion = building.building_type == BuildingType::Greenhouse
            && building.infused_element.as_ref() == Some(&herb.element);

        let can_plant = in_season || has_infusion;

        // Display herb option
        let status = if !in_season && !has_infusion {
            " (out of season)"
        } else {
            ""
        };

        let label = format!("{} [{}] T{}{}", herb.name, herb.element, herb.tier, status);
        let text_color = if can_plant {
            herb.element.color()
        } else {
            TEXT_SECONDARY
        };

        draw_ui_text(
            &label,
            modal_x + 20.0,
            h_y + 18.0,
            FONT_BODY_SIZE,
            text_color,
        );

        if can_plant {
            found_any = true;
            if draw_button(
                Rect::new(modal_x + modal_w - 100.0, h_y, 80.0, 30.0),
                "Sow",
                false,
            ) {
                return HerbUiResult::with_action(Action::PlantHerb(
                    building.id,
                    selected_plot,
                    herb.id.clone(),
                ));
            }
        }

        // Tooltip on hover
        let text_rect = Rect::new(modal_x + 20.0, h_y, modal_w - 140.0, 30.0);
        if text_rect.contains(mouse_position().into()) {
            let lines = vec![
                herb.description.clone(),
                format!(
                    "Growth vigil: {}",
                    crate::data::time::days_label(herb.grow_time_ticks)
                ),
                format!(
                    "Favorable seasons: {}",
                    herb.grow_seasons
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ];
            draw_tooltip_box(mouse_position().0 + 15.0, mouse_position().1 + 15.0, &lines);
        }

        h_y += 40.0;

        if h_y > modal_y + modal_h - 50.0 {
            draw_ui_text("...", modal_x + 20.0, h_y, FONT_BODY_SIZE, TEXT_SECONDARY);
            break;
        }
    }

    if !found_any {
        draw_ui_text(
            "No seed slips favor this season.",
            modal_x + 20.0,
            h_y,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
    }

    HerbUiResult::none()
}

/// Draw the disciple assignment modal for buildings
pub fn draw_disciple_assignment_modal(
    building: &Building,
    disciples: &[Disciple],
    all_buildings: &[Building],
) -> HerbUiResult {
    let (screen_w, screen_h) = (screen_width(), screen_height());
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

    let modal_w = 400.0;
    let modal_h = 450.0;
    let modal_x = (screen_w - modal_w) / 2.0;
    let modal_y = (screen_h - modal_h) / 2.0;

    draw_panel(
        Rect::new(modal_x, modal_y, modal_w, modal_h),
        Some("Appoint Hall Attendant"),
    );

    if draw_button(
        Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
        "X",
        false,
    ) {
        return HerbUiResult::close();
    }

    let mut d_y = modal_y + 55.0;

    // Unassign option
    if building.assigned_disciple.is_some() {
        if draw_button(
            Rect::new(modal_x + 20.0, d_y, modal_w - 40.0, 35.0),
            "Recall Attendant",
            false,
        ) {
            return HerbUiResult::with_action(Action::AssignDiscipleToBuilding(building.id, None));
        }
        d_y += 45.0;
    }

    draw_ui_text(
        "Outer disciples awaiting duty:",
        modal_x + 20.0,
        d_y,
        FONT_BODY_SIZE,
        TEXT_HIGHLIGHT,
    );
    d_y += 30.0;

    // Get IDs of disciples already assigned to buildings
    let assigned_ids: std::collections::HashSet<u64> = all_buildings
        .iter()
        .filter_map(|b| b.assigned_disciple)
        .collect();

    let mut found_any = false;
    for disciple in disciples.iter().filter(|d| d.rank == DiscipleRank::Outer) {
        let is_assigned_elsewhere =
            assigned_ids.contains(&disciple.id) && building.assigned_disciple != Some(disciple.id);

        if is_assigned_elsewhere {
            draw_ui_text(
                &format!("{} (serving another hall)", disciple.name),
                modal_x + 20.0,
                d_y + 18.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
            d_y += 30.0;
            continue;
        }

        found_any = true;
        let is_current = building.assigned_disciple == Some(disciple.id);
        let label = if is_current {
            format!(
                "{} (current attendant) - Spirit Root: {}",
                disciple.name, disciple.attributes.spirit
            )
        } else {
            format!(
                "{} - Spirit Root: {}",
                disciple.name, disciple.attributes.spirit
            )
        };

        if draw_button(
            Rect::new(modal_x + 20.0, d_y, modal_w - 40.0, 35.0),
            &label,
            is_current,
        ) {
            if !is_current {
                return HerbUiResult::with_action(Action::AssignDiscipleToBuilding(
                    building.id,
                    Some(disciple.id),
                ));
            }
        }
        d_y += 40.0;

        if d_y > modal_y + modal_h - 50.0 {
            break;
        }
    }

    if !found_any {
        draw_ui_text(
            "No outer disciples await duty.",
            modal_x + 20.0,
            d_y,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
        d_y += 25.0;
        draw_ui_text(
            "Recruit more hands at the Sect Hall.",
            modal_x + 20.0,
            d_y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
    }

    HerbUiResult::none()
}

/// Draw the drying pavilion panel
pub fn draw_drying_pavilion_panel(
    building: &Building,
    data: &GameData,
    inventory: &std::collections::HashMap<String, u32>,
    x: f32,
    y: f32,
    width: f32,
) -> Option<Action> {
    let mut cur_y = y;

    let loss_rate = building.get_drying_loss_rate();
    let output_per_5 = ((5.0 * (1.0 - loss_rate)).ceil() as u32).max(1);

    draw_ui_text(
        &format!(
            "Drying rite: 5 fresh -> {} preserved ({:.0}% loss)",
            output_per_5,
            loss_rate * 100.0
        ),
        x,
        cur_y,
        FONT_BODY_SIZE,
        TEXT_SECONDARY,
    );
    cur_y += 35.0;

    draw_ui_text(
        "Fresh Herbs for the Drying Racks",
        x,
        cur_y,
        FONT_HEADER_SIZE,
        TEXT_HIGHLIGHT,
    );
    cur_y += 30.0;

    let mut action: Option<Action> = None;

    // List raw herbs that can be dried
    for herb in data.herbs.values() {
        let herb_id = &herb.id;

        // Skip if already dried
        if herb_id.starts_with("dried_") {
            continue;
        }

        let count = *inventory.get(herb_id).unwrap_or(&0);
        if count >= 5 {
            let label = format!(
                "{}: {} fresh (preserve 5 -> {})",
                herb.name, count, output_per_5
            );
            if draw_button(Rect::new(x, cur_y, width - 20.0, 35.0), &label, false) {
                action = Some(Action::ProcessDryingPavilion(building.id, herb_id.clone()));
            }
            cur_y += 40.0;
        } else if count > 0 {
            draw_ui_text(
                &format!("{}: {} fresh (need 5)", herb.name, count),
                x,
                cur_y + 20.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
            cur_y += 30.0;
        }
    }

    action
}

/// Draw the greenhouse panel with infusion controls
pub fn draw_greenhouse_panel(
    building: &Building,
    data: &GameData,
    disciples: &[Disciple],
    current_season: &Season,
    x: f32,
    y: f32,
    width: f32,
) -> f32 {
    let mut cur_y = y;

    // Current infusion
    let infusion_text = building
        .infused_element
        .as_ref()
        .map(|e| format!("{}", e))
        .unwrap_or_else(|| "None".to_string());

    let infusion_color = building
        .infused_element
        .as_ref()
        .map(|e| e.color())
        .unwrap_or(TEXT_SECONDARY);

    draw_ui_text(
        "Season-Bending Infusion:",
        x,
        cur_y,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    draw_ui_text(
        &infusion_text,
        x + 150.0,
        cur_y,
        FONT_BODY_SIZE,
        infusion_color,
    );
    cur_y += 30.0;

    if building.infused_element.is_some() {
        draw_ui_text(
            "The array lets matching herbs ignore the mortal season.",
            x,
            cur_y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        cur_y += 25.0;
    }

    cur_y += 10.0;

    // Show standard herb garden panel
    cur_y = draw_herb_garden_panel(building, data, disciples, current_season, x, cur_y, width);

    cur_y
}

/// Draw infusion selection modal
pub fn draw_infusion_modal(building: &Building) -> HerbUiResult {
    let (screen_w, screen_h) = (screen_width(), screen_height());
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

    let modal_w = 350.0;
    let modal_h = 400.0;
    let modal_x = (screen_w - modal_w) / 2.0;
    let modal_y = (screen_h - modal_h) / 2.0;

    draw_panel(
        Rect::new(modal_x, modal_y, modal_w, modal_h),
        Some("Tune Greenhouse Array"),
    );

    if draw_button(
        Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
        "X",
        false,
    ) {
        return HerbUiResult::close();
    }

    let mut e_y = modal_y + 55.0;

    // Clear infusion option
    if building.infused_element.is_some() {
        if draw_button(
            Rect::new(modal_x + 20.0, e_y, modal_w - 40.0, 35.0),
            "Extinguish Array",
            false,
        ) {
            return HerbUiResult::with_action(Action::SetGreenhouseInfusion(building.id, None));
        }
        e_y += 45.0;
    }

    draw_ui_text(
        "Choose array aspect:",
        modal_x + 20.0,
        e_y,
        FONT_BODY_SIZE,
        TEXT_HIGHLIGHT,
    );
    e_y += 30.0;

    let elements = [
        Element::Wood,
        Element::Fire,
        Element::Water,
        Element::Metal,
        Element::Earth,
    ];

    for elem in elements {
        let is_current = building.infused_element.as_ref() == Some(&elem);
        let btn_color = elem.color();

        // Draw colored indicator
        draw_rectangle(modal_x + 20.0, e_y + 5.0, 20.0, 25.0, btn_color);

        if draw_button(
            Rect::new(modal_x + 50.0, e_y, modal_w - 70.0, 35.0),
            &format!("{}", elem),
            is_current,
        ) {
            if !is_current {
                return HerbUiResult::with_action(Action::SetGreenhouseInfusion(
                    building.id,
                    Some(elem),
                ));
            }
        }
        e_y += 45.0;
    }

    HerbUiResult::none()
}

/// Draw the herb storage panel
pub fn draw_herb_storage_panel(
    building: &Building,
    data: &GameData,
    inventory: &std::collections::HashMap<String, u32>,
    x: f32,
    y: f32,
    _width: f32,
) -> f32 {
    let mut cur_y = y;

    let decay_reduction = building.get_decay_reduction();
    draw_ui_text(
        &format!("Preservation Seal: {:.0}%", decay_reduction * 100.0),
        x,
        cur_y,
        FONT_BODY_SIZE,
        SUCCESS,
    );
    cur_y += 30.0;

    let base_decay = 10.0;
    let effective_decay = base_decay * (1.0 - decay_reduction);
    draw_ui_text(
        &format!("Expected withering: {:.1}% per season", effective_decay),
        x,
        cur_y,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    cur_y += 35.0;

    draw_ui_text(
        "Apothecary Stores",
        x,
        cur_y,
        FONT_HEADER_SIZE,
        TEXT_HIGHLIGHT,
    );
    cur_y += 30.0;

    // List herbs in inventory
    let mut found_any = false;
    for herb in data.herbs.values() {
        let raw_count = *inventory.get(&herb.id).unwrap_or(&0);
        let dried_id = format!("dried_{}", herb.id);
        let dried_count = *inventory.get(&dried_id).unwrap_or(&0);

        if raw_count > 0 || dried_count > 0 {
            found_any = true;

            if raw_count > 0 {
                draw_rectangle(x, cur_y, 8.0, 18.0, herb.element.color());
                draw_ui_text(
                    &format!("{}: {}", herb.name, raw_count),
                    x + 15.0,
                    cur_y + 15.0,
                    FONT_BODY_SIZE,
                    TEXT_PRIMARY,
                );
                cur_y += 25.0;
            }

            if dried_count > 0 {
                draw_rectangle(x, cur_y, 8.0, 18.0, herb.element.color());
                draw_ui_text(
                    &format!("Dried {}: {}", herb.name, dried_count),
                    x + 15.0,
                    cur_y + 15.0,
                    FONT_BODY_SIZE,
                    SECONDARY,
                );
                cur_y += 25.0;
            }
        }
    }

    if !found_any {
        draw_ui_text(
            "No herbs rest in the jars.",
            x,
            cur_y,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
        cur_y += 25.0;
    }

    cur_y
}
