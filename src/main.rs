extern crate olc_pixel_game_engine;

mod gof;
mod common;
mod component_handler;
use std::ops::Deref;

use common::Component;
use gof::Gof;
use component_handler::ComponentHandler;
use crate::olc_pixel_game_engine as olc;

struct Game {
  pub component_handler: ComponentHandler,
}

impl olc::Application for Game {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.component_handler.add_component(Gof::new());
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    //olc::clear(olc::Pixel::rgba(255u8, 255u8, 255u8, 255u8));
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
  olc::start("Hello, World!", &mut game, 100, 100, 1, 1).unwrap();
}
