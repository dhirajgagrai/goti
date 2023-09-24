use nannou::{geom::Rect, prelude::Rgb, Draw};
use std::{thread, time};

use crate::colors::Color;
use crate::Render;

const DIAMETER: f32 = 10.0;
const RADIUS: f32 = DIAMETER / 2.0;
const SPEED: f32 = 6.9;

enum HMotion {
    Left,
    Right,
}

enum VMotion {
    Up,
    Down,
}

#[derive(Default)]
struct Position {
    x: f32,
    y: f32,
}

pub struct Boundary {
    pub x_p: f32,
    pub x_n: f32,
    pub y_p: f32,
    pub y_n: f32,
}

impl From<Rect> for Boundary {
    fn from(r: Rect) -> Self {
        Boundary {
            x_p: r.right() - RADIUS,
            x_n: r.left() + RADIUS,
            y_p: r.top() - RADIUS,
            y_n: r.bottom() + RADIUS,
        }
    }
}

pub struct Goti {
    color: Color,
    diameter: f32,
    speed: f32,
    h_motion: Option<HMotion>,
    v_motion: Option<VMotion>,
    position: Position,
    pub launch_angle: Option<f32>,
}

impl Goti {
    fn edge_collison(&mut self, bound: Boundary) {
        match self.h_motion {
            Some(HMotion::Left) => {
                if self.position.x <= bound.x_n {
                    self.h_motion = Some(HMotion::Right);
                }
            }
            Some(HMotion::Right) => {
                if self.position.x >= bound.x_p {
                    self.h_motion = Some(HMotion::Left);
                }
            }
            None => {}
        }
        match self.v_motion {
            Some(VMotion::Up) => {
                if self.position.y >= bound.y_p {
                    self.v_motion = Some(VMotion::Down);
                }
            }
            Some(VMotion::Down) => {
                if self.position.y <= bound.y_n {
                    self.v_motion = Some(VMotion::Up);
                }
            }
            None => {}
        }
    }

    fn calc_h_v_speed(&mut self) -> (f32, f32) {
        let tan_value = self
            .launch_angle
            .unwrap_or_default()
            .to_radians()
            .tan()
            .abs();
        let slow_x_speed = ((1.0 / tan_value) * self.speed, 1.0 * self.speed);
        let slow_y_speed = (1.0 * self.speed, tan_value * self.speed);
        match self.launch_angle.unwrap_or_default() {
            a if a >= 0.0 && a <= 45.0 => slow_y_speed,
            a if a > 45.0 && a <= 90.0 => slow_x_speed,
            a if a > 90.0 && a <= 135.0 => slow_x_speed,
            a if a > 135.0 && a <= 180.0 => slow_y_speed,
            a if a > -180.0 && a <= -135.0 => slow_y_speed,
            a if a > -135.0 && a <= -90.0 => slow_x_speed,
            a if a > -90.0 && a <= -45.0 => slow_x_speed,
            a if a > -45.0 && a < 0.0 => slow_y_speed,
            _ => (0.0, 0.0),
        }
    }
}

impl Default for Goti {
    fn default() -> Self {
        Self {
            color: Color::Honeydew,
            diameter: DIAMETER,
            speed: SPEED,
            h_motion: None,
            v_motion: None,
            position: Position::default(),
            launch_angle: None,
        }
    }
}

impl Render for Goti {
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .w(self.diameter)
            .h(self.diameter)
            .x_y(self.position.x, self.position.y)
            .color(Rgb::from(self.color));
    }

    fn update(&mut self, b: Boundary) {
        let (h_speed, v_speed) = self.calc_h_v_speed();

        thread::sleep(time::Duration::from_micros(9999));

        match self.h_motion {
            Some(HMotion::Right) => {
                if self.position.x < b.x_p {
                    self.position.x += h_speed;
                }
            }
            Some(HMotion::Left) => {
                if self.position.x > b.x_n {
                    self.position.x -= h_speed;
                }
            }
            None => {
                self.h_motion = match self.launch_angle {
                    Some(a) => match a {
                        a if a >= 0.0 && a <= 90.0 => Some(HMotion::Right),
                        a if a > 90.0 && a <= 180.0 => Some(HMotion::Left),
                        a if a > -180.0 && a <= -90.0 => Some(HMotion::Left),
                        a if a > -90.0 && a < 0.0 => Some(HMotion::Right),
                        _ => None,
                    },
                    None => None,
                }
            }
        }

        match self.v_motion {
            Some(VMotion::Up) => {
                if self.position.y < b.y_p {
                    self.position.y += v_speed;
                }
            }
            Some(VMotion::Down) => {
                if self.position.y > b.y_n {
                    self.position.y -= v_speed;
                }
            }
            None => {
                self.v_motion = match self.launch_angle {
                    Some(a) => match a {
                        a if a >= 0.0 && a <= 90.0 => Some(VMotion::Up),
                        a if a > 90.0 && a <= 180.0 => Some(VMotion::Up),
                        a if a > -180.0 && a <= -90.0 => Some(VMotion::Down),
                        a if a > -90.0 && a < 0.0 => Some(VMotion::Down),
                        _ => None,
                    },
                    None => None,
                }
            }
        }

        self.edge_collison(b);
    }
}
