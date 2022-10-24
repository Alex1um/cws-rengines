use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use crate::geometry::position::Position;
use crate::renders::base::render::Render;
use crate::renders::base::screen::Screen;
use crate::renders::sdl::scene::SceneRef;
use crate::renders::sdl::window::WindowRef;
// use std::rc::Rc;
// use std::cell::RefCell;

pub type Creator = TextureCreator<WindowContext>;

pub struct SDLRender {
  screen: Screen,
  window: WindowRef,
}

impl SDLRender {
  pub fn new(screen: Screen, window: WindowRef) -> SDLRender
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
    let scene = scene.borrow();
    for v in &self.screen.view_stack {
      let Position { x: xs, y: ys, z: zs } = v.get_pos();
      let width = v.get_width();
      let height = v.get_height();
      let layers = v.get_layers();
      for z in zs..layers {
        for y in ys..height {
          for x in xs..width {
            if let Some(cur_obj) = scene.get(x, y, z) {
              let obj = Rect::new((x * self.screen.ratio_x) as i32,
                                  (y * self.screen.ratio_y) as i32,
                                  (self.screen.ratio_x) as u32,
                                  (self.screen.ratio_y) as u32,
              );

              let texture = scene.textures.get(cur_obj.get_type() as usize).expect("texture of object type");
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
