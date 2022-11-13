use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use rustc_hash::FxHashMap;
use crate::events::event::{Event};
use crate::events::event_provider::EventProvider;
use crate::renders::base::render::Render;
use std::time::{Duration, Instant};
use crate::renders::sdl::scene::SceneRef;

#[cfg(not(target_os = "emscripten"))]
use std::thread::sleep;

#[cfg(target_os = "emscripten")]
extern "C" {
  fn emscripten_sleep(ms: u32);
}

impl EventProvider for Vec<Event> {
  fn provide_events(&mut self, buf: &mut Vec<Event>) {
    buf.extend(self.drain(..));
  }
}

pub type InLoopProviderRef<'a> = &'a mut Vec<Event>;

type EventCallBack = Box<dyn FnMut(&Event, InLoopProviderRef)>;

pub struct EventLoop<'a, T: Render + Sized> {
  scene: SceneRef<'a>,
  render: T,
  event_listeners: FxHashMap<Event, Vec<EventCallBack>>,
  event_providers: Vec<Rc<RefCell<dyn EventProvider>>>,
  millis_per_frame: u64,
}

impl<T> EventLoop<'_, T> where T: Render + Sized {
  pub fn new(scene: SceneRef, render: T, max_fps: u64) -> EventLoop<T> {
    EventLoop {
      scene,
      render,
      event_listeners: FxHashMap::<Event, Vec<EventCallBack>>::default(),
      event_providers: vec![],
      millis_per_frame: 1000 / max_fps,
    }
  }

  pub fn get_render(&mut self) -> &mut T {
    return &mut self.render;
  }

  pub fn add_event_listener(&mut self, event: Event, f: EventCallBack) -> Result<(), Box<dyn Error>> {
    if let Some(v) = self.event_listeners.get_mut(&event) {
      v.push(f);
    } else {
      self.event_listeners.insert(event, vec![f]);
    }
    Ok(())
  }

  pub fn add_event_provider(&mut self, provider: Rc<RefCell<dyn EventProvider>>) {
    self.event_providers.push(provider);
  }

  pub fn set_max_fps(&mut self, max_fps: u64) {
    self.millis_per_frame = 1000 / max_fps;
  }

  pub fn start(&mut self) {
    let mut buf: Vec<Event> = vec![];
    let mut inlp = Vec::<Event>::new();
    let mut tick_event = Event::Loop {tick: 0};
    'main_loop: loop {
      let clock = Instant::now();
      for provider in &self.event_providers {
        provider.borrow_mut().provide_events(&mut buf);
      }
      buf.extend(inlp.drain(..));
      for e in buf.drain(..) {
        if let Some(listeners) = self.event_listeners.get_mut(&e) {
          for listener in listeners {
            listener(&e, &mut inlp);
          }
        }
        if e == Event::Exit {
          break 'main_loop;
        }
      }
      if let Some(listeners) = self.event_listeners.get_mut(&tick_event) {
        for listener in listeners {
          listener(&tick_event, &mut inlp);
        }
      }
      self.render.render(&self.scene);
      match tick_event {
        Event::Loop {ref mut tick} => { *tick += 1; }
        _ => {}
      };

      let time = (clock.elapsed().as_millis()) as u64;

      if self.millis_per_frame > time {
        #[cfg(not(target_os = "emscripten"))]
        sleep(Duration::from_millis(self.millis_per_frame - time));

        #[cfg(target_os = "emscripten")]
        unsafe { emscripten_sleep(self.millis_per_frame - time); }
      }
      // #[cfg(feature = "provide_dbg")]
      // println!("time for frame(without sleep): {} millis; fps(with sleep): {}",time, 1. / clock.elapsed().as_secs_f64());
    }
  }
}