use crate::common::{Component, Vec2};
use crate::{olc_pixel_game_engine as olc, Game};
use rand::Rng;

#[derive(Clone)]
pub struct Gof {
    content: Vec<u8>,
    dimensions: Vec2<usize>,
    offset: Vec2<f32>,
    zoom: i32,
    running: bool,
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
            offset: Vec2 { x: 0.0, y: 0.0 },
            zoom: 1,
            running: true,
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

    pub fn clear(&mut self) {
        self.content.iter_mut().for_each(|f| *f = 0);
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

    #[inline]
    pub fn screen_to_world_point(&self, x: i32, y: i32) -> Vec2<i32> {
        let world_x = (x / self.zoom) - self.offset.x as i32;
        let world_y = (y / self.zoom) - self.offset.y as i32;

        Vec2{x: world_x, y: world_y}
    }
}

impl Component for Gof {
    fn id(&self) -> Option<u32> {
        Some(0u32)
    }

    fn start(&mut self) {
        self.randomize();
        self.clear();
    }

    fn poll_inputs(&mut self, elapsed_time: f32) {
        // toggle pause simulation when SPACEBAR is pressed
        if olc::get_key(olc::Key::SPACE).pressed {
            self.running = !self.running;
        }

        // randomize cells when R is pressed
        if olc::get_key(olc::Key::R).pressed {
            self.randomize();
        }

        // set every cell to 0 when C is pressed
        if olc::get_key(olc::Key::C).pressed {
            self.clear();
        }

        // simulate 1 frame if RIGHT is pressed
        if olc::get_key(olc::Key::RIGHT).pressed {
            self.tick();
        }

        // change zoom with SCROLLWHEEL
        let scroll = olc::get_mouse_wheel();
        let old_pos = self.screen_to_world_point(olc::get_mouse_x(), olc::get_mouse_y());
        if scroll > 1 {
            self.zoom += 1;
        } else if scroll < -1 {
            if self.zoom > 1 {
                self.zoom -= 1;
            }
        }
        // calculate mouse position difference on then subtract it to
        // the rendering offset after each zoom to zoom under cursor
        let new_pos = self.screen_to_world_point(olc::get_mouse_x(), olc::get_mouse_y());
        let diff_x = old_pos.x - new_pos.x;
        let diff_y = old_pos.y - new_pos.y;
        println!("x: {}, y: {}", diff_x, diff_y);
        self.offset.x -= diff_x as f32;
        self.offset.y -= diff_y as f32;

        // pan around the world with WASD keys
        let pan_speed: f32 = 100.0 / (self.zoom as f32 * 0.25);
        if olc::get_key(olc::Key::A).held {
            self.offset.x += pan_speed * elapsed_time;
        }
        if olc::get_key(olc::Key::D).held {
            self.offset.x -= pan_speed * elapsed_time;
        }
        if olc::get_key(olc::Key::W).held {
            self.offset.y += pan_speed * elapsed_time;
        }
        if olc::get_key(olc::Key::S).held {
            self.offset.y -= pan_speed * elapsed_time;
        }

        // if left mouse button is being pressed, make the cell under the cursor alive
        if olc::get_mouse(0).held {
            let (mx, my) = (olc::get_mouse_x(), olc::get_mouse_y());
            let coords = self.screen_to_world_point(mx, my);
            if (coords.x > 0 && coords.x < self.dimensions.x as i32)
                && (coords.y > 0 && coords.y < self.dimensions.y as i32)
            {
                self.set_cell(coords.x as usize, coords.y as usize, 1u8);
            }
        }
    }

    fn update(&mut self, _elapsed_time: f32) {
        // update sim only of the sim is not paused
        if self.running {
            self.tick();
        }
    }

    fn draw(&mut self) {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let mut p = olc::Pixel::rgb(255u8, 255u8, 255u8);
                if *self.get_cell(x as usize, y as usize) == 1 {
                    p = olc::Pixel::rgb(255u8, 140u8, 140u8);
                }
                let world_x = (x as i32 + self.offset.x as i32) * self.zoom as i32;
                let world_y = (y as i32 + self.offset.y as i32) * self.zoom as i32;
                olc::fill_rect(world_x as i32, world_y as i32,self.zoom as i32, self.zoom as i32, p);

            }
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::gof::Gof;

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
