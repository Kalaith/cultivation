use super::*;
use macroquad_toolkit::ui::draw_ui_text;

impl SectBaseState {
    pub(super) fn draw_map_view(
        &mut self,
        rect: Rect,
        data: &mut GameData,
        _grid: &mut Grid,
        textures: &TextureManager,
        spirit_stones: u32,
        _unlocked_techs: &[String],
    ) -> Option<UpdateResult> {
        self.draw_mountain_vista(rect);
        let map_rect = self.sect_map_view_rect(rect);
        draw_rectangle(
            map_rect.x,
            map_rect.y,
            map_rect.w,
            map_rect.h,
            Color::new(0.04, 0.035, 0.025, 0.20),
        );
        draw_rectangle_lines(
            map_rect.x,
            map_rect.y,
            map_rect.w,
            map_rect.h,
            1.5,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.24),
        );
        draw_ui_text(
            "Mist gathers around the ruined halls of your mountain refuge.",
            map_rect.x + 18.0,
            map_rect.y + map_rect.h - 16.0,
            FONT_SMALL_SIZE,
            Color::new(TEXT_SECONDARY.r, TEXT_SECONDARY.g, TEXT_SECONDARY.b, 0.72),
        );

        let mouse = vec2(mouse_position().0, mouse_position().1);

        if map_rect.contains(mouse.into()) {
            let wheel = mouse_wheel().1;
            if wheel.abs() > 0.0 {
                let prev_zoom = self.map_zoom;
                self.map_zoom = (self.map_zoom * (1.0 + wheel * 0.1)).clamp(0.2, 2.0);
                let world_before = self.screen_to_world_with_zoom(map_rect, mouse, prev_zoom);
                let world_after = self.screen_to_world(map_rect, mouse);
                self.map_center += world_before - world_after;
            }
        }

        self.handle_map_drag(map_rect, mouse);
        self.clamp_map_center(map_rect);
        self.draw_map_base(map_rect, textures);
        self.draw_spirit_terraces(map_rect);
        self.draw_placement_preview(map_rect, 0.0, 0.0);

        if let Some(res) = self.handle_placement_click(map_rect, textures, data, mouse) {
            return Some(res);
        }

        let hovered_building_id =
            self.draw_buildings_and_detect_hover(map_rect, data, textures, mouse);

        if let Some(id) = hovered_building_id {
            if is_mouse_button_pressed(MouseButton::Left) {
                self.view = SectView::BuildingDetails(id);
            }
        }

        if self.crafting_modal_open {
            return self.draw_construction_modal(data, _unlocked_techs, spirit_stones);
        }

        None
    }

    fn sect_map_view_rect(&self, rect: Rect) -> Rect {
        let side_clearance = 260.0;
        let right_clearance = 300.0;
        Rect::new(
            rect.x + side_clearance,
            rect.y + 8.0,
            (rect.w - side_clearance - right_clearance).max(420.0),
            rect.h - 16.0,
        )
    }

    fn draw_mountain_vista(&self, rect: Rect) {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.13, 0.18, 0.19, 1.0),
        );

        self.draw_sky_band(rect);
        self.draw_mountain_layer(
            rect,
            0.18,
            Color::new(0.42, 0.54, 0.54, 0.32),
            Color::new(0.18, 0.24, 0.24, 0.24),
        );
        self.draw_mountain_layer(
            rect,
            0.34,
            Color::new(0.23, 0.29, 0.30, 0.50),
            Color::new(0.09, 0.11, 0.12, 0.42),
        );
        self.draw_side_cliffs(rect);
        self.draw_waterfall_axis(rect);
        self.draw_cloud_bank(rect, rect.y + rect.h * 0.22, 0.34);
        self.draw_cloud_bank(rect, rect.y + rect.h * 0.72, 0.46);
        self.draw_celestial_seal(rect);
    }

    fn draw_sky_band(&self, rect: Rect) {
        for i in 0..8 {
            let t = i as f32 / 8.0;
            draw_rectangle(
                rect.x,
                rect.y + rect.h * t,
                rect.w,
                rect.h / 8.0 + 1.0,
                Color::new(0.20 - t * 0.08, 0.30 - t * 0.12, 0.34 - t * 0.13, 0.48),
            );
        }
        draw_circle(
            rect.x + rect.w * 0.72,
            rect.y + rect.h * 0.16,
            44.0,
            Color::new(0.95, 0.76, 0.42, 0.18),
        );
    }

    fn draw_mountain_layer(&self, rect: Rect, y_factor: f32, ridge: Color, shade: Color) {
        let base_y = rect.y + rect.h * (y_factor + 0.36);
        let peaks: [(f32, f32); 8] = [
            (0.02, 0.42),
            (0.14, 0.18),
            (0.28, 0.38),
            (0.43, 0.12),
            (0.58, 0.32),
            (0.73, 0.16),
            (0.90, 0.36),
            (1.04, 0.22),
        ];

        for window in peaks.windows(2) {
            let (x1, h1) = window[0];
            let (x2, h2) = window[1];
            let peak_x = rect.x + rect.w * ((x1 + x2) * 0.5);
            let peak_y = rect.y + rect.h * (y_factor + h1.min(h2) * 0.18);
            draw_triangle(
                vec2(rect.x + rect.w * x1, base_y),
                vec2(peak_x, peak_y),
                vec2(rect.x + rect.w * x2, base_y),
                ridge,
            );
            draw_triangle(
                vec2(peak_x, peak_y),
                vec2(rect.x + rect.w * x2, base_y),
                vec2(peak_x + rect.w * 0.035, base_y),
                shade,
            );
        }
    }

    fn draw_side_cliffs(&self, rect: Rect) {
        let left = [
            vec2(rect.x, rect.y + rect.h),
            vec2(rect.x, rect.y + rect.h * 0.30),
            vec2(rect.x + rect.w * 0.14, rect.y + rect.h * 0.48),
            vec2(rect.x + rect.w * 0.18, rect.y + rect.h),
        ];
        let right = [
            vec2(rect.x + rect.w, rect.y + rect.h),
            vec2(rect.x + rect.w, rect.y + rect.h * 0.24),
            vec2(rect.x + rect.w * 0.84, rect.y + rect.h * 0.42),
            vec2(rect.x + rect.w * 0.78, rect.y + rect.h),
        ];
        draw_triangle(
            left[0],
            left[1],
            left[2],
            Color::new(0.06, 0.07, 0.07, 0.58),
        );
        draw_triangle(
            left[0],
            left[2],
            left[3],
            Color::new(0.09, 0.10, 0.09, 0.62),
        );
        draw_triangle(
            right[0],
            right[1],
            right[2],
            Color::new(0.05, 0.06, 0.06, 0.62),
        );
        draw_triangle(
            right[0],
            right[2],
            right[3],
            Color::new(0.08, 0.09, 0.08, 0.58),
        );
    }

    fn draw_waterfall_axis(&self, rect: Rect) {
        let center_x = rect.x + rect.w * 0.52;
        for i in 0..6 {
            let offset = (i as f32 - 2.5) * 8.0;
            draw_line(
                center_x + offset,
                rect.y + rect.h * 0.05,
                center_x - rect.w * 0.03 + offset * 0.4,
                rect.y + rect.h * 0.92,
                9.0 - i as f32,
                Color::new(0.70, 0.92, 0.98, 0.045 + i as f32 * 0.012),
            );
        }
        draw_line(
            center_x - 18.0,
            rect.y + rect.h * 0.18,
            center_x - 48.0,
            rect.y + rect.h * 0.82,
            2.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.12),
        );
        draw_line(
            center_x + 22.0,
            rect.y + rect.h * 0.14,
            center_x + 55.0,
            rect.y + rect.h * 0.84,
            2.0,
            Color::new(SECONDARY.r, SECONDARY.g, SECONDARY.b, 0.12),
        );
    }

    fn draw_cloud_bank(&self, rect: Rect, y: f32, alpha: f32) {
        let clouds = [
            (0.08, 32.0),
            (0.18, 44.0),
            (0.31, 28.0),
            (0.66, 36.0),
            (0.79, 52.0),
            (0.93, 34.0),
        ];
        for (x_factor, radius) in clouds {
            draw_circle(
                rect.x + rect.w * x_factor,
                y + (x_factor * 37.0).sin() * 18.0,
                radius,
                Color::new(0.88, 0.88, 0.78, alpha * 0.18),
            );
            draw_circle(
                rect.x + rect.w * x_factor + radius * 0.55,
                y + 18.0,
                radius * 0.72,
                Color::new(0.88, 0.88, 0.78, alpha * 0.12),
            );
        }
    }

    fn draw_celestial_seal(&self, rect: Rect) {
        let center = vec2(rect.x + rect.w * 0.50, rect.y + rect.h * 0.12);
        draw_circle_lines(
            center.x,
            center.y,
            72.0,
            2.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.11),
        );
        draw_circle_lines(
            center.x,
            center.y,
            49.0,
            1.5,
            Color::new(SECONDARY.r, SECONDARY.g, SECONDARY.b, 0.10),
        );
        for i in 0..9 {
            let angle = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / 9.0;
            let p = center + vec2(angle.cos() * 72.0, angle.sin() * 72.0);
            draw_circle(
                p.x,
                p.y,
                3.0,
                Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.16),
            );
        }
    }

    fn draw_spirit_terraces(&self, rect: Rect) {
        let center = vec2(rect.x + rect.w * 0.50, rect.y + rect.h * 0.49);
        for i in 0..5 {
            let w = rect.w * (0.34 + i as f32 * 0.085);
            let h = rect.h * (0.14 + i as f32 * 0.035);
            draw_ellipse_lines(
                center.x,
                center.y + i as f32 * 16.0,
                w,
                h,
                0.0,
                1.0,
                Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.08),
            );
        }
        draw_line(
            rect.x + rect.w * 0.28,
            rect.y + rect.h * 0.58,
            rect.x + rect.w * 0.72,
            rect.y + rect.h * 0.42,
            2.0,
            Color::new(SECONDARY.r, SECONDARY.g, SECONDARY.b, 0.08),
        );
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
                Color::new(1.0, 0.96, 0.84, 0.62),
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
                Color::new(0.68, 0.64, 0.56, 0.92)
            } else {
                WHITE
            };
            self.draw_building_sprite(rect, textures, &building.building_type, world_pos, tint);

            if self.tutorial_step_active.is_some() {
                let screen_pos = self.world_to_screen(rect, world_pos);
                if rect.contains(screen_pos.into()) {
                    let half = 60.0 * self.map_zoom.max(0.5);
                    let target = Rect::new(
                        screen_pos.x - half,
                        screen_pos.y - half * 1.4,
                        half * 2.0,
                        half * 2.0,
                    );
                    match building.building_type {
                        BuildingType::SectHall => {
                            self.register_tutorial_target(0, target);
                            self.register_tutorial_target(1, target);
                            self.register_tutorial_target(3, target);
                        }
                        BuildingType::MissionBoard => {
                            self.register_tutorial_target(4, target);
                        }
                        _ => {}
                    }
                }
            }

            if self.placement_mode.is_none() && rect.contains(mouse.into()) {
                if self.point_hits_building(textures, building, mouse_world) {
                    hovered_building_id = Some(building.id);
                    self.draw_building_focus(rect, building, world_pos);
                }
            }
        }

        hovered_building_id
    }

    fn draw_building_focus(
        &self,
        rect: Rect,
        building: &crate::data::buildings::Building,
        world_pos: Vec2,
    ) {
        let screen_pos = self.world_to_screen(rect, world_pos);
        draw_circle_lines(
            screen_pos.x,
            screen_pos.y - 22.0 * self.map_zoom,
            42.0 * self.map_zoom.max(0.6),
            2.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.72),
        );

        let label = match building.status {
            BuildingStatus::Active => format!("{}", building.building_type),
            BuildingStatus::Ruined => format!("Ruined {}", building.building_type),
            BuildingStatus::Constructing => format!("Raising {}", building.building_type),
        };
        let label_w = 178.0;
        let label_h = 30.0;
        let label_x = screen_pos.x - label_w * 0.5;
        let label_y = screen_pos.y - 82.0 * self.map_zoom.max(0.7);
        draw_rectangle(
            label_x,
            label_y,
            label_w,
            label_h,
            Color::new(0.04, 0.03, 0.02, 0.76),
        );
        draw_rectangle_lines(
            label_x,
            label_y,
            label_w,
            label_h,
            1.0,
            Color::new(PRIMARY.r, PRIMARY.g, PRIMARY.b, 0.58),
        );
        draw_ui_text(
            &label,
            label_x + 10.0,
            label_y + 21.0,
            FONT_SMALL_SIZE,
            TEXT_PRIMARY,
        );
    }

    pub(super) fn draw_placement_preview(&mut self, rect: Rect, _map_x: f32, _map_y: f32) {
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
