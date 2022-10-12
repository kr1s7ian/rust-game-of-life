use crate::common::{Component, Vec2};
use crate::olc_pixel_game_engine as olc;

use rand::Rng;
use rand::distributions::Standard;
use rand::prelude::Distribution;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell{
    Alive,
    Dead,
}

impl Cell {
    pub fn as_u8(&self) -> u8 {
        match self {
            Cell::Alive => return 1,
            Cell::Dead => return 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        match self {
            Cell::Alive => return true,
            Cell::Dead => return false,
        }
    }

    pub fn is_dead(&self) -> bool {
        return ! self.is_alive()
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized> (&self, rng: &mut R) -> Cell {
        match rng.gen_range(0..=1) {
            0 => Cell::Dead,
            1 => Cell::Alive,
            _ => Cell::Alive,
        }
    }
}

#[derive(Clone)]
pub struct Gof {
    content: Vec<Cell>,
    dimensions: Vec2<usize>,
    offset: Vec2<f32>,
    zoom: i32,
    running: bool,
}

impl Gof {
    pub fn new() -> Self {
        let height: usize = olc::screen_height() as usize;
        let width: usize = olc::screen_width() as usize;
        let vec_length: usize = width * height;

        let mut result = Self {
            dimensions: Vec2 {
                x: width,
                y: height,
            },
            content: vec![Cell::Dead; vec_length],
            offset: Vec2 { x: 0.0, y: 0.0 },
            zoom: 1,
            running: true,
        };
        result.init();

        result
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Cell {
        let i: usize = y * self.dimensions.x + x;
        return self.content[i];
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: Cell){
        let i: usize = y * self.dimensions.x + x;
        self.content[i] = value;
    }

    pub fn fill_cells(&mut self, state: Cell) {
        self.content.iter_mut().for_each(|f| *f = state);
    }

    pub fn randomize_cells(&mut self) {
        let mut rng = rand::thread_rng();
        for cell in self.content.iter_mut() {
            let random_state = rng.gen();
            *cell = random_state;
        }
    }

    pub fn cell_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut alive_neighbors = 0u8;

        for i in 0..3 {
            for j in 0..3 {
                let x = x - 1 + i;
                let y = y - 1 + j;
                let cell = self.get_cell(x, y);
                alive_neighbors += cell.as_u8();
            }
        }

        alive_neighbors - self.get_cell(x, y).as_u8()
    }

    pub fn advance_generation(&mut self) {
        let mut next_gen = self.clone();
        for x in 1..self.dimensions.x - 1 {
            for y in 1..self.dimensions.y - 1 {
                let neighbors = self.cell_neighbors(x, y);
                let cell = self.get_cell(x, y);

                if cell.is_dead() && neighbors == 3 {
                    next_gen.set_cell(x, y, Cell::Alive);
                } else if cell.is_alive() && (neighbors < 2 || neighbors > 3) {
                    next_gen.set_cell(x, y, Cell::Dead);
                } else {
                    next_gen.set_cell(x, y, cell);
                }
            }
        }
        self.content = next_gen.content;
    }

    #[inline]
    pub fn screen_to_world_point(&self, x: i32, y: i32) -> Vec2<i32> {
        let world_x = (x / self.zoom) - self.offset.x as i32;
        let world_y = (y / self.zoom) - self.offset.y as i32;

        Vec2{x: world_x, y: world_y}
    }
}

impl Component for Gof {

    fn init(&mut self) {
        self.randomize_cells();
    }

    fn poll_inputs(&mut self, elapsed_time: f32) {
        // toggle pause simulation when SPACEBAR is pressed
        if olc::get_key(olc::Key::SPACE).pressed {
            self.running = !self.running;
        }

        // randomize cells when R is pressed
        if olc::get_key(olc::Key::R).pressed {
            self.randomize_cells();
        }

        // set every cell to 0 when C is pressed
        if olc::get_key(olc::Key::C).pressed {
            self.fill_cells(Cell::Dead);
        }

        // simulate 1 frame if RIGHT is pressed
        if olc::get_key(olc::Key::RIGHT).pressed {
            self.advance_generation();
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
            if self.intersects(coords.x as usize, coords.y as usize) {
                self.set_cell(coords.x as usize, coords.y as usize, Cell::Alive);
            }
        }
    }

    fn update(&mut self, _elapsed_time: f32) {
        if self.running {
            self.advance_generation();
        }
    }

    fn draw(&self) {
        const WHITE: olc::Pixel = olc::Pixel::rgb(255u8, 255u8,255u8);
        const RED: olc::Pixel = olc::Pixel::rgb(255u8, 140u8,140u8);

        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                let cell = self.get_cell(x as usize, y as usize);
                let mut color = WHITE;
                if cell.is_alive() { color = RED }

                let world_x = (x as i32 + self.offset.x as i32) * self.zoom as i32;
                let world_y = (y as i32 + self.offset.y as i32) * self.zoom as i32;
                olc::fill_rect(world_x as i32, world_y as i32,self.zoom as i32, self.zoom as i32, color);
            }
        }
    }

    fn intersects(&mut self, x: usize, y: usize) -> bool {
        (x >= 0 && x < self.dimensions.x) && (y >= 0 && y < self.dimensions.y)
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::gof::{Gof, Cell};

    #[test]
    fn test_neighbors() {
        let mut gof = Gof::new();
        for x in 0..3 {
            for y in 0..3 {
                gof.set_cell(x, y, Cell::Alive);
            }
        }
        let result = gof.cell_neighbors(1, 1);
        assert_eq!(result, 8);

        for x in 0..3 {
            for y in 0..3 {
                gof.set_cell(x, y, Cell::Dead);
            }
        }
        for y in 0..3 {
            gof.set_cell(0, y, Cell::Alive);
        }
        let result = gof.cell_neighbors(1, 1);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_cell_is_alive() {
        let cell = Cell::Alive;
        assert_eq!(cell.is_alive(), true);

        let cell = Cell::Dead;
        assert_eq!(cell.is_alive(), false);
    }

    #[test]
    fn test_cell_is_dead() {
        let cell = Cell::Dead;
        assert_eq!(cell.is_dead(), true);

        let cell = Cell::Alive;
        assert_eq!(cell.is_dead(), false);
    }
}
