use std::thread;
use rand::Rng;

use egui::{
    Context, Ui, CentralPanel, Key, ViewportCommand,
    SidePanel,
    RichText, Vec2, vec2, Color32, Button,
    FontFamily,
    FontData,
    FontDefinitions,
    Grid,
    TextStyle,
    Response,
};

use ears::{
    Sound,
    AudioController
};

const HEADING: TextStyle = TextStyle::Heading;
const WHITE:   Color32 = Color32::WHITE;
const MIN_DIST: f32 = 2.0 * LEAD_RADIUS;

use crate::consts::{
    GameState,
    HEAD_SIZE,
    DIAMETER,
    MAX_FORWARD,
    MAX_TURN,
    LEAD_RADIUS,
    PURPLE1,
    ZOO_ANIMALS,
    SEA_ANIMALS,
    BIRDS,
    INSECTS,
    FARM_ANIMALS,
    BODY_PARTS,
    FRUITS,
    VEGETABLES,
    FOOD_AND_DRINKS,
    SPORT_AND_GAMES,
};

use crate::food::Food;
use crate::worm::Worm;



pub struct Game {
    paused:     bool,
    worm:       Worm,
    vocabulary: Vec<String>,
    word:       String,
    foods:      Vec<Food>,
    game_state: GameState,
    char_stack: Vec<char>,//<>
    n_chars:    usize,
    forward_f:    f32,
    left_f:       f32,
    right_f:      f32,
    canvas_size:  Vec2,

}

impl Default for Game {
    fn default() -> Self {
        Self {
            paused: false,
            worm:   Worm::default(),
            vocabulary: ZOO_ANIMALS.iter().map(|s| s.to_string()).collect(),
            word:  "hippopotamus".to_string(),
            foods: Vec::new(),
            game_state: GameState::StartUI,
            char_stack: Vec::new(),
            n_chars: 0,

            forward_f:   0.0,
            left_f:      0.0,
            right_f:     0.0,
            canvas_size: vec2(0.0, 0.0),

        }
    }
}

impl Game {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        Default::default()
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Game {

    fn choose_word(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_i = rng.gen_range(0..self.vocabulary.len());
        self.word = self.vocabulary[rand_i].clone();
        self.char_stack = self.word.chars().collect();
        self.char_stack.reverse();
        let s = format!("sounds/{}.wav", self.word);
        thread::spawn(move|| {
            let mut snd = Sound::new(&s).unwrap();
            snd.play();
            while snd.is_playing() {}
        });

    }

    fn create_foods(&mut self) {
        self.foods.clear();
        let mut rng = rand::thread_rng();
        let mut id: usize = 0;
        while self.foods.len() < 5 {
            let x = rng.gen_range(DIAMETER..self.canvas_size.x-DIAMETER);
            let y = rng.gen_range(DIAMETER..self.canvas_size.y-DIAMETER);
            let position = Vec2 { x, y };
            let mut push = true;
            for fd in &self.foods {
                if (fd.pos - position).length() < 2.0 * DIAMETER {
                    push = false;
                    break
                }
            }
            if push {
                let letter: Option<char> = if id < 3 { self.char_stack.pop() } else { None };
                let tag_pos = Vec2 { x: x - 0.8 * HEAD_SIZE, y: y };
                let food = Food {
                    //id,
                    pos: position, tag: letter, tag_pos
                };
                self.foods.push(food);
                id += 1;
            }
        }
        self.n_chars = 3;
    }

    fn find_food(&mut self) -> Option<usize> {
        for i in 0..self.foods.len() {
            if self.foods[i].tag == None {
                continue;
            }
            if  (self.foods[i].tag_pos - self.worm.head.position).length() < DIAMETER {
                return Some(i);
            }
        }
        return None;
    }

    /*fn find_food2(&mut self) -> Option<usize> {
        for i in 0..self.foods.len() {
            let d_tag = (self.foods[i].tag_pos - self.worm.head.position).length();
            let d_head = (self.foods[i].pos - self.worm.head.position).length();
            if d_head > DIAMETER {
                if d_tag > DIAMETER {
                    return None;
                } else {
                    self.worm.head.velocity = 0.95 * self.worm.head.velocity;                    
                    return Some(i);
                }
            }

            // now d_head < DIAMETER
            self.worm.head.velocity = 0.95 * self.worm.head.velocity;                    
            /*if d_tag < DIAMETER {
                return Some(i);
            }

            if d_head < DIAMETER {
                if d_tag > DIAMETER {
                    self.worm.head.velocity = 0.95 * self.worm.head.velocity;                    
                } else {


                }
            }*/



            if self.foods[i].tag == None {
                continue;
            }
            if  (self.foods[i].tag_pos - self.worm.head.position).length() < DIAMETER {
                return Some(i);
            }
        }
        return None;
    }*/

    fn handling_caught(&mut self, idx: usize) {
        if self.foods[idx].tag == self.foods[0].tag {
            self.play_audio("bite".to_owned());
            self.worm.grow(self.foods[idx].tag.unwrap());
            let new_lett = self.char_stack.pop();
            self.foods[idx].tag = new_lett;
            if idx != 0 {
                self.foods.swap(0, idx);
            }
            let pos = self.rand_vec2(self.canvas_size);
            self.foods[0].pos = pos;
            self.foods[0].tag_pos = Vec2 {
                x: pos.x - 0.8 * HEAD_SIZE,
                y: pos.y,
            };
            let _ = &self.foods[0..self.n_chars].rotate_left(1);
            if new_lett == None { self.n_chars -= 1; }
            if self.n_chars == 0 { // winning
                self.game_state = GameState::GameOverUI;
            }
        }
    }

    /*fn play_audio1(&mut self) {
        let s = format!("sounds/{}.wav", self.word);
        thread::spawn(move|| {
            let mut snd = Sound::new(&s).unwrap();
            snd.play();
            while snd.is_playing() {}
        });
    }*/

    fn play_audio(&mut self, s: String) {
        let s = format!("sounds/{}.wav", s);
        thread::spawn(move|| {
            let mut snd = Sound::new(&s).unwrap();
            snd.play();
            while snd.is_playing() {}
        });
    }

    fn center_widgets(&mut self, ui: &mut Ui) {
        ui.add_space(40.);
        let txt = RichText::new("Current Vocab:").color(Color32::RED).size(22.);
        ui.label(txt);
        ui.add_space(20.);
        let txt = RichText::new(self.vocabulary.join(", ")).color(Color32::GREEN).size(20.);
        ui.label(txt);
    }

    fn side_widgets(&mut self, ui: &mut Ui) {
        ui.add_space(10.0);
        ui.label(RichText::new("How to Play:").size(20.0).color(WHITE));
        ui.add_space(10.0);
        ui.label(RichText::new("Move the worm and catch the letters to form a word.").size(18.0).color(WHITE));

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.add_space(50.0);
            ui.vertical(|ui| {
                ui.label(RichText::new("Space  -> pause").size(18.0).color(Color32::GREEN));
                ui.label(RichText::new("Escape -> quit").size(18.0).color(Color32::GREEN));
                ui.label(RichText::new("P      -> play audio").size(18.0).color(Color32::GREEN));
                ui.label(RichText::new("R      -> reset").size(18.0).color(Color32::GREEN));
                ui.label(RichText::new("F1     -> toggle").size(18.0).color(Color32::GREEN));
            });
            ui.add_space(50.0);
            ui.vertical(|ui| {
                ui.label(RichText::new("ArrowUp    -> forward").size(18.0).color(WHITE));
                ui.label(RichText::new("ArrowLeft  -> left").size(18.0).color(WHITE));
                ui.label(RichText::new("ArrowRight -> right").size(18.0).color(WHITE));
                ui.label(RichText::new("ArrowDown  -> break").size(18.0).color(WHITE));
            });
        });
        ui.add_space(50.0);
        self.vocabs(ui);
        ui.add_space(50.0);

        ui.horizontal(|ui| {
            if button(ui, "  Start  ").clicked() {
                self.game_state = GameState::Init;
            }
            if button(ui, "  Quit  ").clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Close);
            }
        });
    }

    fn calc_input_force(&mut self, ctx: &Context) -> Vec2 {
        let keys_down = ctx.input( |i| i.keys_down.to_owned() );

        if keys_down.is_empty() {
            self.forward_f = 0.0;
            self.left_f    = 0.0;
            self.right_f   = 0.0;
            return vec2(0.0, 0.0);
        }

        let ang = (self.worm.head.position - self.worm.neck.position).angle();

        if keys_down.contains(&Key::ArrowUp) {
            self.forward_f += 0.01;
            self.forward_f = self.forward_f.min(MAX_FORWARD);
            self.left_f    = 0.0;
            self.right_f   = 0.0;
            return vec2(
                self.forward_f * ang.cos(),
                self.forward_f * ang.sin()
            );
        }

        if keys_down.contains(&Key::ArrowRight) {
            self.worm.head.velocity = 0.99 * self.worm.head.velocity;
            self.left_f    = 0.0;
            self.forward_f = 0.0;
            self.right_f += 0.01;
            self.right_f = self.right_f.min(MAX_TURN);
            return vec2(
                -(self.right_f+0.2) * ang.sin(),
                 (self.right_f+0.2) * ang.cos()
            );
        }

        if keys_down.contains(&Key::ArrowLeft) {
            self.worm.head.velocity = 0.99 * self.worm.head.velocity;
            self.right_f   = 0.0;
            self.forward_f = 0.0;
            self.left_f += 0.01;
            self.left_f = self.left_f.min(MAX_TURN);
            return vec2(
                 (self.left_f+0.2) * ang.sin(),
                -(self.left_f+0.2) * ang.cos()
            );
        }

        if keys_down.contains(&Key::ArrowDown) {
            self.worm.head.velocity = 0.95 * self.worm.head.velocity;
            self.forward_f = 0.0;
            self.left_f    = 0.0;
            self.right_f   = 0.0;
            return vec2(0.0, 0.0);
        }

        // other keys
        self.left_f    = 0.0;
        self.right_f   = 0.0;
        self.forward_f = 0.0;
        vec2(0.0, 0.0)
    }

    fn rand_vec2(&mut self, canvas: Vec2) -> Vec2 {
        let mut rng = rand::thread_rng();
        let mut pos = Vec2 { x: 0.0, y: 0.0 };
        for _ in 0..10 { // try ten times
            pos = vec2 (
                rng.gen_range(MIN_DIST..canvas.x-MIN_DIST),
                rng.gen_range(MIN_DIST..canvas.y-MIN_DIST),
            );
            let mut overlap = false;
            for fd in &self.foods {
                if (fd.pos - pos).length() < 2.0 * MIN_DIST {
                    overlap = true;
                    break;
                }
            }

            if overlap == false {
                if (self.worm.head.position - pos).length() < 2.0 * MIN_DIST {
                    overlap = true;
                }
            }

            if overlap == false {
                break;
            }
        }
        return pos;
    }


}



impl eframe::App for Game {

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        if self.game_state == GameState::StartUI {
            SidePanel::left("my_left_panel").show(ctx, |ui| {
                self.side_widgets(ui);
            });
            CentralPanel::default().show(ctx, |ui| {
                self.center_widgets(ui);
            });
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        if ctx.input(|i| i.key_pressed(Key::R)) {
            self.reset();
        }

        if self.game_state == GameState::Play {
            if ctx.input(|i| i.key_pressed(Key::Space)) {
                self.paused = !self.paused;
            }

            if ctx.input(|i| i.key_pressed(Key::P)) {
                self.play_audio(self.word.to_owned());
            }

            if ctx.input(|i| i.key_pressed(Key::F1)) {
                self.worm.soft_mode = !self.worm.soft_mode;
                if self.worm.soft_mode {
                    self.worm.head.color = PURPLE1;
                    self.worm.neck.color = PURPLE1;
                } else {
                    self.worm.head.color = Color32::DARK_RED;
                    self.worm.neck.color = Color32::DARK_RED;
                }
            }

            if !self.paused {
                let f: Vec2 = self.calc_input_force(&ctx);
                self.worm.drive_me(f);
                self.worm.cross_border(self.canvas_size);
                if let Some(idx) = self.find_food() {
                    self.handling_caught(idx);
                }
            }
        } // end of Play


        CentralPanel::default().show(ctx, |ui| {
            let avail_size = ui.available_size();
            if avail_size != self.canvas_size {
                self.canvas_size = avail_size;
            }

            if self.game_state == GameState::Init {
                self.worm.reset();
                self.choose_word();
                self.create_foods();
                self.play_audio(self.word.to_owned());
                self.game_state = GameState::Play;
                return;
            }

            if self.game_state == GameState::GameOverUI {
                self.game_over_ui(ui);
            }

            let painter = ui.painter();
            self.worm.paint(painter);
            for fd in &mut self.foods {//&mut 
                fd.paint(painter);
            }
        });
    }
}


impl Game {

    fn vocabs(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Select Vocabs (Default: Zoo Animals):").size(20.0).color(WHITE));
        ui.add_space(10.0);
        Grid::new("some_unique_id").show(ui, |ui| {
            if button(ui, "Zoo Animals").clicked() {
                self.vocabulary = ZOO_ANIMALS.iter().map(|s| s.to_string()).collect();
                self.play_audio("zoo-animals".to_owned());
            }
            if button(ui, "Sea Animals").clicked() {
                self.vocabulary = SEA_ANIMALS.iter().map(|s| s.to_string()).collect();
                self.play_audio("sea-animals".to_owned());
            }
            ui.end_row();

            if button(ui, "Birds").clicked() {
                self.vocabulary = BIRDS.iter().map(|s| s.to_string()).collect();
                self.play_audio("birds".to_owned());
            }
            if button(ui, "Insects").clicked() {
                self.vocabulary = INSECTS.iter().map(|s| s.to_string()).collect();
                self.play_audio("insects".to_owned());
            }
            ui.end_row();

            if button(ui, "Farm Animals").clicked() {
                self.vocabulary = FARM_ANIMALS.iter().map(|s| s.to_string()).collect();
                self.play_audio("farm-animals".to_owned());
            }
            if button(ui, "Body Parts").clicked() {
                self.vocabulary = BODY_PARTS.iter().map(|s| s.to_string()).collect();
                self.play_audio("body-parts".to_owned());
            }
            ui.end_row();

            if button(ui, "Fruits").clicked() {
                self.vocabulary = FRUITS.iter().map(|s| s.to_string()).collect();
                self.play_audio("fruits".to_owned());
            }
            if button(ui, "Vegetables").clicked() {
                self.vocabulary = VEGETABLES.iter().map(|s| s.to_string()).collect();
                self.play_audio("vegetables".to_owned());
            }
            ui.end_row();

            if button(ui, "Food and Drinks").clicked() {
                self.vocabulary = FOOD_AND_DRINKS.iter().map(|s| s.to_string()).collect();
                self.play_audio("food-and-drinks".to_owned());
            }
            if button(ui, "Sport and Games").clicked() {
                self.vocabulary = SPORT_AND_GAMES.iter().map(|s| s.to_string()).collect();
                self.play_audio("sport-and-games".to_owned());
            }
        });
    }

    fn game_over_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let button = Button::new(RichText::new("Restart").text_style(HEADING));//Button
            if ui.add_sized(Vec2{x: self.canvas_size.x/3.0, y: 40.0}, button).clicked() {
                self.reset();
            }
            let button = Button::new(RichText::new("Quit").text_style(HEADING));
            if ui.add_sized(Vec2{x: self.canvas_size.x/3.0, y: 40.0}, button).clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Close);
            }

            let button = Button::new(RichText::new("Continue").text_style(HEADING));
            if ui.add_sized(ui.available_size(), button).clicked() {
                self.game_state = GameState::Init;
            }

        });
    }

}

pub fn configure_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        FontData::from_static(include_bytes!("../fonts/MesloLGS_NF_Regular.ttf"))
    );
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
    fonts.families.get_mut(&FontFamily::Monospace).unwrap().push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

fn button(ui: &mut Ui, text: &str) -> Response {
    ui.add_sized(
        Vec2{x: 280.0, y: 40.0},
        Button::new(RichText::new(text).text_style(HEADING))
    )
}
