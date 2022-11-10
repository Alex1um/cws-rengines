use std::cell::RefCell;
use std::rc::Rc;
use crate::renders::base::view::View;

pub type ScreenRef = Rc<RefCell<Screen>>;

pub struct Screen {
  pub view_stack: Vec<View>,
  // width: usize,
  // height: usize,

  // pub ratio_x: usize,
  // pub ratio_y: usize,
}

impl Screen {
  pub fn new(view: View) -> Screen {
    let mut v = Vec::new();
    v.push(view);
    Screen {
      view_stack: v,
    }
  }

  pub fn create_ref(self) -> ScreenRef {
    Rc::new(RefCell::new(self))
  }

  pub fn add_layer(&mut self, view: View) {
    self.view_stack.push(view)
  }

  pub fn get_layer(&mut self, layer: usize) -> Option<&mut View> {
    return self.view_stack.get_mut(layer);
  }

}