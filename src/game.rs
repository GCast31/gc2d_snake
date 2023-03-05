use gc2d::{self, gc2d::Gc2d, event::EventLoop};

use crate::map::{Map, MAP_NUMBER_IN_HEIGHT, MAP_NUMBER_IN_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use crate::snake::Snake;

const INITIAL_SNAKE_SPEED: Option<f32> = Some(0.2);

#[derive(PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0. }
    }
}

pub struct Game {
    map: Map,
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {  
            map: Map::new(50., 50.),
            snake: Snake::new(50., 50.),
        }
    }

    pub fn init(&mut self) {
        let center_x = ((MAP_NUMBER_IN_WIDTH as f32 / 2f32).floor() - 3.) * TILE_WIDTH as f32;
        let center_y = ((MAP_NUMBER_IN_HEIGHT as f32 / 2f32).floor() - 3.) * TILE_HEIGHT as f32;
        self.snake.init(center_x, center_y, INITIAL_SNAKE_SPEED);
    }
}

impl EventLoop for Game {
    fn load(&mut self, gc2d: &mut Gc2d, audio_manager: &mut gc2d::audio::AudioManager) -> Result<(), gc2d::event::EventError> {
        gc2d.window.set_size(700., 700.);
        gc2d.window.set_title("GC2d(Rust) Snake By GCast31");
        self.init();
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut Gc2d, fonts: &mut gc2d::fonts::FontsManager, dt: f32) -> Result<(), gc2d::event::EventError> {
        self.map.draw(gc2d);
        self.snake.draw(gc2d);
        Ok(())
    }

    fn update(&mut self, gc2d: &mut Gc2d, dt: f32, audio_manager: &mut gc2d::audio::AudioManager) -> Result<(), gc2d::event::EventError> {
        
        self.snake.update(gc2d, dt);

        if !self.snake.is_alive() {
            self.snake.init(0., 0., None);
        }

        Ok(())
    }
}