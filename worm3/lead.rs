use egui::{
    Vec2,
    vec2,
    Painter,
    Color32,
};

use crate::consts::{
    LEAD_RADIUS,
    LEAD_KF,
    INIT_Y,
    PURPLE1,
};




pub struct Lead {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass:     f32,
    pub kf:       f32,
    pub ks:       f32,
    pub radius:   f32,
    pub color:    Color32,
}

impl Lead {

    pub fn default() -> Self {
        Self { 
            position: vec2(90.0, INIT_Y),
            velocity: vec2(2.0, 0.0),
            mass:     3.9,
            kf:       LEAD_KF,
            ks:       0.03,
            radius:   LEAD_RADIUS,
            color:    PURPLE1,
        }
    }

    pub fn drive_me(&mut self, f: Vec2) {
        self.velocity += (f - self.kf * self.velocity) / self.mass;
        self.position += self.velocity;
    }

    pub fn paint(&self, painter: &Painter) {
        let center = self.position.to_pos2();
        painter.circle_filled( center, self.radius, self.color );
    }

    pub fn set_params(&mut self, mass: f32, kf: f32, ks: f32) {
        self.mass = mass;
        self.kf = kf;
        self.ks = ks;
    }

    /*pub fn set_kf(mut self, kf: f32) -> Self {
        self.kf = kf;
        self
    }

    pub fn set_ks(mut self, ks: f32) -> Self {
        self.ks = ks;
        self
    }

    pub fn set_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }*/

}
