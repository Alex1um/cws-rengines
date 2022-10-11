use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use crate::geometry::position::Position;
use super::area::*;
use crate::objects::area::errors;
use std::error::Error as TError;
use std::fmt::Error;

pub type GameObjectID = usize;
pub type GameObjectRef = Rc<RefCell<GameObject>>;

pub struct GameObject {
  r#type: i32,
  pos: Position,
  area: Weak<RefCell<Area>>,
}

impl GameObject {

  pub fn new(r#type: i32, position: Position) -> GameObject {
    GameObject {
      r#type,
      pos: position,
      area: Weak::new(),
    }
  }

  pub fn get_pos(&self) -> Position {
    return self.pos;
  }

  pub fn get_type(&self) -> i32 {
    return self.r#type;
  }

  pub fn create_ref(self) -> GameObjectRef {
    Rc::new(RefCell::new(self))
  }

  pub fn set_area(&mut self, area: &AreaRef) {
    self.area = Rc::downgrade(area);
  }

  pub fn set_pos(&mut self, pos: Position) -> Result<(), Box<dyn TError>> {
    match self.area.upgrade() {
      None => {
        self.pos = pos;
        Ok(())
      }
      Some(mut area) => {
        area.borrow_mut().update_object( self.pos, pos);
        self.pos = pos;
        Ok(())
      }
    }
  }
}

//
// pub fn change_cords(mut obj: GameObjectRef, dx: usize, dy: usize, dz: usize) -> Result<(), Box<dyn TError>> {
//   obj.borrow_mut().pos.change_cords(dx, dy, dz);
//   let pos = obj.borrow().pos;
//   set_cords(obj, pos)
// }
//
// pub fn set_cords(mut obj: GameObjectRef, pos: Position) -> Result<(), Box<dyn TError>> {
//   let mut borrowed = obj.borrow_mut();
//   match borrowed.area.upgrade() {
//     None => {
//       borrowed.pos = pos;
//       Ok(())
//     }
//     Some(area) => {
//       try_insert(&area, &obj)?;
//       Ok(())
//     }
//   }
// }
