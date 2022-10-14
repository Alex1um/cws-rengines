use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use crate::geometry::position::Position;
use super::area::*;
use crate::objects::area::errors;
use std::error::Error as TError;
use std::fmt::Error;

pub type GameObjectID = u32;
pub type GameObjectRef = u32;

pub struct GameObject {
  r#type: i32,
  pos: Position,
}

impl GameObject {

  pub fn new(r#type: i32, position: Position,) -> GameObject {
    GameObject {
      r#type,
      pos: position,
    }
  }

  pub fn get_pos(&self) -> Position {
    return self.pos;
  }

  pub fn set_pos(&mut self, pos: Position) {
    self.pos = pos;
  }

  pub fn get_type(&self) -> i32 {
    return self.r#type;
  }
}
