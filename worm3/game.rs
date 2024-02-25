use std::thread;
use rand::Rng;

use egui::{
    Context, Ui, CentralPanel, Key, ViewportCommand,
    RichText, Vec2, vec2, Color32, Button,
};

use ears::{
    Sound,
    AudioController
};

use crate::consts::{
    GameState,
    ZOO_ANIMALS,
    DIAMETER,
    MAX_FORWARD,
    MAX_TURN,
    LEAD_RADIUS,
    PURPLE1,
};

use crate::food::Food;
use crate::worm::Worm;

const MIN_DIST: f32 = 2.0 * LEAD_RADIUS;

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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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

    fn create_foods(&mut self, canvas_size: Vec2) {
        self.foods.clear();
        let mut rng = rand::thread_rng();
        let mut id: usize = 0;
        while self.foods.len() < 5 {
            let x = rng.gen_range(DIAMETER..canvas_size.x-DIAMETER);
            let y = rng.gen_range(DIAMETER..canvas_size.y-DIAMETER);
            let position = Vec2 { x, y };
            let mut push = true;
            for fd in &self.foods {
                if (fd.position - position).length() < 2.0 * DIAMETER {
                    push = false;
                    break
                }
            }
            if push {
                let letter: Option<char> = if id < 3 { self.char_stack.pop() } else { None };
                self.foods.push(  Food { id, position, letter }  );
                id += 1;
            }
        }
        self.n_chars = 3;
    }

    fn find_food(&mut self) -> Option<usize> {
        for i in 0..self.foods.len() {
            if self.foods[i].letter == None {
                continue;
            }
            if  (self.foods[i].position - self.worm.head.position).length() < DIAMETER {
                return Some(i);
            }
        }
        return None;
    }
    fn handling_caught(&mut self, idx: usize) {
        if self.foods[idx].letter == self.foods[0].letter {
            self.worm.grow(self.foods[idx].letter.unwrap());
            let new_lett = self.char_stack.pop();
            self.foods[idx].letter = new_lett;
            if idx != 0 {
                self.foods.swap(0, idx);
            }
            self.foods[0].position = self.rand_vec2(self.canvas_size);
            let _ = &self.foods[0..self.n_chars].rotate_left(1);
            if new_lett == None { self.n_chars -= 1; }
            if self.n_chars == 0 { // winning
                self.reset();
            }
        }
    }

    fn play_audio(&mut self) {
        let s = format!("sounds/{}.wav", self.word);
        thread::spawn(move|| {
            let mut snd = Sound::new(&s).unwrap();
            snd.play();
            while snd.is_playing() {}
        });
    }

    fn startup_ui(&mut self, ui: &mut Ui) {
        ui.label(RichText::new("Space  -> pause").size(18.0).color(Color32::GREEN));
        ui.label(RichText::new("Escape -> quit").size(18.0).color(Color32::GREEN));
        ui.label(RichText::new("R      -> reset").size(18.0).color(Color32::GREEN));
        ui.label(RichText::new("G      -> grow").size(18.0).color(Color32::GREEN));
        ui.add_space(20.0);
        ui.label(RichText::new("ArrowUp -> acceleration").size(18.0).color(Color32::WHITE));
        ui.label(RichText::new("ArrowLeft  -> left").size(18.0).color(Color32::WHITE));
        ui.label(RichText::new("ArrowRight -> right").size(18.0).color(Color32::WHITE));
        ui.label(RichText::new("ArrowDown  -> break").size(18.0).color(Color32::WHITE));
        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if ui.add_sized([150.,50.], Button::new(RichText::new("  Start  ").size(20.0))).clicked() {
                self.game_state = GameState::Init;
            }
            if ui.add_sized([150., 50.], Button::new(RichText::new("  Quit  ").size(20.0))).clicked() {
                self.game_state = GameState::Exit;
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
                if (fd.position - pos).length() < 2.0 * MIN_DIST {
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

        if self.game_state == GameState::Exit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.game_state = GameState::Exit;
        }

        if ctx.input(|i| i.key_pressed(Key::R)) {
            self.reset();
        }

        if self.game_state == GameState::Play {
            if ctx.input(|i| i.key_pressed(Key::Space)) {
                self.paused = !self.paused;
            }

            if ctx.input(|i| i.key_pressed(Key::P)) {
                self.play_audio();
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

            if self.game_state == GameState::StartUI {
                self.startup_ui(ui);
                return;
            }

            let canvas_size = ui.available_size();
            if self.game_state == GameState::Init {
                self.worm.reset();
                self.choose_word();
                self.create_foods(canvas_size);
                self.play_audio();
                self.game_state = GameState::Play;
                return;
            }

            let painter = ui.painter();
            self.worm.paint(painter);
            for fd in &self.foods {
                fd.paint(painter);
            }
        });
    }
}
