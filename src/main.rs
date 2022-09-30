extern crate olc_pixel_game_engine;

mod gof;
mod common;
use common::Component;
use gof::Gof;
use crate::olc_pixel_game_engine as olc;

struct Game {
  pub gof: Gof,
  pub timer: f32,
}

impl olc::Application for Game {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::Pixel::rgb(255u8, 255u8, 255u8));
    self.gof.update(elapsed_time);
    self.gof.draw();
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}
fn main() {
  let mut game = Game {
    gof: Gof::new(),
    timer: 0.0
  };
  olc::start("Hello, World!", &mut game, 150, 150, 1, 1).unwrap();
}
