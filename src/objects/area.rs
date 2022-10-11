use std::any::Any;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::mem::{swap, take};
use std::rc::Rc;
use sdl2::libc::pollfd;
use crate::geometry::position::Position;
use super::game_object::GameObject;

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
  area: Vec<Vec<Vec<Option<Rc<RefCell<GameObject>>>>>>,
}

impl Area {
  pub fn new(sx: usize, sy: usize, sz: usize) -> Area {
    // let mut area: Vec<Vec<Vec<Cell<Option<GameObjectRef>>>>> = Vec::new();
    // for z in 0..sz {
    //   area.push(Vec::new());
    //   for y in 0..sy {
    //     area[z].push(Vec::new());
    //     for z in 0..sz {
    //       area[z][y].push(Cell::new(Option::<GameObjectRef>::None));
    //     }
    //   }
    // }
    Area {
      sx,
      sy,
      sz,
      area: vec![vec![vec![Option::<GameObjectRef>::None; sx]; sy]; sz],
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

  pub fn get(&self, x: usize, y: usize, z: usize) -> &Option<GameObjectRef> {
    &self.area[z][y][x]
  }

  pub fn get_by_pos(&self, pos: Position) -> &Option<GameObjectRef> {
    &self.area[pos.get_z()][pos.get_y()][pos.get_x()]
  }

  pub fn pop(&mut self, x: usize, y: usize, z: usize) -> Option<GameObjectRef> {
    self.area[z][y][x].take()
  }

  pub fn pop_object(&mut self, obj: &GameObject) -> Option<GameObjectRef> {
    let pos = obj.get_pos();
    self.area[pos.get_z()][pos.get_y()][pos.get_x()].take()
  }

  fn check_pos(&self, pos: &Position) -> bool {
    return pos.get_x() < self.sx && pos.get_y() < self.sy && pos.get_z() < self.sz;
  }

  pub fn update_object(&mut self, old_pos: Position, new_pos: Position) {
    let tmp = self.area[old_pos.z][old_pos.y][old_pos.x].take();
    // let p2 = self.area[new_pos.z][new_pos.y][new_pos.x];
    self.area[old_pos.z][old_pos.y][old_pos.x] = self.area[new_pos.z][new_pos.y][new_pos.x].take();
    self.area[new_pos.z][new_pos.y][new_pos.x] = tmp;

  }

  pub fn create_ref(self) -> AreaRef {
    Rc::new(RefCell::new(self))
  }

  pub fn insert(&mut self, go: GameObjectRef) -> Result<(), Box<dyn Error>> {
    let pos = go.borrow().get_pos();
    match self.get_by_pos(pos) {
      None => {
        self.area[pos.z][pos.y][pos.x] = Some(go);
        Ok(())
      }
      Some(_) => { return Err(Box::new(FoundObjectWhileInserting(pos))); }
    }
  }
}

// pub fn try_insert(area: &Rc<Area>, go_reg: &mut GameObject) -> Result<(), Box<dyn Error>> {
//   let mut flag = false;
//   let mut pos = Position::new(0, 0, 0);
//   {
//     pos = go_reg.get_pos();
//     if abm.check_pos(&pos) {
//       match abm.get_by_pos(pos) {
//         None => {
//           flag = true;
//         }
//         Some(_) => {
//         }
//       }
//     } else {
//       return Err(Box::new(PositionOutOfRange::new(pos, &(*abm))));
//     }
//   }
//   return if flag {
//     let _ = area.borrow_mut().area[pos.get_z()][pos.get_y()][pos.get_x()].insert(Rc::clone(go));
//     go_req.set_area(area);
//     Ok(())
//   } else {
//     Err(Box::new(FoundObjectWhileInserting(pos)))
//   }
// }
