use std::thread;
use rand::Rng;

use egui::{
    Context, Ui, CentralPanel, Key, ViewportCommand,
    RichText, Vec2, Color32, Button,
};

use ears::{
    Sound,
    AudioController
};

use crate::consts::{
    GameState,
    ZOO_ANIMALS,
    DIAMETER,
    MAX_FORCE,
    ACC_PER_FRAME,
    ANG_PER_FRAME,
};

use crate::food::Food;
use crate::worm::Worm;


pub struct Game {
    paused: bool,
    worm:   Worm,
    vocabulary: Vec<String>,
    word:  String,
    foods: Vec<Food>,
    game_state: GameState,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            paused: false,
            worm:   Worm::default(),
            vocabulary: ZOO_ANIMALS.iter().map(|s| s.to_string()).collect(),
            word:  "moose".to_string(),
            foods: Vec::new(),
            game_state: GameState::StartUI,
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
    fn create_foods(&mut self, canvas_size: Vec2) {
        let mut rng = rand::thread_rng();
        //let i = rng.gen_range(0..self.vocabulary.len());
        //self.word = self.vocabulary[i].clone();
        let chars: Vec<_> = self.word.chars().collect();

        self.foods.clear();
        let mut count = 0;
        while self.foods.len() < self.word.len() {
            let x = rng.gen_range(DIAMETER..canvas_size.x-DIAMETER);
            let y = rng.gen_range(DIAMETER..canvas_size.y-DIAMETER);
            let pos = Vec2 { x, y };
            let mut push = true;
            for fd in &self.foods {
                if (fd.position - pos).length() < 2.0 * DIAMETER {
                    push = false;
                    break
                }
            }
            if push {
                self.foods.push(Food { position: pos, letter: chars[count] });
                count += 1;
            }
        }
    }

    fn find_food(&mut self) -> Option<usize> {
        let mut caught: Option<usize> = None;
        for i in 0..self.foods.len() {
            if  (self.foods[i].position-self.worm.head.position).length() < DIAMETER {
                caught = Some(i);
                break;
            }
        }
        return caught;
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
        ui.label(RichText::new("Press Space to pause").size(18.0).color(Color32::GREEN));
        ui.label(RichText::new("Press Escape to quit").size(18.0).color(Color32::GREEN));
        ui.label(RichText::new("Press R to reset").size(18.0).color(Color32::GREEN));
        ui.label(RichText::new("Press G to grow").size(18.0).color(Color32::GREEN));
        ui.add_space(20.0);
        ui.label(RichText::new("Press ArrowUp/Down for acceleration").size(18.0).color(Color32::WHITE));
        ui.label(RichText::new("Press ArrowLeft to turn left").size(18.0).color(Color32::WHITE));
        ui.label(RichText::new("Press ArrowRight to turn right").size(18.0).color(Color32::WHITE));
        ui.add_space(20.0);
        ui.horizontal(|ui| {
            if ui.add_sized([150., 50.], Button::new(RichText::new("  Start  ").size(20.0))).clicked() {
                self.game_state = GameState::Init;
            }
            if ui.add_sized([150., 50.], Button::new(RichText::new("  Quit  ").size(20.0))).clicked() {
                self.game_state = GameState::Exit;
            }
        });
    }

}



impl eframe::App for Game {

    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if self.game_state == GameState::Exit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }
        ctx.request_repaint();
        if ctx.input(|i| i.key_pressed(Key::Space)) {
            self.paused = !self.paused;
        } else if ctx.input(|i| i.key_pressed(Key::R)) {
            self.reset();
        } else if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        let key_down = ctx.input(|i|
            i.keys_down.to_owned()
        );

        if key_down.contains(&Key::ArrowUp) {
            if self.worm.head.force < MAX_FORCE {
                self.worm.head.force += ACC_PER_FRAME;
            }
        } else if key_down.contains(&Key::ArrowLeft) {
            self.worm.head.inc_angle(-ANG_PER_FRAME);
            //    let fx =   0.1 * self.worm.head.angle.sin();
            //    let fy = - 0.1 * self.worm.head.angle.cos();
            //    self.worm.head.force = (fx*fx + fy*fy).sqrt();
        } else if key_down.contains(&Key::ArrowRight) {
            self.worm.head.inc_angle(ANG_PER_FRAME);
            //    let fx = - 0.1 * self.worm.head.angle.sin();
            //    let fy =   0.1 * self.worm.head.angle.cos();
            //    self.worm.head.force = (fx*fx + fy*fy).sqrt();
        } else if key_down.contains(&Key::ArrowDown) {
            self.worm.head.speed = 0.9 * self.worm.head.speed;
        } else {
            self.worm.head.force = 0.0;
        }

        CentralPanel::default().show(ctx, |ui| {
            if self.game_state == GameState::StartUI {
                self.startup_ui(ui);
                return;
            }

            let canvas_size = ui.available_size();
            if self.game_state == GameState::Init {
                self.create_foods(canvas_size);
                self.play_audio();
                self.game_state = GameState::Play;
                return;
            } 

            if self.game_state == GameState::Play && !self.paused {
                self.worm.move_me();
                self.worm.bounce(ui.available_size());
                let found: Option<usize> = self.find_food();
                if let Some(idx) = found {
                    if self.foods[idx].letter == self.foods[0].letter {
                        self.worm.grow();
                        if idx != 0 {
                            self.foods.swap(0, idx);
                        }
                        self.foods.remove(0);
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::P)) {
                    self.play_audio();
                }
                if self.foods.len() == 0 {
                    self.reset();
                };
            }

            let painter = ui.painter();
            self.worm.head.paint(painter);
            for u in &self.worm.units {
                u.paint(painter);
            }
            for fd in &self.foods {
                fd.paint(painter);
            }
        });
    }
}
