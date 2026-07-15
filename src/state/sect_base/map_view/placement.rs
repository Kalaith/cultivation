use super::*;
use macroquad_toolkit::ui::draw_ui_text;

impl SectBaseState {
    pub(super) fn handle_placement_click(
        &mut self,
        rect: Rect,
        textures: &TextureManager,
        data: &GameData,
        mouse: Vec2,
    ) -> Option<UpdateResult> {
        let Some(place_type) = self.placement_mode.clone() else {
            return None;
        };

        if rect.contains(mouse.into()) {
            let world_pos = self.screen_to_world(rect, mouse);
            let can_place = self.can_place_building(textures, data, &place_type, world_pos);
            let tint = if can_place {
                Color::new(0.2, 1.0, 0.2, 0.65)
            } else {
                Color::new(1.0, 0.2, 0.2, 0.65)
            };
            self.draw_building_sprite(rect, textures, &place_type, world_pos, tint);

            if is_mouse_button_pressed(MouseButton::Left) && can_place {
                self.placement_mode = None;
                return Some(UpdateResult::new().with_action(Action::ConstructBuilding(
                    place_type,
                    world_pos.x.round() as i32,
                    world_pos.y.round() as i32,
                )));
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.placement_mode = None;
        }

        None
    }

    pub(super) fn draw_placement_preview(&mut self, rect: Rect) {
        if let Some(place_type) = &self.placement_mode {
            draw_ui_text(
                &format!("Placing: {}", place_type),
                rect.x + 20.0,
                rect.y + rect.h - 40.0,
                FONT_HEADER_SIZE,
                TEXT_HIGHLIGHT,
            );
        }
    }

    fn get_texture_mask<'a>(
        &'a mut self,
        textures: &'a TextureManager,
        key: &str,
    ) -> Option<&'a Image> {
        if !self.building_mask_cache.contains_key(key) {
            if let Some(tex) = textures.get(key) {
                let image = tex.get_texture_data();
                self.building_mask_cache.insert(key.to_string(), image);
            }
        }
        self.building_mask_cache.get(key)
    }

    fn alpha_at(image: &Image, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= image.width as i32 || y >= image.height as i32 {
            return 0;
        }
        let idx = ((y * image.width as i32 + x) * 4) as usize;
        image.bytes.get(idx + 3).copied().unwrap_or(0)
    }

    pub(super) fn point_hits_building(
        &mut self,
        textures: &TextureManager,
        building: &crate::data::buildings::Building,
        world_point: Vec2,
    ) -> bool {
        let world_pos = vec2(building.x as f32, building.y as f32);
        let Some((key, top_left, size, _tex_size)) =
            self.building_sprite_metrics(textures, &building.building_type, world_pos)
        else {
            return false;
        };

        let bottom_right = top_left + size;
        if world_point.x < top_left.x
            || world_point.y < top_left.y
            || world_point.x > bottom_right.x
            || world_point.y > bottom_right.y
        {
            return false;
        }

        let Some(mask) = self.get_texture_mask(textures, &key) else {
            return true;
        };

        let local = (world_point - top_left) / BUILDING_SCALE;
        let alpha = Self::alpha_at(mask, local.x.floor() as i32, local.y.floor() as i32);
        alpha > ALPHA_THRESHOLD
    }

    fn can_place_building(
        &mut self,
        textures: &TextureManager,
        data: &GameData,
        b_type: &BuildingType,
        world_pos: Vec2,
    ) -> bool {
        let Some((key, top_left, size, _tex_size)) =
            self.building_sprite_metrics(textures, b_type, world_pos)
        else {
            return false;
        };

        let bottom_right = top_left + size;
        if top_left.x < 0.0
            || top_left.y < 0.0
            || bottom_right.x > SECT_MAP_WIDTH
            || bottom_right.y > SECT_MAP_HEIGHT
        {
            return false;
        }

        for existing in &data.buildings {
            let existing_pos = vec2(existing.x as f32, existing.y as f32);
            if self.sprites_overlap(textures, &key, top_left, size, existing, existing_pos) {
                return false;
            }
        }

        true
    }

    fn sprites_overlap(
        &mut self,
        textures: &TextureManager,
        candidate_key: &str,
        candidate_top_left: Vec2,
        candidate_size: Vec2,
        existing: &crate::data::buildings::Building,
        existing_pos: Vec2,
    ) -> bool {
        let Some((existing_key, existing_top_left, existing_size, _existing_tex_size)) =
            self.building_sprite_metrics(textures, &existing.building_type, existing_pos)
        else {
            return self.check_bounding_box_overlap(
                candidate_top_left,
                candidate_size,
                existing_pos,
            );
        };

        let a_max = candidate_top_left + candidate_size;
        let b_max = existing_top_left + existing_size;

        let inter_left = candidate_top_left.x.max(existing_top_left.x).floor() as i32;
        let inter_top = candidate_top_left.y.max(existing_top_left.y).floor() as i32;
        let inter_right = a_max.x.min(b_max.x).ceil() as i32;
        let inter_bottom = a_max.y.min(b_max.y).ceil() as i32;

        if inter_right <= inter_left || inter_bottom <= inter_top {
            return false;
        }

        let candidate_mask = self.get_texture_mask(textures, candidate_key).cloned();
        let existing_mask = self.get_texture_mask(textures, &existing_key).cloned();
        let Some(candidate_mask) = candidate_mask else {
            return true;
        };
        let Some(existing_mask) = existing_mask else {
            return true;
        };

        self.check_pixel_overlap(
            &candidate_mask,
            candidate_top_left,
            &existing_mask,
            existing_top_left,
            inter_left,
            inter_top,
            inter_right,
            inter_bottom,
        )
    }

    fn check_bounding_box_overlap(
        &self,
        candidate_top_left: Vec2,
        candidate_size: Vec2,
        existing_pos: Vec2,
    ) -> bool {
        let a_max = candidate_top_left + candidate_size;
        let fallback_size = vec2(64.0, 64.0);
        let fallback_top_left = existing_pos - vec2(fallback_size.x * 0.5, fallback_size.y);
        let b_max = fallback_top_left + fallback_size;
        candidate_top_left.x < b_max.x
            && a_max.x > fallback_top_left.x
            && candidate_top_left.y < b_max.y
            && a_max.y > fallback_top_left.y
    }

    #[allow(clippy::too_many_arguments)]
    fn check_pixel_overlap(
        &self,
        candidate_mask: &Image,
        candidate_top_left: Vec2,
        existing_mask: &Image,
        existing_top_left: Vec2,
        inter_left: i32,
        inter_top: i32,
        inter_right: i32,
        inter_bottom: i32,
    ) -> bool {
        for y in inter_top..inter_bottom {
            for x in inter_left..inter_right {
                let world = vec2(x as f32 + 0.5, y as f32 + 0.5);
                let cand_local = (world - candidate_top_left) / BUILDING_SCALE;
                let cand_alpha = Self::alpha_at(
                    candidate_mask,
                    cand_local.x.floor() as i32,
                    cand_local.y.floor() as i32,
                );
                if cand_alpha <= ALPHA_THRESHOLD {
                    continue;
                }
                let ex_local = (world - existing_top_left) / BUILDING_SCALE;
                let ex_alpha = Self::alpha_at(
                    existing_mask,
                    ex_local.x.floor() as i32,
                    ex_local.y.floor() as i32,
                );
                if ex_alpha > ALPHA_THRESHOLD {
                    return true;
                }
            }
        }
        false
    }
}
