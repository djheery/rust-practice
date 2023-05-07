extern crate sdl2; 
mod renderer; 
mod game_context; 
use sdl2::event::Event; 
use sdl2::keyboard::Keycode; 
use std::time::Duration; 
use crate::renderer::Renderer; 

const SCREEN_WIDTH: u32 = 800; 
const SCREEN_HEIGHT: u32 = 700; 

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?; 

  let window = video_subsystem
              .window("Snake Version 2", SCREEN_WIDTH, SCREEN_HEIGHT)
              .position_centered()
              .opengl()
              .build()
              .map_err(|e| e.to_string())?; 

  let mut renderer = Renderer::new(window)?; 
  let mut event_pump = sdl_context.event_pump()?;
  let mut _frame_counter = 0;

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        _ => {}
      }
    }

    renderer.initialize_window(); 
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 40));
  }

  Ok(())
}
