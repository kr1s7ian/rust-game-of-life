use crate::common::{Component, Vec2};
use crate::olc_pixel_game_engine as olc;
use rand::Rng;

#[derive(Debug, Clone, Default)]
pub struct Gof {
    content: Vec<u8>,
    dimensions: Vec2<usize>,
    offset: Vec2<i32>
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
            offset: Vec2{x: 0, y: 0},

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

    pub fn set_offset(&mut self, offset: Vec2<i32>) {
        self.offset = offset;
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

    fn update(&mut self, elapsed_time: f32) {
        self.tick();
        self.draw();
    }

    fn draw(&mut self) {
        for y in 0..olc::screen_width() {
            for x in 0..olc::screen_height() {
                let mut p = olc::Pixel::rgb(255u8, 255u8, 255u8);
                if *self.get_cell(x as usize, y as usize) == 1 {
                    p = olc::Pixel::rgb(255u8, 140u8, 140u8);
                }
                olc::draw(self.offset.x + x, self.offset.y + y, p);
            }
        }
    }
}

mod tests {
    use super::*;

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
