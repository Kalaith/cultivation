use super::*;

impl DiscipleRosterState {
    pub(super) fn draw_disciple_details(
        &mut self,
        data: &GameData,
        disciple: &Disciple,
        _idx: usize,
        right_rect: &Rect,
        _inventory: &std::collections::HashMap<String, u32>,
    ) {
        let x = right_rect.x + 24.0;
        let top = right_rect.y + 58.0;
        let stage = data.stages.get(&disciple.realm);
        let realm_name = stage.map(|s| s.name.as_str()).unwrap_or(&disciple.realm);
        let stage_count = stage.map(|s| s.sub_stages.len()).unwrap_or(0).max(1);
        let stage_number = (disciple.sub_stage + 1).min(stage_count);
        let sub_stage_name = stage
            .and_then(|s| s.sub_stages.get(disciple.sub_stage))
            .map(|ss| ss.name.as_str())
            .unwrap_or("Unawakened");
        let progress = disciple.exp as f32 / disciple.exp_to_next_level.max(1) as f32;
        let breakthrough_ready =
            disciple.can_attempt_breakthrough() && disciple.breakthrough_bottleneck.is_none();

        draw_ui_text(&disciple.name, x, top, FONT_TITLE_SIZE, PRIMARY);
        draw_ui_text(
            rank_title(&disciple.rank),
            x + 2.0,
            top + 30.0,
            FONT_BODY_SIZE,
            SECONDARY,
        );

        let circle_center = vec2(right_rect.x + right_rect.w - 145.0, right_rect.y + 142.0);
        draw_cultivation_circle(
            circle_center,
            64.0,
            stage_number,
            stage_count,
            progress,
            breakthrough_ready,
        );
        draw_ui_text(
            if breakthrough_ready {
                "HEAVENLY GATE"
            } else {
                "NEXT GATE"
            },
            circle_center.x - 82.0,
            circle_center.y + 98.0,
            FONT_SMALL_SIZE,
            if breakthrough_ready {
                WARNING
            } else {
                TEXT_SECONDARY
            },
        );

        let mut y = top + 80.0;
        draw_ui_text(realm_name, x, y, FONT_HEADER_SIZE, TEXT_HIGHLIGHT);
        y += 30.0;
        draw_ui_text(
            &format!(
                "Stage {} / {} - {}",
                stage_number, stage_count, sub_stage_name
            ),
            x,
            y,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );
        y += 24.0;
        draw_ui_text(
            &format!(
                "{:.0}% toward the next heavenly gate",
                progress.min(1.0) * 100.0
            ),
            x,
            y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );

        y += 34.0;
        draw_ink_divider(x, y, right_rect.w - 210.0);
        y += 30.0;

        let seal_w = ((right_rect.w - 220.0) / 3.0).max(90.0);
        draw_stat_seal(
            Rect::new(x, y, seal_w, 72.0),
            "Body Vessel",
            disciple.attributes.body,
            ACCENT,
        );
        draw_stat_seal(
            Rect::new(x + seal_w + 10.0, y, seal_w, 72.0),
            "Mind Sea",
            disciple.attributes.mind,
            SECONDARY,
        );
        draw_stat_seal(
            Rect::new(x + (seal_w + 10.0) * 2.0, y, seal_w, 72.0),
            "Spirit Root",
            disciple.attributes.spirit,
            PRIMARY,
        );
        y += 104.0;

        if let Some(ref injury) = disciple.injury {
            draw_rectangle(
                x,
                y - 18.0,
                right_rect.w - 210.0,
                58.0,
                Color::new(0.45, 0.12, 0.10, 0.34),
            );
            draw_ui_text(
                &format!("Injury: {} {}", injury.severity_str(), injury.injury_type),
                x + 12.0,
                y,
                FONT_BODY_SIZE,
                FAILURE,
            );
            let heal_rate =
                crate::data::disciples::Injury::get_recovery_rate(disciple.attributes.body);
            let recovery_secs = injury.recovery_ticks_remaining / heal_rate.max(1);
            draw_ui_text(
                &format!("Recovery omen: about {} seconds", recovery_secs),
                x + 12.0,
                y + 22.0,
                FONT_SMALL_SIZE,
                WARNING,
            );
            y += 76.0;
        }

        let law_name = disciple
            .law_id
            .as_ref()
            .and_then(|id| data.laws.get(id))
            .map(|l| l.name.as_str())
            .unwrap_or("No inherited law");
        draw_ui_text(
            &format!("Cultivation Law: {}", law_name),
            x,
            y,
            FONT_BODY_SIZE,
            TEXT_PRIMARY,
        );
        y += 24.0;

        draw_ui_text(
            &format!("Talent: {:?}", disciple.talent),
            x,
            y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        y += 22.0;

        let equipped = disciple.equipment.len();
        draw_ui_text(
            &format!("Artifacts Bound: {} / 10", equipped),
            x,
            y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        y += 22.0;

        if let Some(bloodline_text) = bloodline_summary(data, disciple) {
            draw_ui_text(&bloodline_text, x, y, FONT_SMALL_SIZE, WARNING);
            y += 22.0;
        }

        if !disciple.fate_traits.is_empty() {
            let traits_str = disciple
                .fate_traits
                .iter()
                .take(3)
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            draw_ui_text(
                &format!("Fate Marks: {}", traits_str),
                x,
                y,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
            y += 22.0;
        }

        if let Some(ref bottleneck) = disciple.breakthrough_bottleneck {
            let desc = bottleneck.description(data);
            draw_rectangle(
                x,
                y - 16.0,
                right_rect.w - 210.0,
                44.0,
                Color::new(0.48, 0.34, 0.10, 0.32),
            );
            draw_ui_text(
                &format!("Bottleneck: {}", desc),
                x + 12.0,
                y + 7.0,
                FONT_SMALL_SIZE,
                WARNING,
            );
        }
    }

    pub(super) fn handle_detail_actions(
        &mut self,
        data: &GameData,
        disciples: &[Disciple],
        idx: usize,
        _inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        let disciple = disciples.get(idx)?;
        let right_x = 350.0;
        let screen_h = screen_height();
        let btn_y = screen_h - 178.0;
        let btn_w = 150.0;
        let btn_h = 36.0;
        let spacing = 10.0;

        if disciple.can_attempt_breakthrough() {
            if disciple.breakthrough_bottleneck.is_some() {
                let desc = disciple
                    .breakthrough_bottleneck
                    .as_ref()
                    .unwrap()
                    .description(data);
                draw_ui_text(
                    &format!("Breakthrough sealed: {}", desc),
                    right_x,
                    btn_y + 24.0,
                    FONT_SMALL_SIZE,
                    WARNING,
                );
            } else {
                let readiness_bonus = (disciple.get_readiness_bonus() * 100.0) as u32;
                if draw_button(
                    Rect::new(right_x, btn_y, btn_w * 2.0 + spacing, btn_h),
                    &format!("Seek Breakthrough (+{}%)", readiness_bonus),
                    false,
                ) {
                    return Some(
                        UpdateResult::new()
                            .with_action(crate::engine::actions::Action::AttemptBreakthrough(idx)),
                    );
                }
            }
        }

        let row2_y = btn_y + btn_h + spacing;
        if draw_button(
            Rect::new(right_x, row2_y, btn_w, btn_h),
            "Administer Pill",
            false,
        ) {
            self.item_modal_open = true;
        }

        if draw_button(
            Rect::new(right_x + btn_w + spacing, row2_y, btn_w, btn_h),
            "Bind Artifact",
            false,
        ) {
            self.equip_modal_open = true;
        }

        let row3_y = row2_y + btn_h + spacing;
        if disciple.rank != DiscipleRank::Outer && disciple.realm != "Mortal" {
            if draw_button(
                Rect::new(right_x, row3_y, btn_w * 2.0 + spacing, btn_h),
                "Assign Cultivation Law",
                false,
            ) {
                self.law_modal_open = true;
            }
        }

        let row4_y = row3_y + btn_h + spacing;
        if disciple.rank == DiscipleRank::Outer && disciple.realm != "Mortal" {
            if draw_button(
                Rect::new(right_x, row4_y, btn_w * 2.0 + spacing, btn_h),
                "Raise to Inner Court (100 SS)",
                false,
            ) {
                return Some(
                    UpdateResult::new()
                        .with_action(crate::engine::actions::Action::PromoteDisciple(idx)),
                );
            }
        } else if disciple.rank == DiscipleRank::Outer && disciple.realm == "Mortal" {
            draw_ui_text(
                "Promotion awaits the first trace of Qi.",
                right_x,
                row4_y + 20.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }

        None
    }
}

fn rank_title(rank: &DiscipleRank) -> &'static str {
    match rank {
        DiscipleRank::Outer => "Outer Disciple",
        DiscipleRank::Inner => "Inner Disciple",
        DiscipleRank::Elder => "Sect Elder",
        DiscipleRank::SectLeader => "Patriarch",
    }
}

fn bloodline_summary(data: &GameData, disciple: &Disciple) -> Option<String> {
    let bloodline_id = disciple.bloodline.bloodline_id.as_ref()?;
    let bloodline = data.bloodlines.get(bloodline_id)?;
    Some(format!(
        "Bloodline: {} ({:.0}% awakened)",
        bloodline.name,
        disciple.bloodline.awakening_progress * 100.0
    ))
}
