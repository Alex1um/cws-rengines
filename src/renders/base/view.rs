use crate::geometry::rect::Rect;
use crate::geometry::relative_component::RelativeComponent;
use crate::geometry::size::RelativeSize2D;

pub struct View {
  area_rect: Rect,
  layers: usize,
  screen_rect: Rect,
}

impl View {
  pub fn new(area_rect: Rect,
             layers: usize,
             screen_rect: Rect) -> Self {
    View {
      area_rect,
      layers,
      screen_rect,
    }
  }

  pub fn get_layers(&self) -> usize { self.layers }

  pub fn get_area_rect(&self, scene_size: &(usize, usize)) -> ((usize, usize), (usize, usize)) { self.area_rect.get_absolute(scene_size) }
  pub fn get_screen_rect(&self, resolution: &(usize, usize)) -> ((usize, usize), (usize, usize)) { self.screen_rect.get_absolute(resolution) }

  pub fn set_screen_size(&mut self, new: RelativeSize2D) {
    self.screen_rect.size = new;
  }
  pub fn set_screen_pos(&mut self, new: RelativeSize2D) {
    self.screen_rect.pos = new;
  }

  pub fn set_size(&mut self, new: RelativeSize2D) {
    self.screen_rect.size = new;
  }
}
