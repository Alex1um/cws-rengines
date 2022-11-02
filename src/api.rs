use crate::geometry::position::{CoordsType, Position};
use crate::objects::area::{Area};
use crate::objects::game_object::{GameObject, GameObjectID};
use crate::renders::base::screen::Screen;
use crate::renders::base::view::View;
use std::boxed::Box;
use std::ffi::{c_char, CStr, CString};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::{Read};
use std::rc::Rc;
use crate::events::cevent::CEvent;
use crate::events::event::Event;
use crate::events::event_loop::EventLoop;
use crate::renders::sdl::render::SDLRender;
use crate::renders::sdl::scene::{Scene, SceneRef};
use crate::renders::sdl::window::{Window, WindowRef};


// type ScenePtr<'a> = *const RefCell<Scene<'a>>;
// type WindowPtr = *const RefCell<Window>;

#[no_mangle]
pub extern "C" fn create_object(area: &SceneRef, x: i32, y: i32, z: i32, r#type: i32) -> GameObjectID {
  // let area = SceneRef::from_raw(area);
  let obj = GameObject::new(r#type, Position::new(x as CoordsType, y as CoordsType, z as CoordsType));
  return area.borrow_mut().add_object(obj).expect("Successful adding");
}

#[no_mangle]
pub extern "C" fn remove_object(scene: &SceneRef, obj_id: GameObjectID) {
  scene.borrow_mut().remove_object(obj_id);
}

#[no_mangle]
extern "C" fn create_scene(x: usize, y: usize, z: usize) -> SceneRef<'static> {
  let rf = Area::new(x, y, z);
  let scene = Scene::new(rf);
  return scene;
}

#[no_mangle]
extern "C" fn create_window(res_x: i32, res_y: i32) -> WindowRef {
  return Window::new(res_x as usize, res_y as usize)
    .expect("Created window")
    .create_ref();
}

#[no_mangle]
extern "C" fn testing() {
  println!("HELLO from rust");
}

#[no_mangle]
unsafe extern "C" fn load_texture(scene: &SceneRef, window: &WindowRef, path: *const c_char) {
  // let scene = Rc::from_raw(ctx);
  // let window = Rc::from_raw(win);
  let path = &CStr::from_ptr(path).to_str().expect("correct string");
  scene.borrow_mut().load_texture(window.borrow().get_texture_creator(), &path);
}

#[no_mangle]
extern "C" fn clone_scene<'a>(scene: &SceneRef<'a>) -> SceneRef<'a> {
  return Rc::clone(scene);
}

#[no_mangle]
extern "C" fn create_event_loop<'a>(scene: &SceneRef<'a>, win: WindowRef) -> Box<EventLoop<'a, SDLRender>> {
  println!("creating...");
  // let scene = Rc::from_raw(scene);
  // let win = Rc::from_raw(win);
  let render = SDLRender::new(
    Screen::new(
      View::new(
        Position::new(0, 0, 0),
        scene.borrow().get_size_x(),
        scene.borrow().get_size_y(),
        scene.borrow().get_size_z(),
      ),
      win.borrow().get_width() / scene.borrow().get_size_x(),
      win.borrow().get_height() / scene.borrow().get_size_y(),
    ),
    Rc::clone(&win),
  );
  let l = Box::new(EventLoop::new(Rc::clone(scene), render, win));
  return l;
  // println!("starting...");
  // l.start();


}

#[no_mangle]
extern "C" fn add_event_listener(eloop: &mut Box<EventLoop<SDLRender>>, callback: extern "C" fn(CEvent)) {
  let clos = move |e: &Event| {
    let ce = e.create_c();
    callback(ce)
  };
  let boxed = Box::new(clos);
  eloop.add_event_listener(Event::Loop, boxed).expect("added callback");
}

#[no_mangle]
extern "C" fn add_keyboard_listener(eloop: &mut Box<EventLoop<SDLRender>>, key: i32, callback: extern "C" fn(CEvent)) {
  let clos = move |e: &Event| {
    let ce = e.create_c();
    callback(ce)
  };
  let boxed = Box::new(clos);
  eloop.add_event_listener(Event::KeyBoard {key}, boxed).expect("added callback");
}

#[no_mangle]
extern "C" fn start_event_loop(mut eloop: Box<EventLoop<SDLRender>>) {
  println!("starting...");
  eloop.start();
}

#[no_mangle]
extern "C" fn change_type(scene: &SceneRef, id: GameObjectID, new_type: i32) {
  scene.borrow_mut().get_by_id(id).expect("correct id").set_type(new_type);
}

#[cfg(target_os = "emscripten")]
extern "C" { fn emscripten_run_script(script: *const c_char); }

#[cfg(target_os = "emscripten")]
#[no_mangle]
unsafe extern "C" fn output_file(fname: *const c_char) {
  let path = &CStr::from_ptr(fname).to_str().expect("correct string");
  let command = CString::new(format!("download_generated('{}')", path)).expect("correct pth and c5tring");
  emscripten_run_script(command.as_ptr());
}

#[cfg(not(target_os = "emscripten"))]
#[no_mangle]
extern "C" fn output_file(fname: *const c_char) {}

#[cfg(test)]
mod objects_tests {
  use super::*;

  #[test]
  fn ffi_testing() {
    testing();
  }
}