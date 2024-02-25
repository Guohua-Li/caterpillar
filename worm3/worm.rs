use egui::{
    Vec2,
    vec2,
    Painter,
    Color32,
};

use crate::lead::Lead;
use crate::unit::Unit;

use crate::consts::{
    LEAD_RADIUS,
    LEAD_L0,
    UNIT_RADIUS,
    UNIT_L0,
    calc_hooke_force,
    //PURPLE1,
};

const PURPLE2: Color32   = Color32::from_rgb(190, 79, 233);
const PURPLE3: Color32   = Color32::from_rgb(208,106, 230);


pub struct Worm {
    pub head:  Lead,
    pub neck:  Lead,
    pub units: Vec<Unit>,
    pub soft_mode: bool,
}

impl Default for Worm {
    fn default() -> Self {
        Self {
            head:  Lead::default(),
            neck:  Lead::default(),
            units: vec![],
            soft_mode: true,
        }
    }
}

impl Worm {

    pub fn reset(&mut self) {
        self.units.clear();

        self.neck.set_params(0.10, 0.15, 0.06);
        self.neck.position.x = self.head.position.x - LEAD_L0;
        self.neck.radius   = (LEAD_RADIUS+UNIT_RADIUS)/2.0;

        let mut u  = Unit::default();
        u.color    = PURPLE2;
        u.position.x = self.head.position.x - LEAD_L0 - UNIT_L0;
        self.units.push(u);

        let mut u  = Unit::default();
        u.color    = PURPLE3;
        u.position.x = self.head.position.x - LEAD_L0 - 2.0*UNIT_L0;
        self.units.push(u);
    }

    pub fn drive_me(&mut self, f: Vec2) {
        self.head.drive_me(f);
        let f = calc_hooke_force(self.head.position - self.neck.position, LEAD_L0, self.neck.ks);
        self.neck.drive_me(f);

        if self.soft_mode {
            let mut p_lead = self.neck.position;
            for seg in &mut self.units {
                seg.pull_me(p_lead);
                p_lead = seg.position;
            }
        } else {
            let ang = (self.neck.position - self.units[0].position).angle();
            let xt = self.neck.position.x - UNIT_L0 * ang.cos();
            let yt = self.neck.position.y - UNIT_L0 * ang.sin();

            let mut p_target = vec2(xt, yt);
            let mut p_lead = self.neck.position;

            for seg in &mut self.units {
                p_target = seg.move_me(p_target, p_lead);
                p_lead   = seg.position;
            }
        }
    }

    pub fn cross_border(&mut self, size: Vec2) {
        if self.head.position.x <= LEAD_RADIUS || self.head.position.x > size.x - LEAD_RADIUS {
            self.head.velocity.x = -0.8 * self.head.velocity.x;
        } 
        if self.head.position.y <= LEAD_RADIUS || self.head.position.y > size.y - LEAD_RADIUS {
            self.head.velocity.y = -0.8 * self.head.velocity.y;
        } 
    }

    pub fn grow(&mut self, letter: char) {
        let mut u = Unit::default();
        u.letter = letter;
        u.position = self.units.last().unwrap().position;
        self.units.push(u);
    }


/*
    pub fn grow(&mut self, letter: char) {
        let mut u = Unit::default();
        u.letter = letter;
        if let Some(b) = self.units.last() {
            u.position = b.position;
        } else {
            u.position = self.head.position;
        }
        self.units.push(u);
    }

    pub fn shrink(&mut self) {
        if self.units.len() > 0 {
            self.units.pop();
        }
    }*/

    pub fn paint(&mut self, painter: &Painter) {
        for t in &mut self.units {
            t.paint(painter);//, self.pause_count
        }
        self.head.paint(painter);//, self.pause_count
        self.neck.paint(painter);//, self.pause_count
    }

}
