//! Technology/Research Tree UI Components
//!
//! Data-driven UI for the sect's research system.

use crate::data::loader::GameData;
use crate::data::tech::Technology;
use crate::engine::actions::Action;
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;

/// State for the tech tree modal
pub struct TechTreeState {
    pub scroll_offset: f32,
    pub hovered_tech: Option<String>,
}

impl TechTreeState {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0.0,
            hovered_tech: None,
        }
    }
}

/// Calculate the display height needed for a single tech entry
fn calc_tech_entry_height(tech: &Technology) -> f32 {
    let base_height = 70.0; // Name + description + spacing
    let unlocks_height = if !tech.unlocks_buildings.is_empty() || !tech.unlocks_missions.is_empty()
    {
        25.0
    } else {
        0.0
    };
    let prereq_height = if !tech.prerequisites.is_empty() {
        20.0
    } else {
        0.0
    };
    base_height + unlocks_height + prereq_height
}

/// Draw the tech tree modal with proper layout and scrolling
pub fn draw_tech_tree_modal(
    state: &mut TechTreeState,
    data: &GameData,
    unlocked_techs: &[String],
    spirit_stones: u32,
) -> Option<Action> {
    let (screen_w, screen_h) = (screen_width(), screen_height());

    // Darken background
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

    let modal_w = 650.0;
    let modal_h = 550.0;
    let modal_x = (screen_w - modal_w) / 2.0;
    let modal_y = (screen_h - modal_h) / 2.0;

    draw_panel(
        Rect::new(modal_x, modal_y, modal_w, modal_h),
        Some("Sect Knowledge Tree"),
    );

    // Close button (top-right)
    draw_button(
        Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
        "Close",
        false,
    );

    // Spirit stones display
    draw_text(
        &format!("Spirit Stones: {}", spirit_stones),
        modal_x + 200.0,
        modal_y + 28.0,
        FONT_BODY_SIZE,
        TEXT_HIGHLIGHT,
    );

    // Content area (with padding for header)
    let content_x = modal_x + 15.0;
    let content_y = modal_y + 50.0;
    let content_w = modal_w - 30.0;
    let content_h = modal_h - 60.0;

    // Sort techs by prerequisites (dependency order)
    let sorted_techs = sort_techs_by_dependency(data);

    // Calculate total content height
    let total_height: f32 = sorted_techs.iter().map(|t| calc_tech_entry_height(t)).sum();

    // Handle scrolling with mouse wheel
    let mouse_pos: Vec2 = mouse_position().into();
    let content_rect = Rect::new(content_x, content_y, content_w, content_h);
    if content_rect.contains(mouse_pos) {
        let wheel = mouse_wheel().1;
        state.scroll_offset -= wheel * 30.0;
        state.scroll_offset = state
            .scroll_offset
            .clamp(0.0, (total_height - content_h + 20.0).max(0.0));
    }

    // Begin scissor/clip region for scrolling
    // Note: macroquad doesn't have built-in scissor, so we'll just offset and skip out-of-bounds items

    let mut y_pos = content_y - state.scroll_offset;
    let mut result_action: Option<Action> = None;
    state.hovered_tech = None;

    for tech in &sorted_techs {
        let entry_height = calc_tech_entry_height(tech);

        // Skip if completely above visible area
        if y_pos + entry_height < content_y {
            y_pos += entry_height;
            continue;
        }

        // Stop if below visible area
        if y_pos > content_y + content_h {
            break;
        }

        // Draw tech entry
        if let Some(action) = draw_tech_entry(
            tech,
            data,
            unlocked_techs,
            spirit_stones,
            content_x,
            y_pos,
            content_w,
            state,
        ) {
            result_action = Some(action);
        }

        y_pos += entry_height;
    }

    // Draw scroll indicators and scrollbar if needed
    if total_height > content_h {
        if state.scroll_offset > 0.0 {
            draw_text(
                "^ Scroll up ^",
                modal_x + modal_w / 2.0 - 40.0,
                content_y + 15.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }
        if state.scroll_offset < total_height - content_h {
            draw_text(
                "v Scroll down v",
                modal_x + modal_w / 2.0 - 45.0,
                modal_y + modal_h - 15.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }

        let track_x = content_x + content_w - 6.0;
        let track_y = content_y;
        let track_h = content_h;
        draw_rectangle(track_x, track_y, 4.0, track_h, PANEL_BORDER);

        let handle_h = (content_h * content_h / total_height).max(20.0);
        let max_offset = (total_height - content_h).max(1.0);
        let handle_y = track_y + (state.scroll_offset / max_offset) * (track_h - handle_h);
        draw_rectangle(track_x - 1.0, handle_y, 6.0, handle_h, TEXT_HIGHLIGHT);
    }

    // Draw tooltip for hovered tech
    if let Some(tech_id) = &state.hovered_tech {
        if let Some(tech) = data.techs.get(tech_id) {
            draw_tech_tooltip(tech, data, unlocked_techs);
        }
    }

    result_action
}

/// Sort technologies by dependency order (prerequisites first)
fn sort_techs_by_dependency(data: &GameData) -> Vec<&Technology> {
    let mut result: Vec<&Technology> = Vec::new();
    let mut added: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Simple topological sort
    let techs: Vec<&Technology> = data.techs.values().collect();
    let mut remaining = techs.clone();

    while !remaining.is_empty() {
        let mut made_progress = false;

        remaining.retain(|tech| {
            let prereqs_met = tech.prerequisites.iter().all(|p| added.contains(p));
            if prereqs_met || tech.prerequisites.is_empty() {
                result.push(tech);
                added.insert(tech.id.clone());
                made_progress = true;
                false // Remove from remaining
            } else {
                true // Keep in remaining
            }
        });

        // Prevent infinite loop if there's a cycle
        if !made_progress && !remaining.is_empty() {
            // Just add remaining in order
            for tech in remaining {
                result.push(tech);
            }
            break;
        }
    }

    result
}

/// Draw a single tech entry
fn draw_tech_entry(
    tech: &Technology,
    data: &GameData,
    unlocked_techs: &[String],
    spirit_stones: u32,
    x: f32,
    y: f32,
    width: f32,
    state: &mut TechTreeState,
) -> Option<Action> {
    let is_unlocked = unlocked_techs.contains(&tech.id);
    let prereqs_met = tech
        .prerequisites
        .iter()
        .all(|p| unlocked_techs.contains(p));
    let can_afford = spirit_stones >= tech.cost_spirit_stones;
    let can_research = !is_unlocked && prereqs_met && can_afford;

    // Entry background
    let bg_color = if is_unlocked {
        Color::new(0.1, 0.2, 0.1, 0.6) // Green tint for unlocked
    } else if prereqs_met {
        Color::new(0.15, 0.15, 0.1, 0.6) // Yellow tint for available
    } else {
        Color::new(0.1, 0.1, 0.1, 0.4) // Gray for locked
    };

    let entry_height = calc_tech_entry_height(tech);
    let entry_rect = Rect::new(x, y, width, entry_height - 5.0);

    draw_rectangle(
        entry_rect.x,
        entry_rect.y,
        entry_rect.w,
        entry_rect.h,
        bg_color,
    );

    // Border color based on status
    let border_color = if is_unlocked {
        SUCCESS
    } else if can_research {
        PRIMARY
    } else if prereqs_met {
        Color::new(0.6, 0.6, 0.2, 1.0) // Can't afford
    } else {
        PANEL_BORDER
    };
    draw_rectangle_lines(
        entry_rect.x,
        entry_rect.y,
        entry_rect.w,
        entry_rect.h,
        2.0,
        border_color,
    );

    // Check hover for tooltip
    let mouse_pos: Vec2 = mouse_position().into();
    if entry_rect.contains(mouse_pos) {
        state.hovered_tech = Some(tech.id.clone());
    }

    let mut text_y = y + 22.0;

    // Tech name with status indicator
    let status_icon = if is_unlocked {
        "[OK]"
    } else if prereqs_met {
        "[?]"
    } else {
        "[X]"
    };
    let name_color = if is_unlocked {
        SUCCESS
    } else if prereqs_met {
        TEXT_PRIMARY
    } else {
        TEXT_SECONDARY
    };

    draw_text(status_icon, x + 10.0, text_y, FONT_BODY_SIZE, name_color);
    draw_text(&tech.name, x + 50.0, text_y, FONT_HEADER_SIZE, name_color);

    // Cost (right-aligned)
    if !is_unlocked {
        let cost_text = format!("{} SS", tech.cost_spirit_stones);
        let cost_color = if can_afford { TEXT_HIGHLIGHT } else { FAILURE };
        draw_text(
            &cost_text,
            x + width - 100.0,
            text_y,
            FONT_BODY_SIZE,
            cost_color,
        );
    }

    text_y += 22.0;

    // Description (truncated if too long)
    let desc = if tech.description.len() > 70 {
        format!("{}...", &tech.description[..67])
    } else {
        tech.description.clone()
    };
    draw_text(&desc, x + 15.0, text_y, FONT_SMALL_SIZE, TEXT_SECONDARY);

    text_y += 20.0;

    // Unlocks info
    if !tech.unlocks_buildings.is_empty() || !tech.unlocks_missions.is_empty() {
        let mut unlocks_parts: Vec<String> = Vec::new();

        if !tech.unlocks_buildings.is_empty() {
            let buildings: Vec<String> = tech
                .unlocks_buildings
                .iter()
                .map(|b| format!("{}", b))
                .collect();
            unlocks_parts.push(format!("Buildings: {}", buildings.join(", ")));
        }

        if !tech.unlocks_missions.is_empty() {
            let missions: Vec<String> = tech
                .unlocks_missions
                .iter()
                .map(|m| format!("{:?}", m))
                .collect();
            unlocks_parts.push(format!("Missions: {}", missions.join(", ")));
        }

        let unlocks_text = format!("Unlocks: {}", unlocks_parts.join(" | "));
        draw_text(&unlocks_text, x + 15.0, text_y, FONT_SMALL_SIZE, SECONDARY);
        text_y += 20.0;
    }

    // Prerequisites (if not met)
    if !tech.prerequisites.is_empty() && !is_unlocked {
        let prereq_names: Vec<String> = tech
            .prerequisites
            .iter()
            .filter_map(|id| data.techs.get(id))
            .map(|t| {
                if unlocked_techs.contains(&t.id) {
                    format!("{} [OK]", t.name)
                } else {
                    t.name.clone()
                }
            })
            .collect();

        let prereq_color = if prereqs_met { TEXT_SECONDARY } else { FAILURE };
        draw_text(
            &format!("Requires: {}", prereq_names.join(", ")),
            x + 15.0,
            text_y,
            FONT_SMALL_SIZE,
            prereq_color,
        );
    }

    // Research button
    if can_research {
        let btn_rect = Rect::new(x + width - 110.0, y + entry_height - 40.0, 100.0, 30.0);
        if draw_button(btn_rect, "Research", false) {
            return Some(Action::ResearchTech(tech.id.clone()));
        }
    } else if is_unlocked {
        draw_text(
            "Learned",
            x + width - 80.0,
            y + entry_height - 25.0,
            FONT_SMALL_SIZE,
            SUCCESS,
        );
    }

    None
}

/// Draw detailed tooltip for a tech
fn draw_tech_tooltip(tech: &Technology, data: &GameData, unlocked_techs: &[String]) {
    let (mx, my) = mouse_position();

    let mut lines: Vec<String> = Vec::new();

    lines.push(tech.name.clone());
    lines.push(String::new()); // Empty line
    lines.push(tech.description.clone());
    lines.push(String::new());
    lines.push(format!("Cost: {} Spirit Stones", tech.cost_spirit_stones));

    if !tech.prerequisites.is_empty() {
        lines.push(String::new());
        lines.push("Prerequisites:".to_string());
        for prereq_id in &tech.prerequisites {
            if let Some(prereq) = data.techs.get(prereq_id) {
                let status = if unlocked_techs.contains(prereq_id) {
                    " [Learned]"
                } else {
                    ""
                };
                lines.push(format!("  - {}{}", prereq.name, status));
            }
        }
    }

    if !tech.unlocks_buildings.is_empty() {
        lines.push(String::new());
        lines.push("Unlocks Buildings:".to_string());
        for building in &tech.unlocks_buildings {
            lines.push(format!("  - {}", building));
        }
    }

    if !tech.unlocks_missions.is_empty() {
        lines.push(String::new());
        lines.push("Unlocks Mission Types:".to_string());
        for mission in &tech.unlocks_missions {
            lines.push(format!("  - {:?}", mission));
        }
    }

    draw_tooltip_box(mx + 20.0, my + 20.0, &lines);
}

/// Check if the close button was clicked (call this before draw_tech_tree_modal)
pub fn check_tech_tree_close(modal_x: f32, modal_y: f32, modal_w: f32) -> bool {
    let close_rect = Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0);
    let mouse_pos: Vec2 = mouse_position().into();
    close_rect.contains(mouse_pos) && is_mouse_button_pressed(MouseButton::Left)
}
