use std::any::Any;
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
    data: Box<dyn Any>,
  },
  ServerSync {
    data: Vec<u8>,
  },
  Message {
    data: Vec<u8>,
  },
  FileInput {
    file_name: String,
  },
  Command {
    command: String,
  },
  Loop,
  Exit,
}

impl Event {
  pub(crate) fn get_hasher(&self) -> Event {
    match self {
      Event::KeyBoard { key } => Event::KeyBoard { key: *key },
      Event::Mouse { key, pos } => Event::Mouse { key: *key, pos: *pos },
      Event::Loop => Event::Loop,
      Event::Custom { r#type, .. } => Event::Custom { r#type: *r#type, data: Box::new(()) },
      Event::ServerSync { .. } => Event::ServerSync { data: vec![] },
      Event::Message { .. } => Event::Message { data: vec![] },
      Event::FileInput { .. } => Event::FileInput { file_name: String::default() },
      Event::Command { .. } => Event::Command { command: String::default() },
      Event::Exit => Event::Exit,
    }
  }
}

impl Hash for Event {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Event::Loop => {
        0.hash(state);
      }
      Event::KeyBoard { key } => {
        // k.hash(state);
        key.hash(state);
        1.hash(state);
      }
      Event::Mouse { key: _, pos: _ } => {
        // k.hash(state);
        2.hash(state);
      }
      Event::Custom { r#type: d, data: _ } => {
        3.hash(state);
        d.hash(state);
      }
      Event::ServerSync { data: _ } => {
        4.hash(state);
      }
      Event::Message { data: _ } => {
        5.hash(state);
      }
      Event::FileInput { .. } => {
        6.hash(state);
      }
      Event::Command { .. } => {
        7.hash(state);
      }
      Event::Exit => {
        8.hash(state);
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
