use sdl2::render::{RendererContext, TextureCreator};
use sdl2::video::WindowContext;
use crate::geometry::position::{CoordsType, Position};
use crate::objects::area::{Area, AreaRef};
use crate::objects::game_object::{GameObject, GameObjectID};
use crate::renders::base::screen::Screen;
use crate::renders::base::view::View;
use crate::renders::sdl::render::SDLRender;
use std::boxed::Box;
use std::cell::RefCell;
use std::env::current_dir;
use std::ffi::{c_char, CStr, CString};
use std::rc::Rc;
use sdl2::image::LoadTexture;
use crate::events::event_loop::EventLoop;


type AreaPtr = *const RefCell<Area>;
type RenderPtr = *mut (TextureCreator<WindowContext>, SDLRender<'static>);

#[no_mangle]
unsafe extern "C" fn create_object(area: AreaPtr, x: i32, y: i32, z: i32, r#type: i32) -> GameObjectID {
  let area = AreaRef::from_raw(area);
  let obj = GameObject::new(r#type, Position::new(x as CoordsType, y as CoordsType, z as CoordsType));
  return area.borrow_mut().insert(obj).expect("Successful adding");
}

#[no_mangle]
extern "C" fn create_area(x: usize, y: usize, z: usize) -> AreaPtr {
  let rf = Area::new(x, y, z).create_ref();
  return Rc::into_raw(rf);
}

#[no_mangle]
unsafe extern "C" fn render_new(area: AreaPtr, res_x: i32, res_y: i32) -> RenderPtr {
  let area = Rc::from_raw(area);
  let ptr = SDLRender::new(
    Screen::new(
      View::new(
        &area,
        Position::new(0, 0, 0),
        area.borrow().get_size_x(),
        area.borrow().get_size_y(),
        area.borrow().get_size_z(),
      ),
      (res_x as usize) / area.borrow().get_size_x(),
      (res_y as usize) / area.borrow().get_size_y(),
    ),
    res_x as usize,
    res_y as usize,
  ).expect("Created Render");
  return Box::into_raw(Box::new(ptr));
}

#[no_mangle]
extern "C" fn testing() {
  println!("HELLO from rust");
}

#[no_mangle]
unsafe extern "C" fn load_texture(ctx: RenderPtr, path: *mut c_char) {
  let path = CString::from_raw(path).into_string().expect("correct c string");
}

#[no_mangle]
unsafe extern "C" fn start_event_loop(area: AreaPtr, ctx: RenderPtr) {
  if !area.is_null() && !ctx.is_null() {
    let (c, r) = *Box::from_raw(ctx);
    println!("creating...");
    let mut l = EventLoop::new(Rc::from_raw(area), r);
    println!("starting...");
    l.start();
  }
}

#[cfg(test)]
mod objects_tests {
  use super::*;

  #[test]
  fn ffi_testing() {
    testing();
  }
}