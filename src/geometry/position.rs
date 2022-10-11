use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct Position {
  pub x: usize,
  pub y: usize,
  pub z: usize,
}


impl Position {
  pub fn new(x: usize, y: usize, z: usize) -> Position {
    Position {
      x,
      y,
      z
    }
  }

  pub fn set(&mut self, x: usize, y: usize, z: usize) {
    self.x = x;
    self.y = y;
    self.z = z
  }

  pub fn change_cords(&mut self, dx: usize, dy: usize, dz: usize) {
    self.x += dx;
    self.y += dy;
    self.z += dz;
  }

  pub fn get_x(&self) -> usize {
    self.x
  }

  pub fn get_y(&self) -> usize {
    self.y
  }

  pub fn get_z(&self) -> usize {
    self.z
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}
