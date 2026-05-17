use super::*;

impl SectBaseState {
    pub(super) fn draw_map_view(
        &mut self,
        rect: Rect,
        data: &mut GameData,
        _grid: &mut Grid,
        textures: &TextureManager,
        _spirit_stones: u32,
        _unlocked_techs: &[String],
    ) -> Option<UpdateResult> {
        draw_panel(rect, Some("Sect Map"));
        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            2.5,
            Color::new(TEXT_HIGHLIGHT.r, TEXT_HIGHLIGHT.g, TEXT_HIGHLIGHT.b, 0.35),
        );

        let mouse = vec2(mouse_position().0, mouse_position().1);

        if rect.contains(mouse.into()) {
            let wheel = mouse_wheel().1;
            if wheel.abs() > 0.0 {
                let prev_zoom = self.map_zoom;
                self.map_zoom = (self.map_zoom * (1.0 + wheel * 0.1)).clamp(0.2, 2.0);
                let world_before = self.screen_to_world_with_zoom(rect, mouse, prev_zoom);
                let world_after = self.screen_to_world(rect, mouse);
                self.map_center += world_before - world_after;
            }
        }

        self.handle_map_drag(rect, mouse);
        self.clamp_map_center(rect);
        self.draw_map_base(rect, textures);
        self.draw_placement_preview(rect, 0.0, 0.0);

        if let Some(res) = self.handle_placement_click(rect, textures, data, mouse) {
            return Some(res);
        }

        let hovered_building_id = self.draw_buildings_and_detect_hover(rect, data, textures, mouse);

        if let Some(id) = hovered_building_id {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.view = SectView::BuildingDetails(id);
            }
        }

        if self.crafting_modal_open {
            return self.draw_construction_modal(data, _unlocked_techs);
        }

        None
    }

    fn handle_map_drag(&mut self, rect: Rect, mouse: Vec2) {
        let drag_active = is_mouse_button_down(MouseButton::Middle)
            || (is_key_down(KeyCode::Space) && is_mouse_button_down(MouseButton::Left));

        if drag_active && rect.contains(mouse.into()) {
            if !self.map_dragging {
                self.map_dragging = true;
                self.map_last_mouse = mouse;
            } else {
                let delta = mouse - self.map_last_mouse;
                self.map_center -= delta / self.map_zoom;
                self.map_last_mouse = mouse;
            }
        } else {
            self.map_dragging = false;
        }
    }

    fn draw_map_base(&self, rect: Rect, textures: &TextureManager) {
        if let Some(tex) = textures.get("sect_map_base") {
            let top_left = self.world_to_screen(rect, vec2(0.0, 0.0));
            draw_texture_ex(
                tex,
                top_left.x,
                top_left.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        SECT_MAP_WIDTH * self.map_zoom,
                        SECT_MAP_HEIGHT * self.map_zoom,
                    )),
                    ..Default::default()
                },
            );
        } else {
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                Color::new(0.12, 0.11, 0.10, 1.0),
            );
        }
    }

    fn handle_placement_click(
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

    fn draw_buildings_and_detect_hover(
        &mut self,
        rect: Rect,
        data: &GameData,
        textures: &TextureManager,
        mouse: Vec2,
    ) -> Option<u64> {
        let mut hovered_building_id: Option<u64> = None;
        let mouse_world = self.screen_to_world(rect, mouse);

        for building in data.buildings.iter() {
            let world_pos = vec2(building.x as f32, building.y as f32);
            let tint = if building.status == BuildingStatus::Ruined {
                Color::new(0.6, 0.6, 0.6, 1.0)
            } else {
                WHITE
            };
            self.draw_building_sprite(rect, textures, &building.building_type, world_pos, tint);

            if self.placement_mode.is_none() && rect.contains(mouse.into()) {
                if self.point_hits_building(textures, building, mouse_world) {
                    hovered_building_id = Some(building.id);
                }
            }
        }

        hovered_building_id
    }

    pub(super) fn draw_placement_preview(&mut self, rect: Rect, _map_x: f32, _map_y: f32) {
        if let Some(place_type) = &self.placement_mode {
            draw_text(
                &format!(
                    "Placing: {:?} (Click to Build, RMB/Esc to Cancel)",
                    place_type
                ),
                rect.x + 20.0,
                rect.y + rect.h - 40.0,
                FONT_HEADER_SIZE,
                TEXT_HIGHLIGHT,
            );
        }
    }

    pub(super) fn world_to_screen(&self, rect: Rect, world: Vec2) -> Vec2 {
        let view_center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        view_center + (world - self.map_center) * self.map_zoom
    }

    pub(super) fn screen_to_world(&self, rect: Rect, screen: Vec2) -> Vec2 {
        self.screen_to_world_with_zoom(rect, screen, self.map_zoom)
    }

    pub(super) fn screen_to_world_with_zoom(&self, rect: Rect, screen: Vec2, zoom: f32) -> Vec2 {
        let view_center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
        self.map_center + (screen - view_center) / zoom
    }

    pub(super) fn clamp_map_center(&mut self, rect: Rect) {
        let half_view = vec2(
            rect.w / (2.0 * self.map_zoom),
            rect.h / (2.0 * self.map_zoom),
        );
        let min_x = half_view.x;
        let min_y = half_view.y;
        let max_x = SECT_MAP_WIDTH - half_view.x;
        let max_y = SECT_MAP_HEIGHT - half_view.y;

        if max_x < min_x {
            self.map_center.x = SECT_MAP_WIDTH * 0.5;
        } else {
            self.map_center.x = self.map_center.x.clamp(min_x, max_x);
        }

        if max_y < min_y {
            self.map_center.y = SECT_MAP_HEIGHT * 0.5;
        } else {
            self.map_center.y = self.map_center.y.clamp(min_y, max_y);
        }
    }

    pub(super) fn building_texture_key(&self, b_type: &BuildingType) -> Option<&'static str> {
        match b_type {
            BuildingType::SectHall => Some("bld_sect_hall"),
            BuildingType::Dormitory => Some("bld_dormitory"),
            BuildingType::TrainingYard => Some("bld_training_yard"),
            BuildingType::LibraryPavilion => Some("bld_library"),
            BuildingType::MissionBoard => Some("bld_mission_board"),
            BuildingType::SpiritGarden => Some("bld_spirit_garden"),
            BuildingType::AlchemyFurnace => Some("bld_alchemy"),
            BuildingType::ArtifactForge => Some("bld_forge"),
            BuildingType::Blacksmith => Some("bld_forge"),
            BuildingType::HerbGarden => Some("bld_herb_garden"),
            BuildingType::Greenhouse => Some("bld_greenhouse"),
            BuildingType::DryingPavilion => Some("bld_drying"),
            BuildingType::HerbStorage => Some("bld_storage"),
            BuildingType::TalismanScriptorium => Some("bld_scriptorium"),
            BuildingType::Decoration => None,
        }
    }

    pub(super) fn building_sprite_metrics(
        &mut self,
        textures: &TextureManager,
        b_type: &BuildingType,
        world_pos: Vec2,
    ) -> Option<(String, Vec2, Vec2, Vec2)> {
        let key = self.building_texture_key(b_type)?.to_string();
        let tex = textures.get(&key)?;
        let tex_size = vec2(tex.width(), tex.height());
        let size = tex_size * BUILDING_SCALE;
        let top_left = world_pos - vec2(size.x * 0.5, size.y);
        Some((key, top_left, size, tex_size))
    }

    pub(super) fn draw_building_sprite(
        &mut self,
        rect: Rect,
        textures: &TextureManager,
        b_type: &BuildingType,
        world_pos: Vec2,
        tint: Color,
    ) {
        if let Some((key, top_left, size, _tex_size)) =
            self.building_sprite_metrics(textures, b_type, world_pos)
        {
            if let Some(tex) = textures.get(&key) {
                let screen_pos = self.world_to_screen(rect, top_left);
                let draw_size = size * self.map_zoom;
                draw_texture_ex(
                    tex,
                    screen_pos.x,
                    screen_pos.y,
                    tint,
                    DrawTextureParams {
                        dest_size: Some(draw_size),
                        ..Default::default()
                    },
                );
                return;
            }
        }

        let screen_pos = self.world_to_screen(rect, world_pos);
        draw_brush_stroke(
            screen_pos.x - 12.0,
            screen_pos.y - 12.0,
            24.0,
            24.0,
            tint,
            1.0,
        );
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

    pub(super) fn can_place_building(
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
