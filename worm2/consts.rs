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

pub const ZOO_ANIMALS: [&str; 54] = [
  "anteater", "armadillo", "badger", "bat", "bear", "beaver", "bison", "camel", "chameleon", "cheetah",
  "chimpanzee", "chipmunk", "crocodile", "deer", "elephant", "fox", "frog", "giraffe", "goldfish", "gorilla",
  "hamster", "hedgehog", "hippopotamus", "hyena", "iguana", "kangaroo", "koala", "leopard", "lion", "lizard",
  "llama", "monkey", "moose", "mouse", "octopus", "orangutan", "panda", "pangolin", "raccoon", "rhinoceros",
  "scorpion", "seal", "skunk", "snail", "snake", "spider", "squirrel", "tiger", "toad", "turtle",
  "walrus", "warthog", "wolf", "zebra",
];

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

