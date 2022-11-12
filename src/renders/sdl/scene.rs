use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use rustc_hash::FxHashMap;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use crate::geometry::position::Position;
use crate::objects::area::Area;
use crate::objects::game_object::{GameObject, GameObjectID};

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

  pub(crate) fn get_texture(&self, obj_type: usize) -> Option<&Texture<'a>> {
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

  pub fn get_by_pos(&self, pos: Position) -> Option<&GameObject> {
    return match &self.area.get_pos(&pos) {
      None => None,
      Some(id) => self.objects.get(id),
    }
  }

  pub fn get_by_id(&mut self, id: GameObjectID) -> Option<&mut GameObject> {
    return self.objects.get_mut(&id);
  }

  pub fn get_object_pos(&self, id: GameObjectID) -> Result<Position, Box<dyn Error>> {
    return Ok(self.objects.get(&id).ok_or::<Box<dyn Error>>("Object not found".into())?.get_pos());
  }

  pub fn remove_object(&mut self, id: GameObjectID) {
    let obj = self.objects.remove(&id);
    if let Some(o) = obj {
      self.area.set_pos(o.get_pos(), None);
    }
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

  pub fn resize(&mut self, x: usize, y: usize, z: usize) {
    self.area.resize(x, y, z);
  }

  pub fn resize_with_objects(&mut self, x: usize, y: usize, z: usize) {
    self.area.resize(x, y, z);
    let mut to_remove = Vec::<GameObjectID>::new();
    for (id, obj) in &self.objects {
      if obj.get_pos().x > x || obj.get_pos().y > y || obj.get_pos().z > z {
        to_remove.push(*id);
      }
    }
    for id in to_remove {
      self.objects.remove(&id);
    }
  }
}

