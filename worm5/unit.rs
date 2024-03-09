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
    R_UNIT,
    MS_UNIT,
    KS_UNIT,
    L0_UNIT,
    KF_UNIT,
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
            position: vec2(130.0-L0_UNIT, INIT_Y),
            velocity: vec2(0.5, 0.0),
            letter:   ' ',
            l0:       L0_UNIT,
            radius:   R_UNIT,
            color:    Color32::GREEN,
        }
    }
}


impl Unit {
    pub fn pull_me(&mut self, pos_lead: Vec2) {
        let f = calc_hooke_force(pos_lead - self.position, L0_UNIT, KS_UNIT);
        self.velocity += (f - KF_UNIT*self.velocity)/MS_UNIT;
        self.position += self.velocity;
    }

    pub fn move_me(&mut self, target_pos: Vec2, preceding_pos: Vec2) -> Vec2 {
        let f = KS_UNIT * (target_pos - self.position);
        self.velocity += (f - KF_UNIT*self.velocity)/MS_UNIT;
        self.position += self.velocity;
        let seg = preceding_pos - self.position;
        let ang = seg.angle(); // this is the key
        let xt = self.position.x - self.l0 * ang.cos();//LENGTH
        let yt = self.position.y - self.l0 * ang.sin();//LENGTH
        vec2(xt, yt) //  // target for next unit
    }

    pub fn paint(&self, painter: &Painter) {
        painter.circle_filled( self.position.to_pos2(),  R_UNIT, self.color, );
        painter.text(
            self.position.to_pos2(),
            Align2::CENTER_CENTER,
            self.letter,
            FontId{size: 15., family: FontFamily::Proportional},
            Color32::BLACK
        );
    }
}
