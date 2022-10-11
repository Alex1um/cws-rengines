pub mod area;
pub mod game_object;

#[cfg(test)]
mod objects_tests {
  use std::rc::Rc;
  use crate::geometry::position::Position;
  use crate::objects::area::*;
  use crate::objects::game_object::*;

  #[test]
  fn area_insert() {
    let mut area = Area::new(2, 2, 2).create_ref();
    let r = GameObject::new(0, Position::new(0, 0, 0)).create_ref();
    area.borrow_mut().insert(Rc::clone(&r)).expect("successful object inser");
    assert_eq!(area.borrow().get(0, 0, 0).is_none(), false);
    assert_eq!(area.borrow().get(1, 1, 1).is_none(), true);
  }

  #[test]
  fn area_pop() {
    let mut area = Area::new(2, 2, 2).create_ref();
    let r = GameObject::new(0, Position::new(0, 0, 0)).create_ref();
    area.borrow_mut().insert(Rc::clone(&r)).expect("successful object inser");
    assert_eq!(area.borrow().get(0, 0, 0).is_none(), false);
    let obj = area.borrow_mut().pop(0, 0, 0).expect("not none");
  }

  #[test]
  #[should_panic]
  fn area_pop_none() {
    let mut area = Area::new(2, 2, 2);
    let obj = area.pop(0, 0, 0).expect("not none");
  }
}
