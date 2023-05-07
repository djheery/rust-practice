pub use sdl2::render::WindowCanvas;
use sdl2::video::Window; 
use crate::Color; 
use crate::Rect; 


pub struct Renderer { canvas: WindowCanvas }

const INNER_BOARD_WIDTH: u32 = 750; 
const INNER_BOARD_HEIGHT: u32 = 550; 

impl Renderer {
  pub fn new(window: Window) -> Result<Renderer, String> {
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    Ok(Renderer { canvas })
  }

  pub fn draw(&mut self) -> Result<(), String> {
    self.draw_background();
    self.draw_inner_board();
    self.draw_screen_top();
    self.draw_screen_left();
    self.draw_board();
    self.canvas.present();
    Ok(())
  }

  pub fn draw_background(&mut self) {
    let color = Color::RGB(28, 63, 98);
    self.canvas.set_draw_color(color);
    self.canvas.clear();
  }

  fn draw_inner_board(&mut self) {
    let inner_board = Rect::new(25, 25, INNER_BOARD_WIDTH, INNER_BOARD_HEIGHT);
    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
    self.canvas.fill_rect(inner_board);
  }

  fn draw_screen_top(&mut self) {
    let header_width: u32 = INNER_BOARD_WIDTH - 50; 
    let header_height: u32  = 100;
    self.canvas.set_draw_color(Color::RGB(23, 33, 44));
    self.canvas.fill_rect(Rect::new(50, 50, header_width, header_height));

  }

  fn draw_screen_left(&mut self) {
    let left_width: u32 = 200;
    let left_height: u32 = 375; 
    let rect = Rect::new(50, 175, left_width, left_height);
    self.canvas.set_draw_color(Color::RGB(23, 33, 44)); 
    self.canvas.fill_rect(rect); 
  }

  fn draw_board(&mut self) {
    let board_width: u32 = INNER_BOARD_WIDTH - 275;
    let board_height: u32 = 375; 
    let rect = Rect::new(275, 175, board_width, board_height);
    self.canvas.set_draw_color(Color::RGB(23, 33, 44)); 
    self.canvas.fill_rect(rect); 
    self.draw_board_tiles(board_width as i32, board_height as i32);
  }

  fn draw_board_tiles(&mut self, width: i32, height: i32) {
    let row = height / 20 as i32;
    let col = width / 16 as i32; 
    let pad = 0; 
    println!("{row}, {col}");
    let mut inner_pad = 10; 
    let mut row_pad = 10;
    for i in 1..row {
      for c in 1..col {
        let x = (275 + c) + (pad + inner_pad);
        let y = (175 + i) + pad + row_pad;
        let rect = Rect::new(x, y, 10, 10);
        self.canvas.set_draw_color(Color::RGB(244, 233, 123));
        self.canvas.fill_rect(rect); 
        inner_pad += 15;
      }
      inner_pad = 10;
      row_pad += 20; 
    }
  }
}