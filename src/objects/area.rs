use std::any::Any;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::rc::Rc;
use crate::geometry::position::Position;
use super::game_object::GameObject;
use rustc_hash::FxHashMap;

pub mod errors {
  use std::error::Error;
  use super::{Position, Area};
  use std::fmt::{Display, Formatter, write};

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
use crate::objects::game_object::{GameObjectID, GameObjectRef};

pub type AreaRef = Rc<RefCell<Area>>;

pub struct Area {
  sx: usize,
  sy: usize,
  sz: usize,
  area: Vec<Vec<Vec<Option<GameObjectID>>>>,
  objects: FxHashMap<GameObjectID, GameObject>,
  _obj_id: GameObjectID,
}

impl Area {
  pub fn new(sx: usize, sy: usize, sz: usize) -> Area {
    Area {
      sx,
      sy,
      sz,
      area: vec![vec![vec![Option::<GameObjectID>::None; sx]; sy]; sz],
      objects: FxHashMap::<GameObjectID, GameObject>::default(),
      _obj_id: GameObjectID::default(),
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

  pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&GameObject> {
    if let Some(id) =  self.area[z][y][x] {
        return self.objects.get(&id);
    }
    return None;
  }

  pub fn get_by_pos(&self, pos: Position) -> Option<&GameObject> {
    if let Some(id) = self.area[pos.get_z()][pos.get_y()][pos.get_x()] {
      return self.objects.get(&id);
    }
    return None;
  }

  pub fn remove(&mut self, x: usize, y: usize, z: usize) -> Option<GameObjectID> {
    self.area[z][y][x].take()
  }

  pub fn pop(&mut self, obj: GameObjectID) -> Option<GameObject> {
    self.objects.remove(&obj)
  }

  fn check_pos(&self, pos: &Position) -> bool {
    return pos.get_x() < self.sx && pos.get_y() < self.sy && pos.get_z() < self.sz;
  }

  pub fn update_object(&mut self, old_pos: Position, new_pos: Position) {
    let tmp = self.area[old_pos.z][old_pos.y][old_pos.x].take();
    self.area[old_pos.z][old_pos.y][old_pos.x] = self.area[new_pos.z][new_pos.y][new_pos.x].take();
    self.area[new_pos.z][new_pos.y][new_pos.x] = tmp;

  }

  pub fn create_ref(self) -> AreaRef {
    Rc::new(RefCell::new(self))
  }

  pub fn insert(&mut self, go: GameObject) -> Result<GameObjectID, Box<dyn Error>> {
    let pos = go.get_pos();
    let id = self._obj_id;
    self._obj_id += 1;
    self.objects.insert(self._obj_id, go);
    match self.get_by_pos(pos) {
      None => {
        self.area[pos.z][pos.y][pos.x] = Some(id);
        Ok(id)
      }
      Some(_) => { return Err(Box::new(FoundObjectWhileInserting(pos))); }
    }
  }
}
