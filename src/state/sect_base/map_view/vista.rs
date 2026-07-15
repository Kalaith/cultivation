use super::*;

impl SectBaseState {
    pub(super) fn draw_mountain_vista(&self, rect: Rect) {
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

    pub(super) fn draw_spirit_terraces(&self, rect: Rect) {
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
}
