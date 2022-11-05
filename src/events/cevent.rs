use std::any::Any;
use std::ffi::{c_char, c_void, CStr, CString};
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
  data: *const c_void,
}

#[repr(C)]
struct CEventServerSync {
  data: *const c_char,
}

#[repr(C)]
struct CEventMessage {
  data: *const c_char,
}

#[repr(C)]
struct CEventFileInput {
  file_name: *const c_char,
}

#[repr(C)]
struct CEventCommand {
  command: *const c_char,
}

#[repr(C)]
pub union CEventContainer {
  keyboard: ManuallyDrop<CEventKeyboard>,
  mouse: ManuallyDrop<CEventMouse>,
  custom: ManuallyDrop<CEventCustom>,
  server_sync: ManuallyDrop<CEventServerSync>,
  server_msg: ManuallyDrop<CEventMessage>,
  file_input: ManuallyDrop<CEventFileInput>,
  command: ManuallyDrop<CEventCommand>,
}

#[repr(i32)]
pub enum CEventType {
  Keyboard,
  Mouse,
  Custom,
  Sync,
  Msg,
  FileInput,
  Command,
  Loop,
  Exit,
}

#[repr(C)]
pub struct CEvent {
  r#type: CEventType,
  event: CEventContainer,
}


impl Event {
  pub(crate) fn from_c(ce: CEvent) -> Event {
    unsafe {
      match ce {
        CEvent { r#type: CEventType::Keyboard, event: cec } => {
          Event::KeyBoard {
            key: cec.keyboard.key,
          }
        }
        CEvent { r#type: CEventType::Mouse, event: cec } => {
          Event::Mouse {
            pos: Position::new(cec.mouse.x as usize, cec.mouse.y as usize, 0),
            key: cec.mouse.key,
          }
        }
        CEvent { r#type: CEventType::Custom, event: cec } => {
          Event::Custom {
            data: Box::new(cec.custom.data),
            r#type: cec.custom.r#type,
          }
        }
        CEvent { r#type: CEventType::Sync, event: cec } => {
          Event::ServerSync {
            data: CStr::from_ptr(cec.server_sync.data).to_bytes().to_vec(),
          }
        }
        CEvent { r#type: CEventType::Msg, event: cec } => {
          Event::Message {
            data: CStr::from_ptr(cec.server_sync.data).to_bytes().to_vec(),
          }
        }
        CEvent { r#type: CEventType::Loop, .. } => { Event::Loop }
        CEvent { r#type: CEventType::FileInput, event: cec } => {
          Event::FileInput {
            file_name: CStr::from_ptr(cec.file_input.file_name).to_str().expect("correct str convertation").to_string(),
          }
        }
        CEvent { r#type: CEventType::Command, event: cec } => {
          Event::Command {
            command: CStr::from_ptr(cec.command.command).to_str().expect("correct str convertation").to_string()
          }
        }
        CEvent { r#type: CEventType::Exit, .. } => {
          Event::Exit
        }
      }
    }
  }

  pub(crate) fn to_c(&self) -> CEvent {
    match self {
      Event::Custom { r#type, data } => {
        CEvent {
          r#type: CEventType::Custom,
          event: CEventContainer {
            custom: ManuallyDrop::new(CEventCustom {
              r#type: *r#type,
              data: data as *const Box<dyn Any> as *const c_void,
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
              data: CStr::from_bytes_with_nul(data).expect("correct bytes").as_ptr(),
            })
          },
        }
      }
      Event::Message { data } => {
        CEvent {
          r#type: CEventType::Msg,
          event: CEventContainer {
            server_msg: ManuallyDrop::new(CEventMessage {
              data: CStr::from_bytes_with_nul(data).expect("correct bytes").as_ptr(),
            })
          },
        }
      }
      Event::Loop => {
        CEvent {
          r#type: CEventType::Loop,
          event: unsafe { std::mem::zeroed() },
        }
      }
      Event::FileInput { file_name } => {
        CEvent {
          r#type: CEventType::FileInput,
          event: CEventContainer {
            file_input: ManuallyDrop::new(CEventFileInput {
              file_name: CString::new(file_name.to_string()).expect("correct rust to c string conversion").into_raw()
            })
          }
        }
      }
      Event::Command { command } => {
        CEvent {
          r#type: CEventType::Command,
          event: CEventContainer {
            command: ManuallyDrop::new(CEventCommand {
              command: CString::new(command.to_string()).expect("correct rust to c string conversion").into_raw()
            })
          }
        }
      }
      Event::Exit => {
        CEvent {
          r#type: CEventType::Exit,
          event: unsafe { std::mem::zeroed() }
        }
      }
    }
  }
}