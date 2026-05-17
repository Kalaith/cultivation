use super::*;

impl SectBaseState {
    pub(super) fn draw_tutorial_overlay(
        &self,
        screen_w: f32,
        screen_h: f32,
        tutorial: &mut TutorialState,
    ) {
        let total_steps = 5;
        let (title, body) = self.get_tutorial_step_text(tutorial.step);
        let header = format!(
            "Tutorial {}/{}",
            (tutorial.step + 1).min(total_steps),
            total_steps
        );

        let overlay_w = 700.0;
        let overlay_h = 220.0;
        let overlay_x = (screen_w - overlay_w) / 2.0;
        let overlay_y = screen_h - overlay_h - 10.0;
        let rect = Rect::new(overlay_x, overlay_y, overlay_w, overlay_h);

        draw_panel(rect, Some(&header));

        let portrait_w = 110.0;
        let portrait_h = 140.0;
        let portrait_y = rect.y + 50.0;
        let left_portrait = Rect::new(rect.x + 10.0, portrait_y, portrait_w, portrait_h);
        let right_portrait = Rect::new(
            rect.x + rect.w - portrait_w - 10.0,
            portrait_y,
            portrait_w,
            portrait_h,
        );
        draw_panel(left_portrait, Some("Portrait"));
        draw_panel(right_portrait, Some("Portrait"));

        let text_x = rect.x + portrait_w + 25.0;
        let text_w = rect.w - (portrait_w * 2.0) - 50.0;

        draw_text(
            title,
            text_x,
            rect.y + 60.0,
            FONT_HEADER_SIZE,
            TEXT_HIGHLIGHT,
        );

        let mut text_y = rect.y + 85.0;
        let words: Vec<&str> = body.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };

            if test_line.len() as f32 * 7.0 > text_w {
                draw_text(
                    &current_line,
                    text_x,
                    text_y,
                    FONT_SMALL_SIZE,
                    TEXT_SECONDARY,
                );
                text_y += 18.0;
                current_line = word.to_string();
            } else {
                current_line = test_line;
            }
        }
        if !current_line.is_empty() {
            draw_text(
                &current_line,
                text_x,
                text_y,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }

        let btn_y = rect.y + rect.h - 40.0;
        if draw_button(
            Rect::new(rect.x + rect.w - 190.0, btn_y, 80.0, 30.0),
            "Hide",
            false,
        ) {
            tutorial.hidden = true;
        }
        if draw_button(
            Rect::new(rect.x + rect.w - 100.0, btn_y, 80.0, 30.0),
            "Skip",
            false,
        ) {
            tutorial.active = false;
        }
    }

    pub(super) fn update_tutorial_progress(
        &self,
        tutorial: &mut TutorialState,
        data: &GameData,
        unlocked_techs: &[String],
        disciples: &[Disciple],
        ongoing_missions: &[OngoingMission],
        completed_history: &[String],
    ) {
        if !tutorial.active {
            return;
        }

        let total_steps = 5;
        while tutorial.step < total_steps
            && self.is_tutorial_step_complete(
                tutorial.step,
                data,
                unlocked_techs,
                disciples,
                ongoing_missions,
                completed_history,
            )
        {
            tutorial.step += 1;
        }

        if tutorial.step >= total_steps {
            tutorial.active = false;
        }
    }

    fn is_tutorial_step_complete(
        &self,
        step: usize,
        data: &GameData,
        unlocked_techs: &[String],
        disciples: &[Disciple],
        ongoing_missions: &[OngoingMission],
        completed_history: &[String],
    ) -> bool {
        match step {
            0 => data.buildings.iter().any(|b| {
                b.building_type == BuildingType::SectHall && b.status == BuildingStatus::Active
            }),
            1 => unlocked_techs.iter().any(|t| t == "sect_administration"),
            2 => data
                .buildings
                .iter()
                .any(|b| b.building_type == BuildingType::MissionBoard),
            3 => disciples.len() > 1,
            4 => !ongoing_missions.is_empty() || !completed_history.is_empty(),
            _ => true,
        }
    }

    fn get_tutorial_step_text(&self, step: usize) -> (&'static str, &'static str) {
        match step {
            0 => (
                "Restore the Sect Hall",
                "Select the ruined Sect Hall and choose Restore (50 SS).",
            ),
            1 => (
                "Unlock the Mission Board",
                "Open Research / Tech and learn Sect Administration (0 SS).",
            ),
            2 => (
                "Build the Mission Board",
                "Open Construction and place a Mission Board on the map.",
            ),
            3 => (
                "Recruit a Disciple",
                "Select the Sect Hall and press Recruit to bring in help.",
            ),
            4 => (
                "Send a Mission",
                "Open the Mission Board, assign a team, and depart on a mission.",
            ),
            _ => ("Tutorial Complete", "All steps finished."),
        }
    }
}
