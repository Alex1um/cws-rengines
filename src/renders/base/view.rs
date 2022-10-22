use std::rc::Rc;
use crate::geometry::position::Position;
use crate::objects::area::{AreaRef};

pub struct View {
  pos: Position,
  height: usize,
  width: usize,
  layers: usize,
}

impl View {
  pub fn new(pos: Position, width: usize, height: usize, layers: usize) -> Self {
    View {
      pos,
      height,
      width,
      layers
    }
  }

  pub fn get_width(&self) -> usize { self.width }
  pub fn get_height(&self) -> usize { self.height }
  pub fn get_layers(&self) -> usize { self.layers }

  pub fn get_pos(&self) -> Position { self.pos }
}
