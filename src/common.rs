
#[derive(Debug, Clone, Default)]
pub struct Vec2<T> {
  pub x: T,
  pub y: T,
}

pub trait Component {
  fn init(&mut self) {

  }

  fn draw(&self) {

  }

  fn poll_inputs(&mut self, _elapsed_time: f32) {

  }

  fn update(&mut self, _elapsed_time: f32) {

  }

  fn intersects(&mut self, x: usize, y: usize) -> bool {
    false
  }
}
