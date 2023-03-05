
use gc2d::{gc2d::Gc2d, color::Color, keyboard::KeyCode};

use crate::{map::{MAP_NUMBER_IN_HEIGHT, MAP_NUMBER_IN_WIDTH, TILE_HEIGHT, TILE_WIDTH}, game::Vector2};

#[derive(PartialEq, Clone, Copy)]
enum SnakeDirection {
    Left,
    Up,
    Right,
    Down,
}

#[derive(PartialEq)]
enum SnakeState {
    Alive,
    Dead,
}

pub struct Snake {
    drawing_position: Vector2,

    head_position: Vector2,
    queue: Vec<Vector2>,
    speed: Option<f32>,

    actual_move_delay: f32,
    actual_direction: SnakeDirection,
    next_direction: SnakeDirection,

    size: usize,
    state: SnakeState,
}

impl Snake {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            drawing_position: Vector2::new(pos_x, pos_y),
            head_position: Vector2::zero(),
            queue: Vec::new(),
            speed: None,
            actual_move_delay: 0f32,
            actual_direction: SnakeDirection::Right,
            next_direction: SnakeDirection::Right,
            size: 10,
            state: SnakeState::Alive,
        }
    }

    pub fn init(&mut self, pos_x: f32, pos_y: f32, speed: Option<f32>) {
        self.head_position.x = pos_x;
        self.head_position.y = pos_y;
        self.queue.clear();
        self.speed = speed;
        self.actual_move_delay = 0f32;
        self.actual_direction = SnakeDirection::Right;
        self.next_direction = self.actual_direction;
        self.state = SnakeState::Alive;
    }

    pub fn draw(&self, gc2d: &mut Gc2d) {

        for elem in &self.queue {

             let color = if *elem == self.head_position {
                                    Color::GREEN
                                } else {
                                     Color::RED
                                }
                 ;

            gc2d.graphics.rectangle(
                gc2d::graphics::DrawMode::Fill,
                elem.x + self.drawing_position.x,
                elem.y + self.drawing_position.y,
                (TILE_WIDTH - 1) as f32,
                (TILE_HEIGHT - 1) as f32,
                Some(color),
            );
        }
    }

    pub fn update(&mut self, gc2d: &mut Gc2d, dt: f32) {

        // Direction of snake
        if gc2d.keyboard.is_down(KeyCode::Up) && self.actual_direction != SnakeDirection::Down {
            self.next_direction = SnakeDirection::Up;
        }
        if gc2d.keyboard.is_down(KeyCode::Down) && self.actual_direction != SnakeDirection::Up {
            self.next_direction = SnakeDirection::Down;
        }
        if gc2d.keyboard.is_down(KeyCode::Right) && self.actual_direction != SnakeDirection::Left {
            self.next_direction = SnakeDirection::Right;
        }
        if gc2d.keyboard.is_down(KeyCode::Left) && self.actual_direction != SnakeDirection::Right {
            self.next_direction = SnakeDirection::Left;
        }

        // Move snake
        if let Some(speed) = self.speed {
            self.actual_move_delay += dt;
            if self.actual_move_delay > speed {
                self.actual_move_delay = 0f32;
                self.actual_direction = self.next_direction;
                self.move_snake();
            }
        }

    }

    pub fn is_alive(&self) -> bool {
        self.state == SnakeState::Alive
    }

    fn move_snake_number_tiles(&mut self, x: i32, y: i32) {

        self.head_position.x += (x * TILE_WIDTH) as f32;
        self.head_position.y += (y * TILE_HEIGHT) as f32;

        let next_head = self.head_position;

        // Add new position only if snake is alive
        if self.queue.contains(&next_head)
            || next_head.x < 0.
            || next_head.y < 0.
            || next_head.x > ((MAP_NUMBER_IN_WIDTH-1) * TILE_WIDTH) as f32
            || next_head.y > ((MAP_NUMBER_IN_HEIGHT-1) * TILE_HEIGHT) as f32 {
            self.state = SnakeState::Dead;
        } else {
            self.queue.push(next_head);

            if self.queue.len() > self.size {
                self.queue.remove(0);
            }
        }
    }

    fn move_snake(&mut self) {
        match self.actual_direction {
            SnakeDirection::Left => self.move_snake_number_tiles(-1, 0),
            SnakeDirection::Up => self.move_snake_number_tiles(0, -1),
            SnakeDirection::Right => self.move_snake_number_tiles(1, 0),
            SnakeDirection::Down => self.move_snake_number_tiles(0, 1),
        }
    }
}