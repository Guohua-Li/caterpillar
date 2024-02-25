use egui::Vec2;

use crate::head::Head;
use crate::unit::Unit;

pub struct Worm {
    pub head: Head,
    pub units: Vec<Unit>,
}

impl Default for Worm {
    fn default() -> Self {
        Self {
            head: Head::default(),
            units: vec![],
        }
    }
}

impl Worm {
    pub fn move_me(&mut self) {
        self.head.tick();
        let mut p_lead = self.head.position;
        for b in &mut self.units {
            b.tick(p_lead);
            p_lead = b.position;
        }
    }

    pub fn bounce(&mut self, canvas_size: Vec2) {
        self.head.bounce(canvas_size);
    }

    pub fn grow(&mut self) {
        let mut u = Unit::default();
        if let Some(b) = self.units.last() {
            u.position = b.position;
        } else {
            u.position = self.head.position;
        }
        self.units.push(u);
    }

    /*pub fn shrink(&mut self) {
        if self.units.len() > 0 {
            self.units.pop();
        }
    }*/
}
