use std::hash::{Hash, Hasher};
use crate::geometry::position::Position;

pub enum Event {
  KeyBoard {
    key: i32,
  },
  Mouse {
    key: i32,
    pos: Position,
  },
  Custom {
    r#type: i32,
    data: Box<[u8]>,
  },
  ServerSync {
    data: Box<[u8]>,
  },
  Message {
    data: Box<[u8]>,
  },
  Loop,
}

impl Hash for Event {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      self::Event::Custom { r#type: d, data: _ } => {
        d.hash(state);
        3.hash(state);
      }
      self::Event::KeyBoard { key: _ } => {
        // k.hash(state);
        1.hash(state);
      }
      self::Event::Mouse { key: _, pos: _ } => {
        // k.hash(state);
        2.hash(state);
      }
      self::Event::ServerSync { data: _ } => {
        4.hash(state);
      }
      self::Event::Message { data: _ } => {
        5.hash(state);
      }
      self::Event::Loop => {
        0.hash(state);
      }
    }
  }
}

// compare variants not values
impl PartialEq<Self> for Event {
  fn eq(&self, other: &Self) -> bool {
    return std::mem::discriminant(self) == std::mem::discriminant(other) && match (self, other) {
      (Event::Custom { r#type: t, data: _ },
        Event::Custom { r#type: t2, data: _ }) => {
        t == t2
      }
      _ => { true }
    };
  }
}

impl Eq for Event {}
