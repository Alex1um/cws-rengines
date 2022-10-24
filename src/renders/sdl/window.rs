use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::{EventPump, Sdl};
use sdl2::video::WindowContext;
use crate::events::event::Event;
use crate::events::event_provider::EventProvider;

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

impl EventProvider for Window {
  fn provide_events(&mut self, buf: &mut Vec<Event>) {
    buf.append(&mut self.event_pump
      .keyboard_state()
      .pressed_scancodes()
      .map(|e| Event::KeyBoard { key: e as i32 })
      .collect());
  }
}
