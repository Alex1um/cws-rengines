use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::{EventPump, Sdl};
use sdl2::event::Event as SDLEvent;
use sdl2::video::WindowContext;
use crate::events::event::Event;
use crate::events::event_provider::EventProvider;

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
    println!("kb element setted: {}", sdl2::hint::set("SDL_EMSCRIPTEN_KEYBOARD_ELEMENT", "#canvas"));
    // println!("kb element setted: {}", sdl2::hint::set("SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT", "#canvas"));
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
    for e in self.event_pump.poll_iter().take(10) {
      match e {
        SDLEvent::KeyDown { keycode, .. } => {
          let keycode = keycode.unwrap();
          buf.push(Event::KeyBoard { key: keycode as i32 });

          #[cfg(feature = "provide_dbg")]
          println!("key: {}; code: {}", keycode.name(), keycode as i32);

        }
        _ => {}
      }
      // println!("new event! {:?}", event);
    }
    // buf.append(&mut self.event_pump
    //   .keyboard_state()
    //   .pressed_scancodes()
    //   .filter_map(Keycode::from_scancode)
    //   .map(|e| {
    //
    //
    //     Event::KeyBoard { key: e as i32 }
    //   }
    //   )
    //   .collect());
  }
}
