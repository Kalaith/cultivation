use crate::data::history::DeceasedDisciple;
use crate::data::laws::CultivationLaw;
use crate::data::loader::GameData;
use crate::data::tech::Technology;
use crate::state::{StateTransition, UpdateResult};
use crate::ui::components::*;
use crate::ui::theme::*;
use macroquad::prelude::*;
use macroquad_toolkit::ui::{draw_ui_text, measure_ui_text};

pub struct LibraryState;

impl LibraryState {
    pub fn new() -> Self {
        Self
    }

    pub fn update(
        &mut self,
        data: &GameData,
        spirit_stones: u32,
        deceased: &[DeceasedDisciple],
    ) -> UpdateResult {
        if is_key_pressed(KeyCode::Escape) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        draw_mountain_sect_backdrop();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let header_h = 86.0;
        draw_archive_header(screen_w, header_h, spirit_stones, deceased.len(), data);

        let content_y = header_h + 16.0;
        let content_h = screen_h - content_y - 24.0;
        let gutter = 18.0;
        let left_w = (screen_w * 0.33).clamp(330.0, 430.0);
        let right_w = (screen_w * 0.28).clamp(280.0, 360.0);
        let center_w = screen_w - left_w - right_w - gutter * 4.0;

        let ancestral_rect = Rect::new(gutter, content_y, left_w, content_h);
        let scripture_rect = Rect::new(
            ancestral_rect.x + left_w + gutter,
            content_y,
            center_w,
            content_h,
        );
        let annals_rect = Rect::new(
            scripture_rect.x + center_w + gutter,
            content_y,
            right_w,
            content_h,
        );

        draw_ancestral_hall(ancestral_rect, deceased);
        draw_scripture_pavilion(scripture_rect, data);
        draw_sect_annals(annals_rect, data, deceased, spirit_stones);

        if draw_button(
            Rect::new(screen_w - 132.0, screen_h - 62.0, 108.0, 42.0),
            "Return",
            false,
        ) {
            return UpdateResult::new().with_transition(StateTransition::ToSectBase);
        }

        UpdateResult::new()
    }

    pub fn draw(&self, _data: &GameData, _spirit_stones: u32, _deceased: &[DeceasedDisciple]) {
        // Handled in update.
    }
}

fn draw_archive_header(
    screen_w: f32,
    header_h: f32,
    spirit_stones: u32,
    fallen_count: usize,
    data: &GameData,
) {
    draw_panel(Rect::new(0.0, 0.0, screen_w, header_h), None);
    draw_screen_title(
        "Ancestral Archive",
        "Memory, scripture, and unfinished vows of the fallen mountain sect",
        24.0,
        38.0,
    );

    let mut seal_x = screen_w - 508.0;
    seal_x += draw_resource_seal(seal_x, 56.0, "Stones", spirit_stones, PRIMARY) + 8.0;
    seal_x += draw_resource_seal(seal_x, 56.0, "Fallen", fallen_count as u32, FAILURE) + 8.0;
    seal_x += draw_resource_seal(seal_x, 56.0, "Laws", data.laws.len() as u32, SECONDARY) + 8.0;
    draw_resource_seal(seal_x, 56.0, "Doctrines", data.techs.len() as u32, WARNING);
}

fn draw_ancestral_hall(rect: Rect, deceased: &[DeceasedDisciple]) {
    draw_panel(rect, Some("Hall of Fallen Tablets"));

    let mut y = rect.y + 62.0;
    draw_wrapped_text(
        "Every name kept here is a warning and a promise. The patriarch rebuilds with their ashes in the foundation.",
        rect.x + 22.0,
        y,
        rect.w - 44.0,
        FONT_BODY_SIZE,
        TEXT_SECONDARY,
    );
    y += 82.0;
    draw_ink_divider(rect.x + 22.0, y, rect.w - 44.0);
    y += 34.0;

    if deceased.is_empty() {
        draw_empty_memorial(rect, y);
        return;
    }

    for (idx, disciple) in deceased.iter().rev().take(6).enumerate() {
        let card = Rect::new(rect.x + 18.0, y, rect.w - 36.0, 72.0);
        draw_memorial_tablet(card, idx, disciple);
        y += 82.0;
        if y > rect.y + rect.h - 96.0 {
            break;
        }
    }

    if deceased.len() > 6 {
        draw_ui_text(
            &format!(
                "{} more names rest deeper in the annals.",
                deceased.len() - 6
            ),
            rect.x + 24.0,
            rect.y + rect.h - 24.0,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
    }
}

fn draw_empty_memorial(rect: Rect, y: f32) {
    draw_rectangle(
        rect.x + 24.0,
        y,
        rect.w - 48.0,
        118.0,
        Color::new(0.04, 0.030, 0.020, 0.50),
    );
    draw_rectangle_lines(
        rect.x + 24.0,
        y,
        rect.w - 48.0,
        118.0,
        1.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.38),
    );
    draw_wrapped_text(
        "No disciple has fallen yet. The empty tablets wait in silence.",
        rect.x + 42.0,
        y + 42.0,
        rect.w - 84.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
}

fn draw_memorial_tablet(rect: Rect, idx: usize, disciple: &DeceasedDisciple) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.04, 0.030, 0.020, 0.58),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(ACCENT.r, ACCENT.g, ACCENT.b, 0.54),
    );
    draw_circle(
        rect.x + 24.0,
        rect.y + 26.0,
        8.0,
        Color::new(ACCENT.r, ACCENT.g, ACCENT.b, 0.52),
    );
    draw_ui_text(
        &format!("{}. {}", idx + 1, disciple.name),
        rect.x + 44.0,
        rect.y + 26.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "{} | {}",
            realm_label(&disciple.realm_at_death),
            disciple.cause_of_death
        ),
        rect.x + 44.0,
        rect.y + 52.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
    draw_ui_text(
        &format!("Tick {}", disciple.tick_of_death),
        rect.x + rect.w - 84.0,
        rect.y + 52.0,
        FONT_SMALL_SIZE,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.72),
    );
}

fn draw_scripture_pavilion(rect: Rect, data: &GameData) {
    draw_panel(rect, Some("Scripture Pavilion"));
    let mut y = rect.y + 64.0;

    draw_ui_text(
        "Cultivation Laws",
        rect.x + 22.0,
        y,
        FONT_HEADER_SIZE,
        TEXT_HIGHLIGHT,
    );
    y += 28.0;
    let mut laws: Vec<&CultivationLaw> = data.laws.values().collect();
    laws.sort_by(|a, b| a.name.cmp(&b.name));

    if laws.is_empty() {
        y = draw_wrapped_text(
            "No complete cultivation laws have survived the sect's collapse.",
            rect.x + 24.0,
            y + 8.0,
            rect.w - 48.0,
            FONT_BODY_SIZE,
            TEXT_SECONDARY,
        );
    } else {
        for law in laws.iter().take(3) {
            let card = Rect::new(rect.x + 20.0, y + 6.0, rect.w - 40.0, 86.0);
            draw_law_card(card, law);
            y += 96.0;
        }
    }

    y += 12.0;
    draw_ink_divider(rect.x + 22.0, y, rect.w - 44.0);
    y += 36.0;
    draw_ui_text(
        "Recovered Doctrines",
        rect.x + 22.0,
        y,
        FONT_HEADER_SIZE,
        TEXT_HIGHLIGHT,
    );
    y += 34.0;

    let mut techs: Vec<&Technology> = data.techs.values().collect();
    techs.sort_by(|a, b| {
        a.cost_spirit_stones
            .cmp(&b.cost_spirit_stones)
            .then(a.name.cmp(&b.name))
    });
    for tech in techs.iter().take(5) {
        if y > rect.y + rect.h - 62.0 {
            break;
        }
        draw_doctrine_row(rect.x + 24.0, y, rect.w - 48.0, tech);
        y += 52.0;
    }
}

fn draw_law_card(rect: Rect, law: &CultivationLaw) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(0.035, 0.026, 0.018, 0.54),
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::new(SECONDARY.r, SECONDARY.g, SECONDARY.b, 0.54),
    );
    draw_ui_text(
        &law.name,
        rect.x + 14.0,
        rect.y + 25.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    draw_ui_text(
        &format!(
            "Element {:?} | Breakthrough {:+.0}%",
            law.element,
            law.breakthrough_modifier * 100.0
        ),
        rect.x + 14.0,
        rect.y + 48.0,
        FONT_SMALL_SIZE,
        SECONDARY,
    );
    draw_wrapped_text(
        &law.description,
        rect.x + 14.0,
        rect.y + 70.0,
        rect.w - 28.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
}

fn draw_doctrine_row(x: f32, y: f32, width: f32, tech: &Technology) {
    draw_rectangle(
        x,
        y - 20.0,
        width,
        42.0,
        Color::new(0.04, 0.030, 0.020, 0.42),
    );
    draw_rectangle_lines(
        x,
        y - 20.0,
        width,
        42.0,
        1.0,
        Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.30),
    );
    draw_ui_text(&tech.name, x + 12.0, y, FONT_BODY_SIZE, TEXT_PRIMARY);
    let cost = format!("{} SS", tech.cost_spirit_stones);
    let dims = measure_ui_text(&cost, None, FONT_SMALL_SIZE as u16, 1.0);
    draw_ui_text(
        &cost,
        x + width - dims.width - 12.0,
        y,
        FONT_SMALL_SIZE,
        WARNING,
    );
    draw_ui_text(
        &tech.description,
        x + 12.0,
        y + 18.0,
        FONT_SMALL_SIZE,
        TEXT_SECONDARY,
    );
}

fn draw_sect_annals(
    rect: Rect,
    data: &GameData,
    deceased: &[DeceasedDisciple],
    spirit_stones: u32,
) {
    draw_panel(rect, Some("Sect Annals"));
    let x = rect.x + 22.0;
    let mut y = rect.y + 68.0;

    draw_wrapped_text(
        "The archive does not make the sect strong by itself. It tells the patriarch what must be rebuilt, what must be risked, and who paid the price.",
        x,
        y,
        rect.w - 44.0,
        FONT_BODY_SIZE,
        TEXT_PRIMARY,
    );
    y += 118.0;
    draw_ink_divider(x, y, rect.w - 44.0);
    y += 34.0;

    let metrics = [
        ("Stored Spirit Stones", spirit_stones.to_string(), PRIMARY),
        ("Known Laws", data.laws.len().to_string(), SECONDARY),
        ("Recovered Doctrines", data.techs.len().to_string(), WARNING),
        ("Fallen Disciples", deceased.len().to_string(), FAILURE),
    ];
    for (label, value, color) in metrics {
        draw_ui_text(label, x, y, FONT_SMALL_SIZE, TEXT_SECONDARY);
        draw_ui_text(&value, x + 174.0, y, FONT_BODY_SIZE, color);
        y += 32.0;
    }

    y += 12.0;
    draw_ink_divider(x, y, rect.w - 44.0);
    y += 36.0;
    draw_wrapped_text(
        annal_guidance(deceased),
        x,
        y,
        rect.w - 44.0,
        FONT_BODY_SIZE,
        TEXT_SECONDARY,
    );
}

fn annal_guidance(deceased: &[DeceasedDisciple]) -> &'static str {
    if deceased.is_empty() {
        "The first generation still breathes. Spend this peace carefully: halls, training, and medicine decide whether future tablets remain blank."
    } else if deceased.len() < 3 {
        "The mountain has tasted loss. Honor it by preparing stronger disciples before sending them beyond the gate again."
    } else {
        "Too many tablets are filled. The patriarch must rebuild safeguards before ambition consumes the sect."
    }
}

fn realm_label(id: &str) -> String {
    if id.is_empty() {
        return "Unknown Realm".to_string();
    }
    id.split('_')
        .flat_map(|part| part.split('-'))
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
