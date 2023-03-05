use gc2d::{gc2d::Gc2d, color::Color};

pub const TILE_WIDTH: i32 = 24;
pub const TILE_HEIGHT: i32 = 24;

pub const MAP_NUMBER_IN_WIDTH: i32 = 25;
pub const MAP_NUMBER_IN_HEIGHT: i32 = 25;

pub struct Map {
    drawing_position_x: f32,
    drawing_position_y: f32,
}

impl Map {

    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Self {
            drawing_position_x: pos_x,
            drawing_position_y: pos_y,
        }
    }

    pub fn draw(&self, gc2d: &mut Gc2d) {
        for line in 0..MAP_NUMBER_IN_HEIGHT {
            for column in 0..MAP_NUMBER_IN_WIDTH {
                gc2d.graphics.rectangle(
                    gc2d::graphics::DrawMode::Line,
                    column as f32 * TILE_WIDTH as f32+ self.drawing_position_x,
                    line as f32 * TILE_HEIGHT as f32 + self.drawing_position_y,
                    TILE_WIDTH as f32 - 1.,
                    TILE_HEIGHT as f32 - 1.,
                    Some(Color::BLUE),
                );
            }
        }
    }
}