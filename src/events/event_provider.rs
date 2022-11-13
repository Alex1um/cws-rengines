use std::io::{stdin};
use crate::events::event::Event;
use std::ffi::{c_char, CStr, CString, c_int};

pub trait EventProvider {
  fn provide_events(&mut self, buf: &mut Vec<Event>);
}

impl<T> EventProvider for T
  where for<'r> T: FnMut(&'r mut Vec<Event>) {
  fn provide_events(&mut self, buf: &mut Vec<Event>) {
    self(buf);
  }
}

#[cfg(target_os = "emscripten")]
extern "C" {
  fn emscripten_run_script_string(script: *const c_char) -> *const c_char;
  fn emscripten_run_script_int(script: *const c_char) -> c_int;
}

#[cfg(target_os = "emscripten")]
const CONSOLE_INPUT_SCRIPT: &str = "get_console_input()\0";
#[cfg(target_os = "emscripten")]
const CONSOLE_INPUT_CHECK: &str = "check_console_input()\0";

#[cfg(target_os = "emscripten")]
pub fn console_input_command_provider(buf: &mut Vec<Event>) {
  unsafe {
    if emscripten_run_script_int(CONSOLE_INPUT_CHECK.as_ptr() as *const c_char) as i32 == 1 {
      let str = CStr::from_ptr(emscripten_run_script_string(CONSOLE_INPUT_SCRIPT.as_ptr() as *const c_char))
        .to_str().expect("correct console input command conversion")
        .to_string();
      buf.push(Event::Command { command: str });
    }
  }
}


#[cfg(target_os = "emscripten")]
const FILE_INPUT_SCRIPT: &str = "get_file_input()\0";
#[cfg(target_os = "emscripten")]
const FILE_INPUT_CHECK: &str = "check_file_input()\0";

#[cfg(target_os = "emscripten")]
pub fn file_input_provider(buf: &mut Vec<Event>) {
  unsafe {
    if emscripten_run_script_int(FILE_INPUT_CHECK.as_ptr() as *const c_char) as i32 == 1 {
      let str = CStr::from_ptr(emscripten_run_script_string(FILE_INPUT_SCRIPT.as_ptr() as *const c_char))
        .to_str().expect("correct file input command conversion")
        .to_string();
      buf.push(Event::FileInput { file_name: str });
    }
  }
}

#[cfg(not(target_os = "emscripten"))]
pub fn file_input_provider(_: &mut Vec<Event>) {

}

#[cfg(not(target_os = "emscripten"))]
pub fn console_input_command_provider(buf: &mut Vec<Event>) {

  let mut str = String::new();
  if stdin().read_line(&mut str).is_ok() {
    buf.push(Event::Command { command: str.to_string() });
  }
}