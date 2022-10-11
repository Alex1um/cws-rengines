use crate::renders::base::view::View;

pub struct Screen {
  pub view_stack: Vec<View>,
  pub ratio_x: usize,
  pub ratio_y: usize,
}

impl Screen {
  pub fn new(view: View, ratio_x: usize, ratio_y: usize) -> Screen {
    let mut v = Vec::new();
    v.push(view);
    Screen {
      view_stack: v,
      ratio_x,
      ratio_y
    }
  }
  pub fn add_layer(&mut self, view: View) {
    self.view_stack.push(view)
  }

}