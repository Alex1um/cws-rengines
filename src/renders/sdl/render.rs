use std::error::Error;
use std::slice::Windows;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use crate::renders::base::screen::Screen;
use sdl2::video::{Window, WindowContext};
use crate::geometry::position::Position;

pub struct Render<'a> {
  screen: Screen,
  width: usize,
  height: usize,
  canvas: WindowCanvas,
  textures: Vec<Texture<'a>>,
}


impl<'a> Render<'a> {
  pub fn new(screen: Screen, width: usize, height: usize) -> Result<(TextureCreator<WindowContext>, Render<'a>), Box<dyn Error>> {
    let context = sdl2::init()?;
    let video = context.video()?;
    let window = video.window("Main", width as u32, height as u32)
      .build()?;
    let canvas = window.into_canvas().build()?;
    let creator = canvas.texture_creator();
    let mut render = Render {
      screen,
      width,
      height,
      canvas,
      textures: Vec::<Texture>::new(),
    };
    Ok((creator, render))
  }

  pub fn create_texture_creator(&self) -> TextureCreator<WindowContext> {
    self.canvas.texture_creator()
  }

  pub fn load_textures(&mut self, creator: &'a TextureCreator<WindowContext>,tl: Vec<&str>) {
    for f in tl {
      let texture = creator.load_texture(f).expect("loaded texture");
      self.textures.push(texture);
    }
  }

  pub fn render(&mut self) {
    // self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    // self.canvas.clear();
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
