use super::*;
use macroquad_toolkit::ui::draw_ui_text;

impl SectBaseState {
    pub(super) fn draw_buildings_and_detect_hover(
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

    fn building_texture_key(&self, b_type: &BuildingType) -> Option<&'static str> {
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
}
