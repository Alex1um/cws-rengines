use std::any::Any;
use std::ffi::c_void;
use std::mem::ManuallyDrop;
use crate::events::event::Event;
use crate::geometry::position::Position;

#[repr(C)]
struct CEventKeyboard {
  key: i32,
}

#[repr(C)]
struct CEventMouse {
  key: i32,
  x: i32,
  y: i32,
}

#[repr(C)]
struct CEventCustom {
  r#type: i32,
  data: *const Box<dyn Any>,
}

#[repr(C)]
struct CEventServerSync {
  data: *const Box<dyn Any>,
}

#[repr(C)]
struct CEventMessage {
  data: *const Box<dyn Any>,
}

#[repr(C)]
struct CEventLoop {
  ticks: u64,
}

#[repr(C)]
pub union CEventContainer {
  keyboard: ManuallyDrop<CEventKeyboard>,
  mouse: ManuallyDrop<CEventMouse>,
  custom: ManuallyDrop<CEventCustom>,
  server_sync: ManuallyDrop<CEventServerSync>,
  server_msg: ManuallyDrop<CEventMessage>,
  r#loop: ManuallyDrop<CEventLoop>,
}

#[repr(i32)]
#[derive(Hash)]
pub enum CEventType {
  Keyboard,
  Mouse,
  Custom,
  Sync,
  Msg,
  Loop,
}

#[repr(C)]
pub struct CEvent {
  r#type: CEventType,
  event: CEventContainer,
}


impl Event {
  pub(crate) fn create_c(&self) -> CEvent {
    match self {
      Event::Custom { r#type, data } => {
        CEvent {
          r#type: CEventType::Custom,
          event: CEventContainer {
            custom: ManuallyDrop::new(CEventCustom {
              r#type: *r#type,
              data: data as *const Box<dyn Any>,
            })
          },
        }
      }
      Event::KeyBoard { key } => {
        CEvent {
          r#type: CEventType::Keyboard,
          event: CEventContainer {
            keyboard: ManuallyDrop::new(CEventKeyboard {
              key: *key
            })
          },
        }
      }
      Event::Mouse { key, pos } => {
        CEvent {
          r#type: CEventType::Mouse,
          event: CEventContainer {
            mouse: ManuallyDrop::new(CEventMouse {
              key: *key,
              x: pos.x as i32,
              y: pos.y as i32,
            })
          },
        }
      }
      Event::ServerSync { data } => {
        CEvent {
          r#type: CEventType::Sync,
          event: CEventContainer {
            server_sync: ManuallyDrop::new(CEventServerSync {
              data: data as *const Box<dyn Any>,
            })
          },
        }
      }
      Event::Message { data } => {
        CEvent {
          r#type: CEventType::Msg,
          event: CEventContainer {
            server_msg: ManuallyDrop::new(CEventMessage {
              data: data as *const Box<dyn Any>,
            })
          },
        }
      }
      Event::Loop => {
        CEvent {
          r#type: CEventType::Loop,
          event: CEventContainer {
            r#loop: ManuallyDrop::new(CEventLoop {
              ticks: 0
            }),
          },
        }
      }
    }
  }
}