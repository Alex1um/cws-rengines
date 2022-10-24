use crate::renders::sdl::scene::{SceneRef};

pub trait Render {
  fn render(&mut self, scene: &SceneRef);
}