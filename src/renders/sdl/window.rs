use std::cell::RefCell;
use std::error::Error;
use std::io::stdin;
use std::rc::Rc;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::{EventPump, Sdl};
use sdl2::event::Event as SDLEvent;

#[cfg(target_family = "unix")]
use sdl2::libc::{fcntl, O_NONBLOCK, F_SETFL};
#[cfg(target_family = "unix")]
use std::os::unix::io::AsRawFd;
use sdl2::mouse::MouseWheelDirection;

use sdl2::video::WindowContext;
use crate::events::event::Event;
use crate::events::event_provider::EventProvider;
use crate::geometry::position::Position;

pub struct Window {
  ctx: Sdl,
  width: usize,
  height: usize,
  pub canvas: WindowCanvas,
  event_pump: EventPump,
  creator: &'static TextureCreator<WindowContext>,
}

pub type WindowRef = Rc<RefCell<Window>>;

impl Window {
  pub fn new(width: usize, height: usize) -> Result<Window, Box<dyn Error>> {

    sdl2::hint::set("SDL_EMSCRIPTEN_KEYBOARD_ELEMENT", "#canvas");

    #[cfg(target_family = "unix")]
    unsafe { fcntl(stdin().as_raw_fd(), F_SETFL, O_NONBLOCK); };

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
      creator,
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
    buf.extend(self.event_pump
      .poll_iter()
      .filter_map(|e|
        match e {
          SDLEvent::KeyDown { keycode, .. } => {
            let keycode = keycode.unwrap();
            #[cfg(feature = "provide_dbg")]
            println!("key: {}; code: {}", keycode.name(), keycode as i32);

            Some(Event::KeyBoard { key: keycode as i32 })
          }
          SDLEvent::MouseWheel { x, y, .. } => {

            #[cfg(feature = "provide_dbg")]
            println!("mouse wheel: {:?}", e);

            Some(Event::Mouse{ key: 3, pos: (x, y) })
          }
          SDLEvent::MouseMotion { x, y, .. } => {

            #[cfg(feature = "provide_dbg")]
            println!("mouse, move: {:?}", e);

            Some(Event::Mouse{ key: 0, pos: (x, y) })
          }
          _ => {
            #[cfg(feature = "provide_dbg")]
            println!("event: {:?}", e);

            None
          },
        }
      ));
  }
}
