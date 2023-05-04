extern crate sdl2; 
mod renderer; 

use sdl2::event::Event; 
use sdl2::keyboard::Keycode; 
use crate::renderer::Renderer;
pub use sdl2::pixels::Color;
pub use sdl2::render::WindowCanvas;
pub use sdl2::rect::Rect;
pub use sdl2::video::Window;
pub use std::time::Duration;

pub const SCREEN_WIDTH: u32 = 800; 
pub const SCREEN_HEIGHT: u32 = 600; 

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?; 
  let video_subsystem = sdl_context.video()?; 
  let window = video_subsystem
               .window("Layout Test", SCREEN_WIDTH, SCREEN_HEIGHT)
               .position_centered()
               .opengl()
               .build()
               .map_err(|e| e.to_string())?; 
               

  let mut renderer = Renderer::new(window)?;
  let mut context = sdl_context.event_pump()?;
  let mut frame_counter = 0;
  'running: loop {
    for event in context.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        Event::KeyDown { keycode: Some(keycode), .. } => {
          match keycode {
            Keycode::Escape => break 'running,
            _ => {}
          }
        } 
        _ => {}
      }
    }
    frame_counter += 1; 
    if frame_counter & 145 == 0 {
      frame_counter = 0; 
    }
    renderer.draw()?;
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 40));
  }

  Ok(())
}
