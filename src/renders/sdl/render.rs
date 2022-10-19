use std::error::Error;
use std::slice::Windows;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::{Keycode, Scancode};
use crate::renders::base::screen::Screen;
use sdl2::video::{Window, WindowContext};
use crate::events::event::Event;
use crate::events::event_provider::EventProvider;
use crate::geometry::position::Position;
use crate::renders::base::render::Render;

pub struct SDLTextureHolder<'b> {
  textures: Vec<Texture<'b>>,
  texture_creator: &'b TextureCreator<WindowContext>,
}

impl<'b> SDLTextureHolder<'b> {
  fn new(render: &SDLRender) -> SDLTextureHolder<'b> {
    return SDLTextureHolder {
      textures: vec![],
      texture_creator: &render.txt_creator,
    }
  }

  pub fn load_textures(&mut self, tl: Vec<&str>) {
    for f in tl {
      let texture = self.texture_creator.load_texture(f).expect("loaded texture");
      self.textures.push(texture);
    }
  }

  pub fn load_texture(&mut self, tl: &str) {
    let texture = self.texture_creator.load_texture(tl).expect("loaded texture");
    self.textures.push(texture);
  }

}

pub struct SDLRender {
  screen: Screen,
  width: usize,
  height: usize,
  canvas: WindowCanvas,
  event_pump: EventPump,
  // textures: Vec<Texture<'a>>,
  txt_creator: TextureCreator<WindowContext>,
}


impl SDLRender {
  pub fn new(screen: Screen, width: usize, height: usize) -> Result<SDLRender, Box<dyn Error>>
  {
    let context = sdl2::init()?;
    let pump = context.event_pump()?;
    let video = context.video()?;
    let window = video.window("Main", width as u32, height as u32)
      .build()?;
    let canvas = window.into_canvas().build()?;
    let creator = canvas.texture_creator();
    let mut render = SDLRender {
      screen,
      width,
      height,
      canvas,
      event_pump: pump,
      txt_creator: creator,
    };
    return Ok(render);
  }

  pub fn create_texture_creator(&self) -> TextureCreator<WindowContext> {
    self.canvas.texture_creator()
  }

  // pub fn load_textures(&mut self, creator: &'a TextureCreator<WindowContext>, tl: Vec<&str>) {
  //   for f in tl {
  //     let texture = creator.load_texture(f).expect("loaded texture");
  //     self.textures.push(texture);
  //   }
  // }
  //
  // pub fn load_texture(&'a mut self, tl: &str) {
  //   let texture: Texture<'a> = self.txt_creator.load_texture(tl).expect("loaded texture");
  //   self.textures.push(texture);
  // }
}

impl Render for SDLRender {
  fn render(&mut self) {
    for v in &self.screen.view_stack {
      let Position { x: xs, y: ys, z: zs } = v.get_pos();
      let area = v.get_area();
      let width = v.get_width();
      let height = v.get_height();
      let layers = v.get_layers();
      for z in zs..layers {
        for y in ys..height {
          for x in xs..width {
            if let Some(cur_obj) = area.borrow().get(x, y, z) {
              let obj = Rect::new((x * self.screen.ratio_x) as i32,
                                  (y * self.screen.ratio_y) as i32,
                                  (self.screen.ratio_x) as u32,
                                  (self.screen.ratio_y) as u32,
              );

              let texture: &Texture = self.textures.get(cur_obj.get_type() as usize).expect("texture of object type");
              texture.query();
              self.canvas.copy(texture, None, obj).expect("successful texture write");
            }
          }
        }
      }
    }
    self.canvas.present();
  }
}

impl EventProvider for SDLRender {
  fn provide_events(&mut self, buf: &mut Vec<Event>) {
    buf.append(&mut self.event_pump
      .keyboard_state()
      .pressed_scancodes()
      .map(|e| Event::KeyBoard { key: e as i32 })
      .collect());
  }
}
