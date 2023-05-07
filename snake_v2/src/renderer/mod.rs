use sdl2::pixels::Color; 
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect; 
use sdl2::video::Window; 
use crate::game_context::GameContext;
use crate::game_context::Point; 

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
    self.draw_header(); 
    self.draw_snake(game_context)?;
    self.draw_food(game_context)?;
    self.canvas.present();
    Ok(())
  }

  fn draw_header(&mut self) {
    let header = Rect::new(0, 0, HEADER_WIDTH, HEADER_HEIGHT);
    self.canvas.set_draw_color(Color::RGB(44, 44, 55)); 
    self.canvas.fill_rect(header);
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