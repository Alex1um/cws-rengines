use sdl2::render::{RendererContext, TextureCreator};
use sdl2::video::WindowContext;
use crate::geometry::position::{CoordsType, Position};
use crate::objects::area::{Area, AreaRef};
use crate::objects::game_object::{GameObject, GameObjectID};
use crate::renders::base::screen::Screen;
use crate::renders::base::view::View;
use crate::renders::sdl::render::{Scene, SceneRef, SDLRender, Window, WindowRef};
use std::boxed::Box;
use std::cell::RefCell;
use std::env::current_dir;
use std::ffi::{c_char, CStr, CString};
use std::rc::Rc;
use sdl2::image::LoadTexture;
use crate::events::event_loop::EventLoop;


// type ScenePtr<'a> = *const RefCell<Scene<'a>>;
// type WindowPtr = *const RefCell<Window>;

#[no_mangle]
pub unsafe extern "C" fn create_object(area: &SceneRef, x: i32, y: i32, z: i32, r#type: i32) -> GameObjectID {
  // let area = SceneRef::from_raw(area);
  let obj = GameObject::new(r#type, Position::new(x as CoordsType, y as CoordsType, z as CoordsType));
  return area.borrow_mut().add_object(obj).expect("Successful adding");
}

#[no_mangle]
pub extern "C" fn create_scene(x: usize, y: usize, z: usize) -> SceneRef<'static> {
  let rf = Area::new(x, y, z);
  let scene = Scene::new(rf);
  return scene;
}

#[no_mangle]
pub extern "C" fn create_window(res_x: i32, res_y: i32) -> WindowRef {
  return Window::new(res_x as usize, res_y as usize)
    .expect("Created window")
    .create_ref();
}

#[no_mangle]
pub extern "C" fn testing() {
  println!("HELLO from rust");
}

#[no_mangle]
pub unsafe extern "C" fn load_texture(scene: &SceneRef, window: &WindowRef, path: *const c_char) {
  // let scene = Rc::from_raw(ctx);
  // let window = Rc::from_raw(win);
  let path = &CStr::from_ptr(path).to_str().expect("correct string");
  scene.borrow_mut().load_texture(window.borrow().get_texture_creator(), &path);
}

#[no_mangle]
pub unsafe extern "C" fn start_event_loop(scene: SceneRef, win: WindowRef) {
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
    Rc::clone(&win)
  );
  let mut l = EventLoop::new(scene, render, win);
  println!("starting...");
  l.start();
}

#[cfg(test)]
mod objects_tests {
  use super::*;

  #[test]
  fn ffi_testing() {
    testing();
  }
}