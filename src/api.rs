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
use std::rc::Rc;


type AreaPtr = *const RefCell<Area>;

#[no_mangle]
pub unsafe extern "C" fn create_object(area: AreaPtr, x: i32, y: i32, z: i32, r#type: i32) -> GameObjectID {
  let area = AreaRef::from_raw(area);
  let obj = GameObject::new(r#type, Position::new(x as CoordsType, y as CoordsType, z as CoordsType));
  return area.borrow_mut().insert(obj).expect("Successful adding");
}

#[no_mangle]
pub extern "C" fn create_area(x: usize, y: usize, z: usize) -> AreaPtr {
  let rf = Area::new(x, y, z).create_ref();
  return Rc::into_raw(rf);
}

#[no_mangle]
pub unsafe extern "C" fn render_new(area: AreaPtr, res_x: i32, res_y: i32) -> *const (TextureCreator<WindowContext>, SDLRender<'static>) {
  let area = Rc::from_raw(area);
  let ptr = Box::new(SDLRender::new(
    Screen::new(
      View::new(
        &area,
        Position::new(0, 0, 0),
        area.borrow().get_size_x(),
        area.borrow().get_size_y(),
        area.borrow().get_size_z()
      ),
      (res_x as usize) / area.borrow().get_size_x(),
      (res_y as usize) / area.borrow().get_size_y(),
    ),
    res_x as usize,
    res_y as usize,
  ).expect("Created Render"));
  return Box::into_raw(ptr);
}

#[no_mangle]
pub extern "C" fn testing() {
  println!("HELLO from rust");
}
