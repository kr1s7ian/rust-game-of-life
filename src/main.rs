extern crate olc_pixel_game_engine;

mod gof;
mod common;
mod component_handler;

use common::{Component, Vec2};
use gof::Gof;
use component_handler::ComponentHandler;
use crate::olc_pixel_game_engine as olc;

pub struct Game {
  pub component_handler: ComponentHandler,
  pub offset: Vec2<i32>,
}

impl olc::Application for Game {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.component_handler.add_component(Gof::new());
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::Pixel::rgb(140u8, 140u8, 140u8));
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
    offset: Vec2{x: 0, y: 0},
  };
  olc::start("Hello, World!", &mut game, 1920/10, 1080/10, 1, 1).unwrap();
}
