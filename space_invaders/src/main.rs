extern crate sdl2;
mod renderer;

use sdl2::event::Event; 
use sdl2::keyboard::Keycode; 
use sdl2::rect::Rect;
use sdl2::rect::Point;
use std::time::Duration;

use crate::renderer::Renderer;

const SCREEN_WIDTH: u32 = 800; 
const SCREEN_HEIGHT: u32 = 800; 
pub const SPRITE_TILE_SIZE: (u32, u32) = (32, 32);

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?; 
  let video_subsystem = sdl_context.video()?; 
  let window = video_subsystem
               .window("Space Invaders 1.0", SCREEN_WIDTH, SCREEN_HEIGHT)
               .position_centered()
               .build()
               .map_err(|e| e.to_string())?;
  
  let timer = sdl_context.timer()?;
  let frames_per_anim = 4; 

  let mut renderer = Renderer::new(window)?; 
  let mut event_pump = sdl_context.event_pump()?; 

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        _ => { }
      }
    }

    let ticks = timer.ticks() as i32;

    renderer.initialize_window();
    std::thread::sleep(Duration::from_millis(50));
  }

  Ok(())
}
