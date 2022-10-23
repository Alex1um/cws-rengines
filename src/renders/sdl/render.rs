use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
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

pub type Creator = TextureCreator<WindowContext>;

pub struct Scene<'a> {
  pub textures:  Vec<Texture<'a>>,
  area: Area,
  objects: FxHashMap<GameObjectID, GameObject>,
  _obj_id: GameObjectID,
}

pub type SceneRef<'a> = Rc<RefCell<Scene<'a>>>;

impl<'a> Scene<'a> {
  pub fn new(area: Area) -> SceneRef<'a> {
    let sc = Scene {
      area,
      textures: vec![],
      objects: FxHashMap::<GameObjectID, GameObject>::default(),
      _obj_id: GameObjectID::default(),
    };
    return Rc::new(RefCell::new(sc));
  }

  pub fn add_object(&mut self, obj: GameObject) -> Result<GameObjectID, Box<dyn Error>> {
    let id = self._obj_id;
    self.area.try_set_pos(obj.get_pos(), Some(id))?;
    self.objects.insert(self._obj_id, obj);
    self._obj_id += 1;
    return Ok(id);
  }

  fn get_texture(&self, obj_type: i32) -> Option<&Texture<'a>> {
    // TODO: Solve it
    return self.textures.get(obj_type as usize);
  }

  pub fn load_textures<'b: 'a>(&mut self, creator: &'b TextureCreator<WindowContext>, tl: Vec<&str>) {
    for f in tl {
      let texture = creator.load_texture(f).expect("loaded texture");
      self.textures.push(texture);
    }
  }

  pub fn load_texture(&mut self, creator: &'static TextureCreator<WindowContext>, tl: &str) {
    let texture = creator.load_texture(tl).expect("loaded texture");
    self.textures.push(texture);
  }

  pub fn get(&self, x: usize, y: usize, z: usize) -> Option<&GameObject> {
    return match &self.area.get_pos(&Position::new(x, y, z)) {
      None => None,
      Some(id) => self.objects.get(id),
    }
  }

  pub fn get_object_pos(&self, id: GameObjectID) -> Result<Position, Box<dyn Error>> {
    return Ok(self.objects.get(&id).ok_or::<Box<dyn Error>>("Object not found".into())?.get_pos());
  }

  pub fn update_object(&mut self, id: GameObjectID, new_pos: Position) -> Result<(), Box<dyn Error>> {
    let obj = self.objects.get_mut(&id).ok_or("kekw".to_owned())?;
    let old_pos = obj.get_pos();
    self.area.try_set_pos(old_pos, None)?;
    self.area.try_set_pos(new_pos, Some(id))?;
    obj.set_pos(new_pos);
    return Ok(());
  }

  pub fn get_size_y(&self) -> usize {
    self.area.get_size_y()
  }

  pub fn get_size_z(&self) -> usize {
    self.area.get_size_z()
  }

  pub fn get_size_x(&self) -> usize {
    self.area.get_size_x()
  }

}

pub struct Window {
  ctx: Sdl,
  width: usize,
  height: usize,
  pub canvas: WindowCanvas,
  event_pump: EventPump,
  creator: &'static TextureCreator<WindowContext>
}

pub type WindowRef = Rc<RefCell<Window>>;

impl Window {
  pub fn new(width: usize, height: usize) -> Result<Window, Box<dyn Error>> {
    let context = sdl2::init()?;
    let pump = context.event_pump()?;
    let video = context.video()?;
    let window = video.window("Main", width as u32, height as u32)
      .build()?;
    let canvas = window.into_canvas().build()?;
    let creator: &'static _ = Box::leak(Box::new(canvas.texture_creator()));
    let w = Window {
      width,
      height,
      canvas,
      event_pump: pump,
      ctx: context,
      creator
    };
    return Ok(w);
  }

  pub fn get_texture_creator(&self) -> &'static TextureCreator<WindowContext> {
    return self.creator;
    // self.canvas.texture_creator()
  }

  pub fn create_ref(self) -> WindowRef {
    Rc::new(RefCell::new(self))
  }

  pub fn get_width(&self) -> usize {
    self.width
  }

  pub fn get_height(&self) -> usize {
    self.height
  }

}

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

impl EventProvider for Window {
  fn provide_events(&mut self, buf: &mut Vec<Event>) {
    buf.append(&mut self.event_pump
      .keyboard_state()
      .pressed_scancodes()
      .map(|e| Event::KeyBoard { key: e as i32 })
      .collect());
  }
}
