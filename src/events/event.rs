use std::any::Any;
use std::hash::{Hash, Hasher};

pub enum Event {
  KeyBoardButtonDown {
    key: i32,
  },
  KeyBoardButtonUp {
    key: i32,
  },
  MouseButtonDown {
    key: i32,
    x: i32,
    y: i32,
  },
  MouseButtonUp {
    key: i32,
    x: i32,
    y: i32,
  },
  MouseWheel {
    x_dir: i32,
    y_dir: i32,
    x: i32,
    y: i32,
  },
  MouseMotion {
    x: i32,
    y: i32,
    x_rel: i32,
    y_rel: i32,
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

impl Hash for Event {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Event::KeyBoardButtonDown { key } | Event::KeyBoardButtonUp {key} => {
        // k.hash(state);
        key.hash(state);
        std::mem::discriminant(self).hash(state);
      }
      Event::MouseButtonDown {key, ..} | Event::MouseButtonUp { key, .. } => {
        key.hash(state);
        std::mem::discriminant(self).hash(state);
      }
      Event::Custom { r#type: d, .. } => {
        d.hash(state);
        std::mem::discriminant(self).hash(state);
      }
      _ => {
        std::mem::discriminant(self).hash(state);
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
