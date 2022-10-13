use crate::geometry::position::Position;
use crate::objects::area::Area;
use crate::objects::game_object::GameObject;

pub mod geometry;
pub mod objects;
pub mod renders;

pub extern "C" fn create_object(pos: Position, r#type: i32) -> GameObject {
  return GameObject::new(r#type, pos);
}

pub extern "C" fn create_area(x: usize, y: usize, z: usize) -> Area {
  return Area::new(x, y, z);
}


#[cfg(test)]
mod tests {
  use super::*;
}
