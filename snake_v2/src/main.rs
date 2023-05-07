extern crate sdl2; 
mod renderer; 
mod game_context; 
mod ui;
use sdl2::event::Event; 
use sdl2::keyboard::Keycode; 
use std::time::Duration; 
use crate::renderer::Renderer; 
use crate::game_context::GameContext;

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
  let mut game_context = GameContext::new();
  let mut event_pump = sdl_context.event_pump()?;
  let mut frame_counter = 0;
  renderer.draw(&game_context)?; 

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } => break 'running,
        Event::KeyDown { keycode: Some(keycode), .. } => {
          match keycode {
            Keycode::W => game_context.move_up(),
            Keycode::Up => game_context.move_up(), 
            Keycode::A => game_context.move_left(),
            Keycode::Left => game_context.move_left(), 
            Keycode::S => game_context.move_down(), 
            Keycode::Down => game_context.move_down(), 
            Keycode::D => game_context.move_right(), 
            Keycode::Right => game_context.move_right(), 
            Keycode::Escape => game_context.toggle_pause(),
            _ => {}
          }
        } 
        _ => {}
      }
    }

    frame_counter += 1; 
    if frame_counter == 2 {
      game_context.tick();
      frame_counter = 0;
    }
    
    renderer.draw(&game_context)?;
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 40));
  }

  Ok(())
}
