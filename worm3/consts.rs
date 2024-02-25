use egui::{
    FontId,
    FontFamily,
    Vec2,
    vec2,
    Color32,
};


#[derive(PartialEq)]
pub enum GameState {
    StartUI,
    Init,
    Play,
    Exit,
}

pub const ZOO_ANIMALS: [&str; 54] = [
  "anteater", "armadillo", "badger", "bat", "bear", "beaver", "bison", "camel", "chameleon", "cheetah",
  "chimpanzee", "chipmunk", "crocodile", "deer", "elephant", "fox", "frog", "giraffe", "goldfish", "gorilla",
  "hamster", "hedgehog", "hippopotamus", "hyena", "iguana", "kangaroo", "koala", "leopard", "lion", "lizard",
  "llama", "monkey", "moose", "mouse", "octopus", "orangutan", "panda", "pangolin", "raccoon", "rhinoceros",
  "scorpion", "seal", "skunk", "snail", "snake", "spider", "squirrel", "tiger", "toad", "turtle",
  "walrus", "warthog", "wolf", "zebra",
];

pub const INIT_Y:      f32 = 130.;

pub const MAX_FORWARD: f32 = 0.17;
pub const MAX_TURN:    f32 = 1.20;

pub const LEAD_KF:     f32 = 0.01;
pub const LEAD_L0:     f32 = 10.0;

pub const LEAD_RADIUS: f32 = 18.0;
pub const UNIT_RADIUS: f32 = 13.0;
pub const UNIT_L0:     f32 = 1.3 * UNIT_RADIUS;
pub const UNIT_MASS:   f32 = 0.20;
pub const UNIT_KF:     f32 = 0.10;
pub const UNIT_KS:     f32 = 0.03;

pub const DIAMETER:    f32 = 2.0 * UNIT_RADIUS;
pub const FONT_ID: FontId  = FontId{size: 15., family: FontFamily::Proportional};

pub const PURPLE1: Color32   = Color32::from_rgb(190, 52, 229);

pub fn calc_hooke_force(vec: Vec2, l0: f32, ks: f32) -> Vec2 {
    let ang = vec.angle();
    let strain = vec2(
        vec.x - l0 * ang.cos(),
        vec.y - l0 * ang.sin(),
    );
    return ks * strain;
}
