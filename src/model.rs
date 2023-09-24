use nannou::prelude::{Draw, Rgb};

use crate::colors::Color;
use crate::goti::{Boundary, Goti};
use crate::Render;

pub struct Model {
    bg_color: Color,
    pub goti: Goti,
    pub running: bool,
    pub pointer_angle: f32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            bg_color: Color::SteelBlue,
            goti: Goti::default(),
            running: bool::default(),
            pointer_angle: f32::default(),
        }
    }
}

impl Render for Model {
    fn display(&self, draw: &Draw) {
        draw.background().color(Rgb::from(self.bg_color));
        self.goti.display(draw);
    }

    fn update(&mut self, boundary: Boundary) {
        self.goti.update(boundary);
    }
}
