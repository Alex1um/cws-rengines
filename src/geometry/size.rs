pub enum ViewSize {
  Percent(f32, f32),
  Pixels(usize, usize),
}

impl ViewSize {
  pub fn get_in_px(&self, resolution: (usize, usize)) -> (usize, usize) {
    match self {
      ViewSize::Percent(w, h) => {
        return ((*w * resolution.0 as f32) as usize, (*h * resolution.1 as f32) as usize)
      }
      ViewSize::Pixels(w, h) => {
        return (*w, *h);
      }
    }
  }
}