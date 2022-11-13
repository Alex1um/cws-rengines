use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use crate::renders::base::render::Render;
use crate::renders::base::screen::ScreenRef;
use crate::renders::sdl::scene::SceneRef;
use crate::renders::sdl::window::WindowRef;
// use std::rc::Rc;
// use std::cell::RefCell;

pub type Creator = TextureCreator<WindowContext>;

pub struct SDLRender {
  screen: ScreenRef,
  window: WindowRef,
}

impl SDLRender {
  pub fn new(screen: ScreenRef, window: WindowRef) -> SDLRender
  {
    let render = SDLRender {
      screen,
      window
    };
    return render;
  }
}

impl Render for SDLRender {
  fn render(&mut self, scene: &SceneRef) {
    self.window.borrow_mut().canvas.clear();
    let ww = self.window.borrow().get_width() as i32;
    let wh = self.window.borrow().get_height() as i32;
    let scene = scene.borrow();
    for v in &self.screen.borrow().view_stack {
      let zs = 0;
      let ((xs, ys), (width, height)) = v.get_area_rect(&(scene.get_size_x(), scene.get_size_y()));
      let layers = v.get_layers();
      let (screen_pos, screen_size) = v.get_screen_rect(&(ww, wh));
      let ratio_x = screen_size.0 / width as i32;
      let ratio_y = screen_size.1 / height as i32;
      for z in zs..layers {
        for y in ys..height {
          for x in xs..width {
            if let Some(cur_obj) = scene.get(x, y, z) {
              let obj = Rect::new(x as i32 * ratio_x + screen_pos.0,
                                  y  as i32 * ratio_y + screen_pos.1,
                                  (ratio_x) as u32,
                                  (ratio_y) as u32,
              );

              let texture = scene.get_texture(cur_obj.get_type() as usize).expect("texture of object type");
              texture.query();
              self.window.borrow_mut().canvas.copy(texture, None, obj).expect("successful texture write");
            }
          }
        }
      }
    }
    self.window.borrow_mut().canvas.present();
  }
}
