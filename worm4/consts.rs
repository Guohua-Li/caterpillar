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
    GameOverUI,
    Init,
    Play,
}


pub const ZOO_ANIMALS: [&str; 51] = [
  "anteater", "armadillo", "badger", "bat", "bear", "beaver", "bison", "camel", "chameleon", "cheetah",
  "chimpanzee", "chipmunk", "crocodile", "deer", "elephant", "fox", "frog", "giraffe", "goldfish", "gorilla",
  "hamster", "hedgehog", "hippopotamus", "hyena", "iguana", "kangaroo", "koala", "leopard", "lion", "lizard",
  "llama", "monkey", "moose", "mouse", "orangutan", "panda", "pangolin", "raccoon", "rhinoceros",
  "scorpion", "skunk", "snail", "snake", "spider", "squirrel", "tiger", "toad", 
  "walrus", "warthog", "wolf", "zebra",
];

pub const FARM_ANIMALS: [&str; 11] = [
  "cat", "cow", "dog", "donkey", "ferret", "goat", "horse", "pig", "piglet", "rabbit",
  "sheep",
];

pub const FRUITS: [&str; 20] = [
  "apple", "avocado", "banana", "coconut", "durian", "grapes", "guava", "kivi", "lemon", "mango",
  "olives", "orange", "peach", "pear", "pineapple", "plums", "pomegranate", "raspberries", "strawberry", "watermelon"
];

pub const VEGETABLES: [&str; 20] = [
  "asparagus", "broccoli", "cabbage",  "carrot", "cauliflower", "cucumber", "eggplant", "garlic", "ginger", "leek",
  "lettuce", "mushroom", "onion", "peanut", "peas", "potato", "pumpkin", "radish", "tomato", "turnip"
];

pub const BIRDS: [&str; 27] = [
  "cardinal", "chick", "crow", "duck", "eagle", "flamingo", "goose", "hen", "hummingbird", "magpie",
  "ostrich", "owl", "parrot", "peacock", "pelican", "penguin", "pigeon", "puffin", "rooster", "seagull",
  "sparrow", "swallow", "swan", "toucan", "turkey", "vulture", "woodpecker"
];

pub const SEA_ANIMALS: [&str; 19] = [
  "butterflyfish", "clam", "clownfish", "crab", "dolphin", "jellyfish", "lobster", "mussel", "octopus", "pufferfish",
  "sailfish", "seahorse", "seal", "shark", "shrimp", "squid", "starfish", "turtle", "whale"
];

pub const INSECTS: [&str; 17] = [
  "ant", "bee", "beetle", "bumblebee", "butterfly", "caterpillar", "cicada", "cricket", "dragonfly", "firefly",
  "grasshopper", "housefly", "ladybug",  "mantis", "mosquito", "moth", "wasp"
];

pub const FOOD_AND_DRINKS: [&str; 22] = [
  "bread", "burger", "cake", "cheese", "chocolate", "coffee", "cookies", "corn", "doughnut", "dumpling",
  "lollipop", "egg", "hotdog", "juice",    "meat",  "muffin", "noodles", "pizza", 
  "salad", "sandwich", "spaghetti", "sushi"
];

pub const BODY_PARTS: [&str; 14] = [
  "ear", "eye", "eyebrow", "eyelash", "face", "feather", "hair", "hand", "leg", "lips",
  "neck", "nose", "teeth", "tongue"
];

pub const SPORT_AND_GAMES: [&str; 12] = [
  "cycling", "diving", "rowing", "rugby", "sailing", "shuttlecock", "skateboarding", "skiing", "swimming", "taekwondo",
  "volleyball", "weightlifting"
];

pub const HEAD_SIZE:   f32 = 28.0;
pub const TAIL_RADIUS: f32 = 12.0;

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
