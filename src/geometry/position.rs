use std::fmt::{Display, Formatter};

pub type CoordsType = usize;

#[derive(Copy, Clone, Debug, Hash)]
pub struct Position {
  pub x: CoordsType,
  pub y: CoordsType,
  pub z: CoordsType,
}


impl Position {
  pub fn new(x: CoordsType, y: CoordsType, z: CoordsType) -> Position {
    Position {
      x,
      y,
      z
    }
  }

  pub fn set(&mut self, x: CoordsType, y: CoordsType, z: CoordsType) {
    self.x = x;
    self.y = y;
    self.z = z
  }

  pub fn change_cords(&mut self, dx: CoordsType, dy: CoordsType, dz: CoordsType) {
    self.x += dx;
    self.y += dy;
    self.z += dz;
  }

  pub fn get_x(&self) -> CoordsType {
    self.x
  }

  pub fn get_y(&self) -> CoordsType {
    self.y
  }

  pub fn get_z(&self) -> CoordsType {
    self.z
  }
}

impl Display for Position {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}
