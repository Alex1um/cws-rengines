use crate::geometry::relative_component::RelativeComponent;
use crate::geometry::size::{RelativeSize, RelativeSize2D};

pub struct AbsoluteRect {
  x: usize,
  y: usize,
  width: usize,
  height: usize,
}

pub struct Rect {
  pub pos: RelativeSize2D,
  pub size: RelativeSize2D,
}

impl Rect {
  pub fn new(pos: RelativeSize2D, size: RelativeSize2D) -> Rect {
    Rect {
      pos,
      size,
    }
  }

  pub fn square(pos: RelativeSize2D, size: RelativeSize) -> Rect {
    Rect {
      pos,
      size: RelativeSize2D::from(size),
    }
  }
}

impl RelativeComponent<(usize, usize), ((usize, usize), (usize, usize))> for Rect {
  fn get_absolute(&self, t: &(usize, usize)) -> ((usize, usize), (usize, usize)) {
    return (self.pos.get_absolute(t), self.size.get_absolute(t));
  }
}
