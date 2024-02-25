use egui::{
    Vec2,
    Painter,
    Color32,
    Align2,
};


use crate::consts::{
    RADIUS,
    FONT_ID,
};


pub struct Food {
    pub position: Vec2,
    pub letter: char,
}


impl Default for Food {
    fn default() -> Self {
        Self {
            position: Vec2{x: 0.0, y: 0.0},
            letter: ' ',
        }
    }
}


impl Food {
    pub fn paint(&self, painter: &Painter) {
        let pos = self.position.to_pos2();
        painter.circle_filled(pos, RADIUS, Color32::DARK_RED);
        painter.text( pos, Align2::CENTER_CENTER, self.letter, FONT_ID, Color32::WHITE );
    }
}

