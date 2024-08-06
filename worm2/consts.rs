use egui::{
    FontId,
    FontFamily,
    Color32,
    Stroke,
};

#[derive(PartialEq)]
pub enum GameState {
    StartUI,
    Init,
    Play,
    Exit,
}


pub const FRICTION:      f32 = 0.01;
pub const MAX_FORCE:     f32 = 0.1;
pub const KS:            f32 = 0.25;
pub const MASS:          f32 = 2.85;
pub const RADIUS:        f32 = 13.0;
pub const EQLENGTH:      f32 = 1.385 * RADIUS; // 18.005
pub const DIAMETER:      f32 = 2.0 * RADIUS;
pub const ACC_PER_FRAME: f32 = 0.01;
pub const ANG_PER_FRAME: f32 = 0.05;

pub const WHITE_STROKE: Stroke = Stroke{width: 2.0, color: Color32::WHITE};
pub const FONT_ID: FontId      = FontId{size: 15., family: FontFamily::Proportional}; //--

