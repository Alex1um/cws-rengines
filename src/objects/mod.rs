pub mod area;
pub mod game_object;

#[cfg(test)]
mod objects_tests {
  use crate::geometry::position::Position;
  use crate::objects::area::*;

  #[test]
  fn area_insert() {
    let mut area = Area::new(2, 2, 2);
    area.try_set_pos(Position::new(0, 0, 0), Some(1)).expect("successful object inser");
    assert_eq!(area.get(0, 0, 0).is_none(), false);
    assert_eq!(area.get(1, 1, 1).is_none(), true);
  }

  #[test]
  fn area_pop() {
    let mut area = Area::new(2, 2, 2);
    area.try_set_pos(Position::new(0, 0, 0), Some(0)).expect("successful object inser");
    assert_eq!(area.get(0, 0, 0).is_none(), false);
    let _ = area.remove(0, 0, 0).expect("not none");
  }

  #[test]
  #[should_panic]
  fn area_pop_none() {
    let mut area = Area::new(2, 2, 2);
    let _ = area.remove(0, 0, 0).expect("not none");
  }
}
