pub mod position;
pub mod rect;
pub mod size;

#[cfg(test)]
mod tests {
  use crate::geometry::position::Position;

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
