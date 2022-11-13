use std::cell::{RefCell};
use std::cmp::min;
use std::error::Error;
use std::rc::Rc;
use crate::geometry::position::Position;

pub mod errors {
  use std::error::Error;
  use super::{Position, Area};
  use std::fmt::{Display, Formatter};

  #[derive(Debug)]
  pub struct PositionOutOfRange {
    pos: Position,
    sizes: (usize, usize, usize),
  }

  impl PositionOutOfRange {
    pub fn new(pos: Position, area: &Area) -> Self {
      PositionOutOfRange {
        pos,
        sizes: (area.sx, area.sy, area.sz),
      }
    }
  }

  impl Display for PositionOutOfRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "Given coordinates {} are out of area sizes: {:?}", self.pos, self.sizes)
    }
  }

  #[derive(Debug)]
  pub struct FoundObjectWhileInserting(pub Position);

  impl Display for FoundObjectWhileInserting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "At position {} found existing object", self.0)
    }
  }

  impl Error for FoundObjectWhileInserting {}

  impl Error for PositionOutOfRange {}
}

use errors::*;
use crate::objects::game_object::{GameObjectID};

pub type AreaRef = Rc<RefCell<Area>>;

pub struct Area {
  sx: usize,
  sy: usize,
  sz: usize,
  area: Vec<Vec<Vec<Option<GameObjectID>>>>,
}

impl Area {
  pub fn new(sx: usize, sy: usize, sz: usize) -> Area {
    Area {
      sx,
      sy,
      sz,
      area: vec![vec![vec![Option::<GameObjectID>::None; sx]; sy]; sz],
    }
  }

  pub fn get_size_x(&self) -> usize {
    self.sx
  }

  pub fn get_size_y(&self) -> usize {
    self.sy
  }

  pub fn get_size_z(&self) -> usize {
    self.sz
  }

  pub fn remove(&mut self, x: usize, y: usize, z: usize) -> Option<GameObjectID> {
    self.area[z][y][x].take()
  }

  fn check_pos_boundaries(&self, pos: &Position) -> bool {
    return pos.get_x() < self.sx && pos.get_y() < self.sy && pos.get_z() < self.sz;
  }

  pub fn get_pos(&self, pos: &Position) -> &Option<GameObjectID> {
    return &self.area[pos.z][pos.y][pos.x];
  }

  pub fn get(&self, x: usize, y: usize, z: usize) -> &Option<GameObjectID> {
    return &self.area[z][y][x];
  }

  pub fn swap_objects(&mut self, old_pos: Position, new_pos: Position) {
    let tmp = self.area[old_pos.z][old_pos.y][old_pos.x].take();
    self.area[old_pos.z][old_pos.y][old_pos.x] = self.area[new_pos.z][new_pos.y][new_pos.x].take();
    self.area[new_pos.z][new_pos.y][new_pos.x] = tmp;
  }

  pub fn set_pos(&mut self, pos: Position, id: Option<GameObjectID>) {
    self.area[pos.z][pos.y][pos.x] = id;
  }

  pub fn try_set_pos(&mut self, pos: Position, id: Option<GameObjectID>) -> Result<(), Box<dyn Error>> {
    if self.check_pos_boundaries(&pos) {
      match id {
        None => {
          self.area[pos.z][pos.y][pos.x] = id;
          return Ok(());
        }
        Some(_) => {
          if let Some(_) = self.get_pos(&pos) {
            return Err("Found Object at this position".into());
          } else {
            self.area[pos.z][pos.y][pos.x] = id;
            return Ok(());
          }
        }
      }
    }
    return Err(Box::new(PositionOutOfRange::new(pos, self)));
  }

  pub fn create_ref(self) -> AreaRef {
    Rc::new(RefCell::new(self))
  }

  pub fn resize(&mut self,
  sx: usize,
  sy: usize,
  sz: usize) {
    let mut new_area = vec![vec![vec![Option::<GameObjectID>::None; sx]; sy]; sz];
    for z in 0..min(sz, self.sz) {
      for y in 0..min(sy, self.sy) {
        for x in 0..min(sx, self.sx) {
          new_area[z][y][x] = self.area[z][y][x].take();
        }
      }
    }
    self.area = new_area;
    self.sx = sx;
    self.sy = sy;
    self.sz = sz;
  }

}
