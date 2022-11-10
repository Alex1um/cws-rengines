use crate::geometry::relative_component::RelativeComponent;
use crate::geometry::size::RelativeSize::{Absolute, Percent, Shift};

#[derive(Clone, Copy)]
pub enum RelativeSize {
  Percent(f32),
  Absolute(usize),
  Shift(isize),
}

impl RelativeComponent<usize, usize> for RelativeSize {
  fn get_absolute(&self, t: &usize) -> usize {
    return match self {
      RelativeSize::Percent(p) => {
        (*t as f32 * p) as usize
      }
      RelativeSize::Absolute(c) => {
        *c
      }
      RelativeSize::Shift(s) => {
        (*t as isize + *s) as usize
      }
    }
  }

}

impl From<f32> for RelativeSize {
  fn from(p: f32) -> Self {
    RelativeSize::Percent(p)
  }
}

impl From<usize> for RelativeSize {
  fn from(a: usize) -> Self {
    RelativeSize::Absolute(a)
  }
}

impl From<isize> for RelativeSize {
  fn from(s: isize) -> Self {
    RelativeSize::Shift(s)
  }
}

pub struct RelativeSize2D(RelativeSize, RelativeSize);

impl RelativeComponent<(usize, usize), (usize, usize)> for RelativeSize2D {

  fn get_absolute(&self, t: &(usize, usize)) -> (usize, usize) {
    return (self.0.get_absolute(&t.0), self.1.get_absolute(&t.1));
  }
}

impl From<(f32, f32)> for RelativeSize2D {
  fn from(p: (f32, f32)) -> Self {
    RelativeSize2D(Percent(p.0), Percent(p.1))
  }
}

impl From<(isize, isize)> for RelativeSize2D {
  fn from(p: (isize, isize)) -> Self {
    RelativeSize2D(Shift(p.0), Shift(p.1))
  }
}

impl From<(usize, usize)> for RelativeSize2D {
  fn from(p: (usize, usize)) -> Self {
    RelativeSize2D(Absolute(p.0), Absolute(p.1))
  }
}

impl From<RelativeSize> for RelativeSize2D {
  fn from(s: RelativeSize) -> Self {
    RelativeSize2D(s, s)
  }
}
