use crate::common::{Component, Vec2};
use crate::{olc_pixel_game_engine as olc, Game};
use rand::Rng;

#[derive(Clone)]
pub struct Gof {
    content: Vec<u8>,
    dimensions: Vec2<usize>,
    offset: Vec2<f32>,
}

impl Gof {
    pub fn new() -> Self {
        let height: usize = (olc::screen_height()) as usize;
        let width: usize = (olc::screen_width()) as usize;
        let size: usize = width * height;
        let mut result = Self {
            dimensions: Vec2 {
                x: width,
                y: height,
            },
            content: vec![0u8; size],
            offset: Vec2{x: 0.0, y: 0.0},
        };
        result.start();

        result
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let mut state = false;
        for cell in self.content.iter_mut() {
            state = rng.gen();
            *cell = state as u8;
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &u8 {
        let i: usize = y * self.dimensions.x + x;
        return &self.content[i];
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: u8) -> &u8 {
        let i: usize = y * self.dimensions.x + x;
        self.content[i] = value;
        return &self.content[i];
    }

    pub fn neighbors(&self, x: usize, y: usize) -> u8 {
        let mut result = 0u8;

        for i in 0..3 {
            for j in 0..3 {
                let x = x - 1 + i;
                let y = y - 1 + j;
                let cell = self.get_cell(x, y);
                result += cell;
            }
        }

        result - self.get_cell(x, y)
    }

    pub fn tick(&mut self) {
        let mut new = self.clone();

        for x in 1..self.dimensions.x - 1 {
            for y in 1..self.dimensions.y - 1 {
                let cell = self.get_cell(x, y);
                let neighbors = self.neighbors(x, y);

                if *cell == 0u8 && neighbors == 3 {
                    new.set_cell(x, y, 1);
                } else if *cell == 1u8 && (neighbors < 2 || neighbors > 3) {
                    new.set_cell(x, y, 0);
                } else {
                    new.set_cell(x, y, *cell);
                }
            }
        }
        self.content = new.content;
    }
}

impl Component for Gof {

    fn id(&self) -> Option<u32> {
        Some(0u32)
    }

    fn start(&mut self) {
        self.randomize();
    }

    fn poll_inputs(&mut self, elapsed_time: f32) {
        const PAN_SPEED: f32 = 100.0;
        if olc::get_key(olc::Key::A).held {
            self.offset.x += PAN_SPEED * elapsed_time;
        }
        if olc::get_key(olc::Key::D).held {
            self.offset.x -= PAN_SPEED * elapsed_time;
        }
        if olc::get_key(olc::Key::W).held {
            self.offset.y += PAN_SPEED * elapsed_time;
        }
        if olc::get_key(olc::Key::S).held {
            self.offset.y -= PAN_SPEED * elapsed_time;
        }
    }

    fn update(&mut self, elapsed_time: f32) {
        self.tick();
    }

    fn draw(&mut self) {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let mut p = olc::Pixel::rgb(255u8, 255u8, 255u8);
                if *self.get_cell(x as usize, y as usize) == 1 {
                    p = olc::Pixel::rgb(255u8, 140u8, 140u8);
                }
                let offset_x = (self.offset.x + x as f32) as i32;
                let offset_y = (self.offset.y + y as f32) as i32;
                olc::draw(offset_x, offset_y, p);
            }
        }
    }
}

mod tests {
    use super::Gof;
    #[test]
    fn test_neighbors() {
        let mut gof = Gof::new();
        for x in 0..3 {
            for y in 0..3 {
                gof.set_cell(x, y, 1);
            }
        }
        let result = gof.neighbors(1, 1);
        assert_eq!(result, 8);

        for x in 0..3 {
            for y in 0..3 {
                gof.set_cell(x, y, 0);
            }
        }
        for y in 0..3 {
            gof.set_cell(0, y, 1);
        }
        let result = gof.neighbors(1, 1);
        assert_eq!(result, 3);
    }
}
