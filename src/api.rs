use crate::geometry::position::{CoordsType, Position};
use crate::objects::area::{Area};
use crate::objects::game_object::{GameObject, GameObjectID};
use crate::renders::base::screen::{Screen, ScreenRef};
use crate::renders::base::view::View;
use std::boxed::Box;
use std::cell::RefCell;
use std::ffi::{c_char, c_float, c_int, CStr, CString};
use std::rc::Rc;
use crate::events::cevent::CEvent;
use crate::events::event::Event;
use crate::events::event_loop::{EventLoop, InLoopProviderRef};
use crate::events::event_provider::{console_input_command_provider, file_input_provider};
use crate::geometry::rect::Rect;
use crate::geometry::size::{RelativeSize, RelativeSize2D};
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
extern "C" fn create_screen() -> ScreenRef {
  return Screen::new(
    View::new(
      Rect::new((0., 0.).into(), (1., 1.).into()),
      2,
      Rect::new((0., 0.).into(), (1., 1.).into()),
    )).create_ref();
}

#[no_mangle]
extern "C" fn create_event_loop<'a>(scene: &SceneRef<'a>, win: &WindowRef, screen: &ScreenRef, fps_max: c_int) -> Box<EventLoop<'a, SDLRender>> {
  println!("creating...");
  let render = SDLRender::new(Rc::clone(screen),
                              Rc::clone(&win),
  );
  let mut l = Box::new(EventLoop::new(Rc::clone(scene), render, fps_max as u32));
  let link = Rc::clone(win);
  l.add_event_provider(link);
  return l;
  // println!("starting...");
  // l.start();
}

#[no_mangle]
extern "C" fn add_event_listener(eloop: &mut Box<EventLoop<SDLRender>>, ce: CEvent, callback: extern "C" fn(CEvent, InLoopProviderRef)) {
  let clos = move |e: &Event, p: InLoopProviderRef| {
    let ce = e.to_c();
    callback(ce, p)
  };
  let boxed = Box::new(clos);
  eloop.add_event_listener(Event::from_c(ce), boxed).expect("added callback");
}

#[no_mangle]
extern "C" fn add_keyboard_listener(eloop: &mut Box<EventLoop<SDLRender>>, key: i32, callback: extern "C" fn(CEvent, InLoopProviderRef)) {
  let clos = move |e: &Event, provider: InLoopProviderRef| {
    let ce = e.to_c();
    callback(ce, provider)
  };
  let boxed = Box::new(clos);
  eloop.add_event_listener(Event::KeyBoardButtonDown { key }, boxed).expect("added callback");
}

#[no_mangle]
extern "C" fn throw_event(prov: InLoopProviderRef, e: CEvent) {
  prov.push(Event::from_c(e));
}

#[no_mangle]
extern "C" fn start_event_loop(mut eloop: Box<EventLoop<SDLRender>>) {
  println!("starting...");
  eloop.start();
}

#[no_mangle]
extern "C" fn change_type(scene: &SceneRef, id: GameObjectID, new_type: c_int) {
  scene.borrow_mut().get_by_id(id).expect("correct id").set_type(new_type as i32);
}

#[cfg(target_os = "emscripten")]
extern "C" {
  fn emscripten_run_script(script: *const c_char);
}

#[cfg(target_os = "emscripten")]
#[no_mangle]
unsafe extern "C" fn output_file(fname: *const c_char) {
  let path = &CStr::from_ptr(fname).to_str().expect("correct string");
  let command = CString::new(format!("download_generated('{}')", path)).expect("correct pth and c5tring");
  emscripten_run_script(command.as_ptr());
}

#[cfg(not(target_os = "emscripten"))]
#[no_mangle]
extern "C" fn output_file(_: *const c_char) {}

#[no_mangle]
extern "C" fn add_console_input_provider(eloop: &mut Box<EventLoop<SDLRender>>) {
  eloop.add_event_provider(Rc::new(RefCell::new(console_input_command_provider)));
}

#[no_mangle]
extern "C" fn add_file_input_provider(eloop: &mut Box<EventLoop<SDLRender>>) {
  eloop.add_event_provider(Rc::new(RefCell::new(file_input_provider)));
}

#[no_mangle]
extern "C" fn scene_resize(scene: &SceneRef, x: c_int, y: c_int, z: c_int) {
  scene.borrow_mut().resize(x as usize, y as usize, z as usize);
}

#[no_mangle]
extern "C" fn scene_smart_resize(scene: &SceneRef, x: c_int, y: c_int, z: c_int) {
  scene.borrow_mut().resize_with_objects(x as usize, y as usize, z as usize);
}

#[no_mangle]
extern "C" fn set_view_size(screen: &ScreenRef, scale: c_float) {
  let mut screen = screen.borrow_mut();
  let view = screen.get_layer(0).expect("view exist");
  // view.set_pos();
  view.set_screen_size(RelativeSize::Percent(scale as f32).into());
}

#[no_mangle]
extern "C" fn set_view_pos(screen: &ScreenRef, dx: c_int, dy: c_int) {
  let mut screen = screen.borrow_mut();
  let view = screen.get_layer(0).expect("view exist");
  // view.set_pos();
  view.set_screen_pos(RelativeSize2D::from((dx as isize, dy as isize)));
}

#[cfg(test)]
mod objects_tests {
  use super::*;

  #[test]
  fn ffi_testing() {
    testing();
  }
}