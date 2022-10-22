use crate::renders::sdl::render::Scene;

pub trait Render {
  fn render(&mut self, scene: &Scene);
}