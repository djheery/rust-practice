use sdl2::pixels::Color; 
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect; 
use sdl2::video::Window; 

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

  pub fn initialize_window(&mut self) {
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
    self.draw_header(); 
    self.canvas.present();
  }

  fn draw_header(&mut self) {
    let header = Rect::new(0, 0, HEADER_WIDTH, HEADER_HEIGHT);
    self.canvas.set_draw_color(Color::RGB(44, 44, 55)); 
    self.canvas.fill_rect(header);
  }

  fn _draw(&mut self) {
    
  }
}