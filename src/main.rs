extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;

mod gof;
mod common;
mod component_handler;

use gof::Gof;
use component_handler::ComponentHandler;

pub struct Game {
  pub component_handler: ComponentHandler,
}

impl olc::Application for Game {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.component_handler.add_component(Gof::new());
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::Pixel::rgb(255u8, 255u8, 255u8));

    self.component_handler.poll_inputs(elapsed_time);
    self.component_handler.update(elapsed_time);
    self.component_handler.draw();
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}
fn main() {
  let mut game = Game {
    component_handler: ComponentHandler::new(),
  };
  olc::start("rust-game-of-life", &mut game, 1920, 1080, 1, 1).unwrap();
}
