use crate::common::Vec2;
use crate::olc_pixel_game_engine as olc;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Gof {
    content: Vec<u8>,
    dimensions: Vec2<usize>,
}

impl Gof {
    pub fn new() -> Self {
        let height: usize = (olc::screen_height()) as usize;
        let width: usize = (olc::screen_width()) as usize;
        let size: usize = width * height;
        Self {
            dimensions: Vec2 {
                x: width,
                y: height,
            },
            content: vec![0u8; size],
        }
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

    for x in 1..self.dimensions.x-1 {
      for y in 1..self.dimensions.y -1 {
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
