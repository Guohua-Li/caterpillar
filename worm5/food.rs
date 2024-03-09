use std::time::Instant;
use std::f32::consts::PI;

use egui::{
    Painter,
    Align2,
    Vec2,
    Color32,
    vec2,
    pos2,
    Shape,
    Stroke,
};

use crate::consts::{
    HALF_SIZE,
    FOOD_LEN,
    AMPLITUDE,
    FOOD_SPEED,
    R_TAIL,
    FONT_ID,
    BODY,
    EYE1,
    EYE2,
    OMEGA,
};


const DARKRED:   Color32 = Color32::DARK_RED;
const DARKGREEN: Color32 = Color32::DARK_GREEN;
const YELLOW:    Color32 = Color32::YELLOW;
const WHITE:     Color32 = Color32::WHITE;
const CENTER:    Align2  = Align2::CENTER_CENTER;



pub struct Food {
    pub id:      usize,
    pub pos:     Vec2,
    pub tag:     Option<char>,
    pub tag_pos: Vec2,
    pub angle:   f32,
    pub speed:   f32,
    pub t0:      Instant,
}


impl Default for Food {
    fn default() -> Self {
        Self {
            id:      0,
            pos:     vec2(0.0, 0.0),
            tag:     None,
            angle:   0.0,
            speed:   FOOD_SPEED,
            tag_pos: vec2(0.0, 0.0),
            t0:      Instant::now(),
        }
    }
}


impl Food {

    pub fn move_me(&mut self, ui_size: Vec2) { // new
        let rx = if self.pos.x <= 0.0 {
            self.angle = PI - self.angle;
            2.0
        } else if self.pos.x >= ui_size.x {
            self.angle = PI - self.angle;
            -2.0
        } else {
            0.0
        };

        let ry = if self.pos.y <= 0.0 {
            self.angle = PI*2.0 - self.angle;
            2.0
        } else if self.pos.y >= ui_size.y {
            self.angle = PI*2.0 - self.angle;
            -2.0
        } else {
            0.0
        };

        let vx = self.speed * self.angle.cos() + rx;
        let vy = self.speed * self.angle.sin() + ry;
        self.pos.x += vx;
        self.pos.y += vy;
    }

    pub fn paint(&mut self, painter: &Painter) {
        self.wag_tail(); // new

        if let Some(letter) = self.tag {
            painter.circle_filled(self.tag_pos.to_pos2(), R_TAIL, DARKRED);
            painter.text(self.tag_pos.to_pos2(), CENTER, letter, FONT_ID, WHITE );
            painter.circle_filled(self.pos.to_pos2(), HALF_SIZE, DARKRED);
        } else {
            painter.circle_filled(self.tag_pos.to_pos2(), R_TAIL, DARKGREEN);
            let body: Shape = build_shape(self.pos, self.angle, BODY, DARKGREEN); // new
            let eye1: Shape = build_shape(self.pos, self.angle, EYE1, YELLOW); // new
            let eye2: Shape = build_shape(self.pos, self.angle, EYE2, YELLOW); // new
            painter.add(body);
            painter.add(eye1);
            painter.add(eye2);

        }
    }

    pub fn set_id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn set_pos(mut self, pos: Vec2) -> Self {
        self.pos = pos;
        self
    }

    pub fn set_tag(mut self, tag: Option<char>) -> Self {
        self.tag = tag;
        self
    }

    pub fn wag_tail(&mut self) {
        self.tag_pos.x = self.pos.x - FOOD_LEN * self.angle.cos();
        self.tag_pos.y = self.pos.y - FOOD_LEN * self.angle.sin();
        if self.tag == None {
            let t = Instant::now().duration_since(self.t0).as_secs_f32();
            let delta = AMPLITUDE * (OMEGA * t).sin();
            self.tag_pos.x += delta * self.angle.sin();
            self.tag_pos.y -= delta * self.angle.cos();
        }
    }
}


fn build_shape(pos: Vec2, ang: f32, points: [Vec2; 4], color: Color32) -> Shape {
    let mut out = vec!();
    for p in &points {
        let x = p.x * ang.cos() - p.y * ang.sin() + pos.x;
        let y = p.x * ang.sin() + p.y * ang.cos() + pos.y;
        out.push(pos2(x, y));
    }
    let stroke = Stroke::new(0.0, color);
    Shape::convex_polygon(out, color, stroke)
}
