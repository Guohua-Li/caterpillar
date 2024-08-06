use egui::{
    Painter,
    Align2,
    Vec2,
    Color32,
    Rect,
    pos2,
};



use crate::consts::{
    HEAD_SIZE,
    TAIL_RADIUS,
    FONT_ID,
};


const DARKRED:   Color32 = Color32::DARK_RED;
const DARKGREEN: Color32 = Color32::DARK_GREEN;
const YELLOW:    Color32 = Color32::YELLOW;
const WHITE:     Color32 = Color32::WHITE;

const CENTER:    Align2  = Align2::CENTER_CENTER;

#[derive(Default)]
pub struct Food {
    //pub id:      usize,
    pub pos:     Vec2,
    pub tag:     Option<char>,
    pub tag_pos: Vec2
}



impl Food {

    pub fn paint(&mut self, painter: &Painter) {

        /*let tag_pos = Vec2 {
            x: self.pos.x - 0.8 * HEAD_SIZE,
            y: self.pos.y,
        };*/

        if let Some(letter) = self.tag {
            painter.circle_filled(self.tag_pos.to_pos2(), TAIL_RADIUS, DARKRED);
            painter.text(self.tag_pos.to_pos2(), CENTER, letter, FONT_ID, WHITE );
            painter.circle_filled(self.pos.to_pos2(), 0.5*HEAD_SIZE, DARKRED);
        } else {
            painter.circle_filled(self.tag_pos.to_pos2(), TAIL_RADIUS, DARKGREEN);
            let body = Rect {
                min: pos2(self.pos.x-HEAD_SIZE/2.0, self.pos.y-HEAD_SIZE/2.0),
                max: pos2(self.pos.x+HEAD_SIZE/2.0, self.pos.y+HEAD_SIZE/2.0),
            };
            painter.rect_filled(body, 0.0, DARKGREEN);

            /*
            let d = HEAD_SIZE/3.0;
            let pos_eye1 = pos2(self.pos.x+d, self.pos.y-d);
            let pos_eye2 = pos2(self.pos.x+d, self.pos.y+d);
            painter.circle_filled(pos_eye1, d/2., YELLOW );
            painter.circle_filled(pos_eye2, d/2., YELLOW );
            */
            let s = HEAD_SIZE/4.0;
            let rect_eye1 = Rect{
                min: pos2(self.pos.x+s,     self.pos.y-2.0*s),
                max: pos2(self.pos.x+2.0*s, self.pos.y-s    ),
            };
            let rect_eye2 = Rect{
                min: pos2(self.pos.x+s,     self.pos.y+s    ),
                max: pos2(self.pos.x+2.0*s, self.pos.y+2.0*s),
            };
            painter.rect_filled(rect_eye1, 0.0, YELLOW);
            painter.rect_filled(rect_eye2, 0.0, YELLOW);
            //painter.add(eye1);
            //painter.add(eye2);
        }
    }

}
