extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::time::Duration;
use std::path::Path;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const CELL_SIZE: u32 = 20;
const BOARD_WIDTH: u32 = SCREEN_WIDTH / CELL_SIZE;
const BOARD_HEIGHT: u32 = SCREEN_HEIGHT / CELL_SIZE;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Snake Game", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(Path::new("OpenSans-Regular.ttf"), 16)?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut snake_x = BOARD_WIDTH / 2;
    let mut snake_y = BOARD_HEIGHT / 2;
    let mut snake_direction = Direction::Right;

    // Main game loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
                    snake_direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => {
                    snake_direction = Direction::Down;
                },
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => {
                    snake_direction = Direction::Left;
                },
                Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => {
                    snake_direction = Direction::Right;
                },
                _ => {}
            }
        }

        // Update game state
        match snake_direction {
            Direction::Up => {
                snake_y = (snake_y + BOARD_HEIGHT - 1) % BOARD_HEIGHT;
            },
            Direction::Down => {
                snake_y = (snake_y + 1) % BOARD_HEIGHT;
            },
            Direction::Left => {
                snake_x = (snake_x + BOARD_WIDTH - 1) % BOARD_WIDTH;
            },
            Direction::Right => {
                snake_x = (snake_x + 1) % BOARD_WIDTH;
            },
        }

        // Clear the screen to white
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Draw game board
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        // Draw vertical grid lines
        for i in 1..BOARD_WIDTH {
          canvas.draw_line(Point::new(i as i32 * CELL_SIZE as i32, 0),
                           Point::new(i as i32 * CELL_SIZE as i32, SCREEN_HEIGHT as i32))
              .map_err(|e| e.to_string())?;
      }

      // Draw horizontal grid lines
      for i in 1..BOARD_HEIGHT {
          canvas.draw_line(Point::new(0, i as i32 * CELL_SIZE as i32),
                           Point::new(SCREEN_WIDTH as i32, i as i32 * CELL_SIZE as i32))
              .map_err(|e| e.to_string())?;
      }

      // Draw snake
      let snake_rect = Rect::new(snake_x as i32 * CELL_SIZE as i32,
                                 snake_y as i32 * CELL_SIZE as i32,
                                 CELL_SIZE,
                                 CELL_SIZE);
      canvas.set_draw_color(Color::RGB(0, 255, 0));
      canvas.fill_rect(snake_rect).map_err(|e| e.to_string())?;

      // Draw score
      let score_surface = font.render("Score: 0").blended(Color::BLACK).map_err(|e| e.to_string())?;
      let score_texture = texture_creator.create_texture_from_surface(&score_surface).map_err(|e| e.to_string())?;
      
      let texture_query = score_texture.query();
      let score_rect = match texture_query {
          Ok(query) => query.bounding_rect(Point::new(10, 10)),
          Err(e) => return Err(e.to_string()),
      };
      
      canvas.copy(&score_texture, None, score_rect).map_err(|e| e.to_string())?;

      // Update the screen
      canvas.present();

      // Wait for a short amount of time to control the game speed
      std::thread::sleep(Duration::from_millis(100));
  }

  Ok(())
}

enum Direction {
  Up,
  Down,
  Left,
  Right,
}
