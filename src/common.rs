use std::default;


#[derive(Debug, Clone, Default)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

pub trait Component {
  fn start(&mut self) {

  }

  fn draw(&mut self) {

  }

  fn update(&mut self, elapsed_time: f32) {

  }

  fn id(&self) -> Option<u32>{
    None
  }
}
