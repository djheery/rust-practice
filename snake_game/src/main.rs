extern crate sdl2;
use rand::Rng; 
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::ops::Add;

const GRID_X_SIZE: i32 = 40; 
const GRID_Y_SIZE: i32 = 30;
const DOT_SIZE_IN_PXS: i32 = 20; 

pub struct Renderer { canvas: WindowCanvas }
impl Renderer {
  pub fn new(window: Window) -> Result<Renderer, String> {
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    Ok(Renderer { canvas })
  }
  pub fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
    let Point(x, y) = point; 
    self.canvas.fill_rect(Rect::new(
      x * DOT_SIZE_IN_PXS as i32,
      y * DOT_SIZE_IN_PXS as i32, 
      DOT_SIZE_IN_PXS as u32,
      DOT_SIZE_IN_PXS as u32,
    ))?;

    Ok(())
  }
  pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
    self.draw_background(context);
    self.draw_player(context)?;
    self.draw_food(context)?;
    self.canvas.present();
    Ok(())
  }

  fn draw_background(&mut self, context: &GameContext) {
    let color = match context.state {
      GameState::Playing => Color::RGB(0, 0, 0),
      GameState::Paused => Color::RGB(30, 30, 30),
    };

    self.canvas.set_draw_color(color);
    self.canvas.clear();
  }

  fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
    self.canvas.set_draw_color(Color::GREEN);
    for point in &context.player_position {
      self.draw_dot(point)?;
    }

    Ok(())
  }

  fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
    self.canvas.set_draw_color(Color::RED);
    self.draw_dot(&context.food)?;
    Ok(())
  }
}

pub enum GameState { Playing, Paused }
pub enum PlayerDirection { Up, Down, Right, Left }
#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
  type Output = Point;
  fn add(self, rhs: Point) -> Self::Output {
    Point(self.0 + rhs.0, self.1 + rhs.1)
  }
}

pub struct GameContext {
  pub player_position: Vec<Point>,
  pub player_direction: PlayerDirection,
  pub food: Point,
  pub state: GameState,
}

impl GameContext {
  pub fn new() -> GameContext {
    GameContext {
      player_position: vec![Point(5, 1), Point(5, 2), Point(5, 3)],
      player_direction: PlayerDirection::Down, 
      state: GameState::Paused,
      food: Point(3, 3)
    }
  }

  pub fn toggle_pause(&mut self) {
    match self.state {
      GameState::Paused => { self.state = GameState::Playing; },
      GameState::Playing => { self.state = GameState::Paused; }
    }
  }

  pub fn next_tick(&mut self) {
    if let GameState::Paused = self.state { return; }
    let check_is_food: bool = self.check_food_collision();
    if check_is_food == true { 
      let nh: Point = self.get_next_head_positon();
      self.player_position.push(nh);
      self.random_food_pos();
    }

    let next_head_position: Point = self.get_next_head_positon();
    self.player_position.pop();
    self.player_position.reverse();
    self.player_position.push(next_head_position);
    self.player_position.reverse();
  }

  fn check_food_collision(&mut self) -> bool {
    let (food_x, food_y) = (self.food.0, self.food.1); 
    let head = self.player_position.first().unwrap();
    let (head_x, head_y) = (head.0, head.1);  
    
    food_x == head_x && food_y == head_y
  }

  fn get_next_head_positon(&mut self) -> Point {
    let head_position = self.player_position.first().unwrap(); 
    match self.player_direction {
      PlayerDirection::Up => *head_position + Point(0, -1),
      PlayerDirection::Down => *head_position + Point(0, 1),
      PlayerDirection::Right => *head_position + Point(1, 0),
      PlayerDirection::Left => *head_position + Point(-1, 0),
    }
  }

  fn random_food_pos(&mut self) {
    let x = rand::thread_rng().gen_range(1..=GRID_X_SIZE);
    let y = rand::thread_rng().gen_range(1..=GRID_Y_SIZE);
    println!("x{x},y{y}");
    self.food = Point(x, y);
  }

  pub fn move_up(&mut self) {
    match self.player_direction {
      PlayerDirection::Up => {},
      PlayerDirection::Down => {},
      _ => { self.player_direction = PlayerDirection::Up; }
    }
  }

  pub fn move_down(&mut self) {
    match self.player_direction {
      PlayerDirection::Up => {},
      PlayerDirection::Down => {},
      _ => { self.player_direction = PlayerDirection::Down; }
    }
  }

  pub fn move_left(&mut self) {
    match self.player_direction {
      PlayerDirection::Left => {},
      PlayerDirection::Right => {},
      _ => { self.player_direction = PlayerDirection::Left },
    }
  }

  pub fn move_right(&mut self) {
    match self.player_direction {
      PlayerDirection::Left => {},
      PlayerDirection::Right => {},
      _ => { self.player_direction = PlayerDirection::Right },
    }
  }
}

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