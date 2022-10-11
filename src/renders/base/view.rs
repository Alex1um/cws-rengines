use std::cell::RefCell;
use std::rc::Rc;
use crate::geometry::position::Position;
use crate::objects::area::{Area, AreaRef};
use crate::objects::game_object::GameObjectRef;

pub struct View {
  area: AreaRef,
  pos: Position,
  height: usize,
  width: usize,
  layers: usize,
}

impl View {
  pub fn new(area: &AreaRef, pos: Position, width: usize, height: usize, layers: usize) -> Self {
    View {
      area: Rc::clone(area),
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

  pub fn get_game_object(&self, pos: Position) -> Option<GameObjectRef> {
    return match self.area.borrow().get_by_pos(pos) {
      None => { None }
      Some(go) => {
        Some(Rc::clone(go))
      }
    };
  }
}
