use crate::renders::sdl::render::{SceneRef};

pub trait Render {
  fn render(&mut self, scene: &SceneRef);
}