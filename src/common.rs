
#[derive(Debug, Clone)]
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
}
