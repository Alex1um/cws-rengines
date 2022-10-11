pub mod position;
use position::Position;

#[cfg(test)]
mod tests {
  use std::rc::Rc;
  use super::*;

  #[test]
  fn pos_create() {
    Position::new(0, 0,0);
    Position::new(usize::MAX,  usize::MAX,usize::MAX);
  }

  #[test]
  #[should_panic]
  #[allow(arithmetic_overflow)]
  fn pos_invalid_create() {
    Position::new(usize::MAX + 1,  usize::MAX + 1,usize::MAX + usize::MAX);
  }
}
