use egui::{
    Vec2,
    vec2,
    Pos2,
    Painter,
    Color32,
};

use crate::consts::{
    FRICTION,
    MASS,
    RADIUS,
    WHITE_STROKE,
};

use std::f32::consts::PI;


pub struct Head {
    pub position: Vec2,
    pub angle:    f32,
    pub speed:    f32,
    pub force:    f32,
}

impl Head {

    pub fn default() -> Self {
        Self { 
            position: vec2(130.0, 200.0),
            angle:    0.0,
            speed:    2.0,
            force:    0.0,
        }
    }

    pub fn inc_angle(&mut self, increment: f32) {
        self.angle += increment;
    }

    pub fn tick(&mut self) {
        let mut vx = self.angle.cos() * self.speed;
        let mut vy = self.angle.sin() * self.speed;
        vx += (self.angle.cos() * self.force - FRICTION * vx) / MASS;
        vy += (self.angle.sin() * self.force - FRICTION * vy) / MASS;
        self.speed = (vx*vx + vy*vy).sqrt();
        self.position.x += vx;
        self.position.y += vy;
    }

    pub fn bounce(&mut self, canvas_size: Vec2) {
        if self.position.x <= 0.0 || self.position.x > canvas_size.x {
            self.angle = PI - self.angle;
        } else if self.position.y <= 0.0 || self.position.y > canvas_size.y {
            self.angle = 2.0*PI - self.angle;
        }
    }

    pub fn paint(&self, painter: &Painter) {
        let center = self.position.to_pos2();
        painter.circle( center, RADIUS, Color32::DARK_GREEN, WHITE_STROKE );
        let nose = Pos2 {
            x: self.position.x + RADIUS * self.angle.cos(),
            y: self.position.y + RADIUS * self.angle.sin()
        };
        painter.line_segment([center, nose], WHITE_STROKE);
    }
}
