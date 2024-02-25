use egui::{
    Vec2,
    Painter,
    Color32,
    Align2,
};


use crate::consts::{
    UNIT_RADIUS,
    FONT_ID,
};

#[derive(Default)]
pub struct Food {
    pub id:       usize,        // default: value of 0
    pub position: Vec2,         // default: vec2(0.0, 0.0)
    pub letter:   Option<char>, // default: None
}



impl Food {
    pub fn paint(&self, painter: &Painter) {
        let pos = self.position.to_pos2();
        painter.circle_filled(pos, UNIT_RADIUS, Color32::DARK_RED);
        if let Some(s) = self.letter {
            painter.text( pos, Align2::CENTER_CENTER, s, FONT_ID, Color32::WHITE );
        }
    }

    /*pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_pos(mut self, pos: Vec2) -> Self {
        self.position = pos;
        self
    }

    pub fn set_letter(mut self, lett: Option<char>) -> Self {
        self.letter = lett;
        self
    }*/

}

