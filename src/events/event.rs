use std::any::Any;
use std::hash::{Hash, Hasher};
use crate::geometry::position::Position;

pub enum Event {
  KeyBoard {
    key: i32,
  },
  MouseClick {
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
      Event::MouseClick { key, .. } => Event::MouseClick { key: *key, x: i32::default(), y: i32::default() },
      Event::MouseWheel { .. } => Event::MouseWheel { x: i32::default(), y: i32::default(), x_dir: i32::default(), y_dir: i32::default() },
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
      _ => {
        std::mem::discriminant(self).hash(state);
      }
      Event::KeyBoard { key } => {
        // k.hash(state);
        std::mem::discriminant(self).hash(state);
        key.hash(state);
      }
      Event::MouseClick { key, .. } => {
        std::mem::discriminant(self).hash(state);
        key.hash(state);
      }
      Event::Custom { r#type: d, .. } => {
        std::mem::discriminant(self).hash(state);
        d.hash(state);
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
