extern crate sdl2;
mod renderer;
mod game_context;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use crate::renderer::Renderer;
use crate::game_context::GameContext;

const GRID_X_SIZE: i32 = 40; 
const GRID_Y_SIZE: i32 = 30;
const DOT_SIZE_IN_PXS: i32 = 20; 


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let x = GRID_X_SIZE * DOT_SIZE_IN_PXS;
    let y = GRID_Y_SIZE * DOT_SIZE_IN_PXS;

    let window = video_subsystem
        .window("Snake Game", x.try_into().unwrap(), y.try_into().unwrap())
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;
    let mut context = GameContext::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut frame_counter = 0; 

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                  match keycode {
                    Keycode::W => context.move_up(),
                    Keycode::A => context.move_left(),
                    Keycode::S => context.move_down(),
                    Keycode::D => context.move_right(),
                    Keycode::Escape => context.toggle_pause(),
                    _ => {}
                  }
                }
                _ => {}
            }
        }
        frame_counter += 1; 
        if frame_counter & 125 == 0 {
          context.next_tick();
          frame_counter = 0; 
        }
        renderer.draw(&context)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}