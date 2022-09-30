extern crate olc_pixel_game_engine;

mod gof;
mod common;
use gof::Gof;
use crate::olc_pixel_game_engine as olc;

struct ExampleProgram {
  pub gof: Gof,
  pub timer: f32,
}

impl olc::Application for ExampleProgram {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.gof.set_cell(100, 100, 1);
    self.gof.randomize();
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::Pixel::rgb(255u8, 255u8, 255u8));

    for y in 0..olc::screen_width() {
      for x in 0..olc::screen_height() {
        let mut p = olc::Pixel::rgb(255u8, 255u8, 255u8);
          if *self.gof.get_cell(x as usize, y as usize) == 1 {
            p = olc::Pixel::rgb(255u8, 140u8, 140u8);
          }
        olc::draw(x, y, p);
      }
    }
    if self.timer > 0.1 {
      self.timer = 0.0;
      self.gof.tick();
    }
    self.timer += elapsed_time;

    if olc::get_key(olc::Key::SPACE).pressed {
      //self.gof.randomize();
      self.gof.tick();
    }
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}
fn main() {
  let mut example = ExampleProgram {
    gof: Gof::new(),
    timer: 0.0
  };
  olc::start("Hello, World!", &mut example, 150, 150, 1, 1).unwrap();
}
