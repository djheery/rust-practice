use sdl2::pixels::Color; 
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect; 
use sdl2::video::Window; 
use crate::game_context::GameContext;
use crate::game_context::Point; 
use crate::ui::UiNumberDisplay; 
use crate::ui::UiNumber;

const BOARD_HEIGHT: u32 = 600; 
const BOARD_WIDTH: u32 = 800; 
const BOARD_Y: u32 = 100; 
const HEADER_HEIGHT: u32 = 75; 
const HEADER_WIDTH: u32 = 800;


pub struct Renderer { canvas: WindowCanvas }

impl Renderer {
  pub fn new(window: Window) -> Result<Renderer, String> {
    let canvas = window.into_canvas()
                 .build()
                 .map_err(|e| e.to_string())?;

    Ok(Renderer { canvas })
  }

  pub fn draw(&mut self, game_context: &GameContext) -> Result<(), String> {
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
    self.draw_header()?; 
    self.draw_snake(game_context)?;
    self.draw_food(game_context)?;
    self.draw_score(game_context)?;
    self.canvas.present();
    Ok(())
  }

  fn draw_header(&mut self) -> Result<(), String> {
    let header = Rect::new(0, 0, HEADER_WIDTH, HEADER_HEIGHT);
    self.canvas.set_draw_color(Color::RGB(44, 44, 55)); 
    self.canvas.fill_rect(header)?;
    Ok(())
  }

  fn draw_score(&mut self, game_context: &GameContext) -> Result<(), String> {
    let idx_zero: u32 = (game_context.score / 10).try_into().unwrap();
    let idx_two: u32 = (game_context.score - (idx_zero * 10)).try_into().unwrap(); 

    let n1 = UiNumberDisplay::new(self.get_ui_number(idx_zero)); 
    let n2 = UiNumberDisplay::new(self.get_ui_number(idx_two));
    self.draw_score_pixels(n1, 0)?;
    self.draw_score_pixels(n2, 1)?;

    Ok(())
  }

  fn get_ui_number(&mut self, score: u32) -> UiNumber {
    match score {
      0 => UiNumber::Zero, 
      1 => UiNumber::One, 
      2 => UiNumber::Two, 
      3 => UiNumber::Three, 
      4 => UiNumber::Four, 
      5 => UiNumber::Five, 
      6 => UiNumber::Six, 
      7 => UiNumber::Seven,
      8 => UiNumber::Eight, 
      9 => UiNumber::Nine,
      _ => UiNumber::Zero,
    }
  }

  fn draw_score_pixels(&mut self, num: UiNumberDisplay, score_index: u8) -> Result<(), String> {
    let mut x = 10;
    let mut y = 10;
    
    if score_index == 1 { x = 40 };

    self.canvas.set_draw_color(Color::RED); 
    for r in num.matrix_representation.iter() {
      for c in r.iter() {
        if *c == 1 {
          let rect = Rect::new(x, y, 5, 5);
          self.canvas.fill_rect(rect)?;
        } 
        x += 5; 
      }

      if score_index == 1 { x = 40; } 
      else { x = 10; }
      y += 5;
    }

    Ok(())
  }

  fn draw_snake(&mut self, game_context: &GameContext) -> Result<(), String> {
    self.canvas.set_draw_color(Color::GREEN);
    for point in &game_context.player_position {
      self.draw_dot(point)?;
    }

    Ok(())
  }

  fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
    let Point(x, y) = point;
    let x_v = (x * 20) as i32;
    let y_v = (y * 20) + 100 as i32;
    self.canvas.fill_rect(Rect::new(x_v, y_v, 20, 20))?;
    Ok(())
  }

  fn draw_food(&mut self, game_context: &GameContext) -> Result<(), String> {
    self.canvas.set_draw_color(Color::RED); 
    self.draw_dot(&game_context.food)?;
    Ok(())
  }

}