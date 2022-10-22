use std::error::Error;
use rustc_hash::FxHashMap;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::{EventPump, Sdl};
use crate::renders::base::screen::Screen;
use sdl2::video::{WindowContext};
use crate::events::event::Event;
use crate::events::event_provider::EventProvider;
use crate::geometry::position::Position;
use crate::objects::area::Area;
use crate::renders::base::render::Render;

use crate::objects::game_object::{GameObject, GameObjectID};

pub struct Scene<'a> {
  pub textures:  Vec<Texture<'a>>,
  area: Area,
  objects: FxHashMap<GameObjectID, GameObject>,
  _obj_id: GameObjectID,
}

impl<'a> Scene<'a> {
  pub fn new(area: Area) -> Scene<'a> {
    Scene {
      area,
      textures: vec![],
      objects: FxHashMap::<GameObjectID, GameObject>::default(),
      _obj_id: GameObjectID::default(),
    }
  }

  fn add_object(&mut self, obj: GameObject) -> GameObjectID {
    let id = self._obj_id;
    self.area.set_pos(obj.get_pos(), id);
    self.objects.insert(self._obj_id, obj);
    self._obj_id += 1;
    return id;
  }

  fn get_texture(&self, obj_type: i32) -> Option<&Texture<'a>> {
    // TODO: Solve it
    return self.textures.get(obj_type as usize);
  }

  pub fn load_textures(&mut self, creator: &'a TextureCreator<WindowContext>, tl: Vec<&str>) {
    for f in tl {
      let texture = creator.load_texture(f).expect("loaded texture");
      self.textures.push(texture);
    }
  }

  pub fn load_texture(&mut self, creator: &'a TextureCreator<WindowContext>, tl: &str) {
    let texture = creator.load_texture(tl).expect("loaded texture");
    self.textures.push(texture);
  }

  pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&GameObject> {
    return self.objects.get(&self.area.get_pos(&Position::new(x, y, z)).unwrap());
  }

}

pub struct Window {
  ctx: Sdl,
  width: usize,
  height: usize,
  canvas: WindowCanvas,
  event_pump: EventPump,
}

impl Window {
  pub fn new(width: usize, height: usize) -> Result<Window, Box<dyn Error>> {
    let context = sdl2::init()?;
    let pump = context.event_pump()?;
    let video = context.video()?;
    let window = video.window("Main", width as u32, height as u32)
      .build()?;
    let canvas = window.into_canvas().build()?;
    let w = Window {
      width,
      height,
      canvas,
      event_pump: pump,
      ctx: context,
    };
    return Ok(w);
  }
}

pub struct SDLRender<'a> {
  screen: Screen,
  window: &'a mut Window,
}

impl<'a> SDLRender<'a> {
  pub fn new(screen: Screen, window: &mut Window) -> SDLRender
  {
    let render = SDLRender {
      screen,
      window
    };
    return render;
  }

}

impl Render for SDLRender<'_> {
  fn render(&mut self, scene: &Scene) {
    for v in &self.screen.view_stack {
      let Position { x: xs, y: ys, z: zs } = v.get_pos();
      let area = v.get_area();
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
              self.window.canvas.copy(texture, None, obj).expect("successful texture write");
            }
          }
        }
      }
    }
    self.window.canvas.present();
  }
}

impl EventProvider for Window {
  fn provide_events(&mut self, buf: &mut Vec<Event>) {
    buf.append(&mut self.event_pump
      .keyboard_state()
      .pressed_scancodes()
      .map(|e| Event::KeyBoard { key: e as i32 })
      .collect());
  }
}
