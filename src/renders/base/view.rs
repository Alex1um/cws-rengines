use crate::geometry::position::Position;
use crate::geometry::size::ViewSize;

pub struct View {
  pos: Position,
  size: ViewSize,
  layers: usize,
  screen_pos: ViewSize,
  screen_size: ViewSize,
}

impl View {
  pub fn new(pos: Position,
             size: ViewSize,
             layers: usize,
             screen_pos: ViewSize,
             screen_size: ViewSize) -> Self {
    View {
      pos,
      size,
      layers,
      screen_pos,
      screen_size,
    }
  }

  pub fn get_size(&self, scene_size: (usize, usize)) -> (usize, usize) { self.size.get_in_px(scene_size) }
  pub fn get_layers(&self) -> usize { self.layers }

  pub fn get_pos(&self) -> Position { self.pos }
  pub fn get_screen_pos(&self, resolution: (usize, usize)) -> (usize, usize) { self.screen_pos.get_in_px(resolution) }
  pub fn get_screen_size(&self, resolution: (usize, usize)) -> (usize, usize) { self.screen_size.get_in_px(resolution) }
}
