use super::*;

impl SectBaseState {
    pub(super) fn draw_construction_modal(
        &mut self,
        data: &GameData,
        unlocked_techs: &[String],
    ) -> Option<UpdateResult> {
        let (screen_w, screen_h) = (screen_width(), screen_height());
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

        let modal_w = 400.0;
        let modal_h = 500.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(
            Rect::new(modal_x, modal_y, modal_w, modal_h),
            Some("Construction Blueprints"),
        );

        if draw_button(
            Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
            "Close",
            false,
        ) {
            self.crafting_modal_open = false;
        }

        let content_y = modal_y + 50.0;
        let content_h = modal_h - 70.0;
        let content_rect = Rect::new(modal_x + 10.0, content_y, modal_w - 20.0, content_h);

        let mut build_opts: Vec<_> = data.building_definitions.values().collect();
        build_opts.sort_by_key(|a| a.cost);

        let available_defs: Vec<_> = build_opts
            .into_iter()
            .filter(|def| {
                let req_tech = def.tech_required.clone().unwrap_or_default();
                let tech_unlocked = unlocked_techs.contains(&req_tech) || req_tech.is_empty();
                let already_built = def.unique
                    && data
                        .buildings
                        .iter()
                        .any(|b| b.building_type == def.building_type);
                tech_unlocked && !already_built
            })
            .collect();

        let item_height = 70.0;
        let total_height = available_defs.len() as f32 * item_height;
        if content_rect.contains(mouse_position().into()) {
            let wheel = mouse_wheel().1;
            if total_height > content_h {
                self.construction_scroll -= wheel * 30.0;
                self.construction_scroll = self
                    .construction_scroll
                    .clamp(0.0, (total_height - content_h).max(0.0));
            } else {
                self.construction_scroll = 0.0;
            }
        }

        if available_defs.is_empty() {
            draw_text(
                "No available buildings.",
                modal_x + 20.0,
                content_y + 20.0,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            return None;
        }

        let mut b_y = content_y - self.construction_scroll;
        for def in available_defs {
            if b_y + 40.0 < content_y {
                b_y += item_height;
                continue;
            }
            if b_y > content_y + content_h {
                break;
            }

            if draw_button(
                Rect::new(modal_x + 20.0, b_y, modal_w - 40.0, 40.0),
                &format!("{} ({} SS)", def.name, def.cost),
                false,
            ) {
                self.crafting_modal_open = false;
                self.placement_mode = Some(def.building_type.clone());
            }

            let desc_y = b_y + 50.0;
            if desc_y < content_y + content_h {
                let desc = if def.description.len() > 50 {
                    format!("{}...", &def.description[..47])
                } else {
                    def.description.clone()
                };
                draw_text(
                    &desc,
                    modal_x + 25.0,
                    desc_y,
                    FONT_SMALL_SIZE,
                    TEXT_SECONDARY,
                );
            }

            b_y += item_height;
        }

        if total_height > content_h {
            let track_x = modal_x + modal_w - 12.0;
            let track_y = content_y;
            let track_h = content_h;
            draw_rectangle(track_x, track_y, 4.0, track_h, PANEL_BORDER);

            let handle_h = (content_h * content_h / total_height).max(20.0);
            let max_offset = (total_height - content_h).max(1.0);
            let handle_y = track_y + (self.construction_scroll / max_offset) * (track_h - handle_h);
            draw_rectangle(track_x - 1.0, handle_y, 6.0, handle_h, TEXT_HIGHLIGHT);

            draw_text(
                "Scroll",
                modal_x + 20.0,
                modal_y + modal_h - 15.0,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }
        None
    }

    pub(super) fn draw_crafting_modal(
        &mut self,
        data: &GameData,
        b_type: &BuildingType,
        spirit_stones: u32,
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>,
        disciples: &[Disciple],
        discovered_recipes: &[String],
    ) -> Option<UpdateResult> {
        use crate::engine::crafting::{self, CraftingContext};

        let (screen_w, screen_h) = (screen_width(), screen_height());
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.8));

        let modal_w = 550.0;
        let modal_h = 650.0;
        let modal_x = (screen_w - modal_w) / 2.0;
        let modal_y = (screen_h - modal_h) / 2.0;

        draw_panel(
            Rect::new(modal_x, modal_y, modal_w, modal_h),
            Some("Crafting Menu"),
        );

        if draw_button(
            Rect::new(modal_x + modal_w - 60.0, modal_y + 10.0, 50.0, 30.0),
            "Close",
            false,
        ) {
            self.crafting_modal_open = false;
        }

        let building = data
            .buildings
            .iter()
            .find(|b| b.building_type == *b_type && b.status == BuildingStatus::Active);
        let building_level = building.map(|b| b.level).unwrap_or(1);
        let building_element = building.map(|b| b.element.clone()).unwrap_or_default();

        let assigned_disciple = building
            .and_then(|b| b.assigned_disciple)
            .and_then(|did| disciples.iter().find(|d| d.id == did));
        let disciple_mind = assigned_disciple.map(|d| d.attributes.mind);

        let info_y = modal_y + 45.0;
        draw_text(
            &format!("Building Lv.{}", building_level),
            modal_x + 20.0,
            info_y,
            FONT_SMALL_SIZE,
            TEXT_SECONDARY,
        );
        if let Some(d) = assigned_disciple {
            draw_text(
                &format!("Worker: {} (Mind: {})", d.name, d.attributes.mind),
                modal_x + 150.0,
                info_y,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        } else {
            draw_text(
                "No worker assigned",
                modal_x + 150.0,
                info_y,
                FONT_SMALL_SIZE,
                TEXT_SECONDARY,
            );
        }

        let mut r_y = modal_y + 70.0;
        let recipes: Vec<_> = data
            .recipes
            .iter()
            .filter(|r| r.required_building == *b_type && discovered_recipes.contains(&r.id))
            .collect();

        if recipes.is_empty() {
            draw_text(
                "No discovered recipes for this station.",
                modal_x + 20.0,
                r_y,
                FONT_BODY_SIZE,
                TEXT_SECONDARY,
            );
            return None;
        }

        let ctx = CraftingContext {
            building_level,
            building_element,
            disciple_mind,
        };

        for recipe in recipes {
            if r_y > modal_y + modal_h - 30.0 {
                draw_text("...", modal_x + 20.0, r_y, FONT_SMALL_SIZE, TEXT_SECONDARY);
                break;
            }

            if let Some(res) = self.draw_recipe_entry(
                recipe,
                &ctx,
                modal_x,
                modal_w,
                &mut r_y,
                modal_y + modal_h,
                spirit_stones,
                herbs,
                inventory,
            ) {
                return Some(res);
            }
        }

        None
    }

    fn draw_recipe_entry(
        &self,
        recipe: &crate::data::items::Recipe,
        ctx: &crate::engine::crafting::CraftingContext,
        modal_x: f32,
        modal_w: f32,
        r_y: &mut f32,
        _max_y: f32,
        spirit_stones: u32,
        herbs: u32,
        inventory: &std::collections::HashMap<String, u32>,
    ) -> Option<UpdateResult> {
        use crate::engine::crafting;

        let success_chance = crafting::calculate_success_chance(recipe, ctx);

        let mut can_craft = true;
        let mut ing_parts: Vec<String> = Vec::new();
        for (ing, amt) in recipe.ingredients.iter() {
            let ing: &String = ing;
            let has: u32 = match ing.as_str() {
                "spirit_stones" => spirit_stones,
                "herbs" => herbs,
                _ => *inventory.get(ing).unwrap_or(&0),
            };
            if has < *amt {
                can_craft = false;
            }
            ing_parts.push(format!("{}x {} ({}/{})", amt, ing, has, amt));
        }

        let chance_color = if success_chance > 70 {
            Color::new(0.2, 0.8, 0.2, 1.0)
        } else if success_chance > 40 {
            Color::new(0.9, 0.8, 0.2, 1.0)
        } else {
            Color::new(0.9, 0.3, 0.2, 1.0)
        };

        let label = format!("{} [{}%]", recipe.name, success_chance);

        if draw_button(
            Rect::new(modal_x + 20.0, *r_y, modal_w - 40.0, 35.0),
            &label,
            false,
        ) {
            if can_craft {
                return Some(UpdateResult::new().with_action(Action::CraftItem(recipe.id.clone())));
            }
        }

        draw_text(
            &format!("{}%", success_chance),
            modal_x + modal_w - 70.0,
            *r_y + 22.0,
            FONT_SMALL_SIZE,
            chance_color,
        );

        let ing_text = ing_parts.join(", ");
        let status_text = if can_craft {
            ing_text
        } else {
            format!("MISSING: {}", ing_text)
        };
        let status_color = if can_craft {
            TEXT_SECONDARY
        } else {
            Color::new(0.8, 0.3, 0.3, 1.0)
        };
        draw_text(
            &status_text,
            modal_x + 25.0,
            *r_y + 50.0,
            FONT_SMALL_SIZE,
            status_color,
        );

        *r_y += 65.0;

        None
    }
}
