use egui::{
    Vec2,
    vec2,
    Color32,
    Painter,
    FontId,
    FontFamily,
    Align2,
};


use crate::consts::{
    UNIT_RADIUS,
    UNIT_MASS,
    UNIT_KS,
    UNIT_L0,
    UNIT_KF,
    INIT_Y,
    calc_hooke_force,
};


pub struct Unit {
    pub position: Vec2,
    pub velocity: Vec2,
    pub letter:   char,
    pub l0:       f32,
    pub radius:   f32,
    pub color:    Color32,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            position: vec2(130.0-UNIT_L0, INIT_Y),
            velocity: vec2(0.5, 0.0),
            letter:   ' ',
            l0:       UNIT_L0,
            radius:   UNIT_RADIUS,
            color:    Color32::GREEN,
        }
    }
}


impl Unit {
    pub fn pull_me(&mut self, pos_lead: Vec2) {
        let f = calc_hooke_force(pos_lead - self.position, UNIT_L0, UNIT_KS);
        self.velocity += (f - UNIT_KF*self.velocity)/UNIT_MASS;
        self.position += self.velocity;
    }

    pub fn move_me(&mut self, target_pos: Vec2, preceding_pos: Vec2) -> Vec2 {
        let f = UNIT_KS * (target_pos - self.position);
        self.velocity += (f - UNIT_KF*self.velocity)/UNIT_MASS;
        self.position += self.velocity;
        let seg = preceding_pos - self.position;
        let ang = seg.angle(); // this is the key
        let xt = self.position.x - self.l0 * ang.cos();//LENGTH
        let yt = self.position.y - self.l0 * ang.sin();//LENGTH
        vec2(xt, yt) //  // target for next unit
    }

    pub fn paint(&self, painter: &Painter) {
        painter.circle_filled( self.position.to_pos2(),  UNIT_RADIUS, self.color, );
        painter.text(
            self.position.to_pos2(),
            Align2::CENTER_CENTER,
            self.letter,
            FontId{size: 15., family: FontFamily::Proportional},
            Color32::BLACK
        );
    }
}
