pub use sdl2::render::WindowCanvas;
use sdl2::video::Window; 
use crate::Color; 
use crate::Rect; 


pub struct Renderer { canvas: WindowCanvas }

impl Renderer {
  pub fn new(window: Window) -> Result<Renderer, String> {
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    Ok(Renderer { canvas })
  }

  pub fn draw(&mut self) -> Result<(), String> {
    self.draw_background();
    self.canvas.present();
    Ok(())
  }

  pub fn draw_background(&mut self) {
    let color = Color::RGB(28, 63, 98);
    self.canvas.set_draw_color(color);
    self.canvas.clear();
    self.draw_again(0);
    self.draw_again(110);
    self.draw_again(220);
    self.draw_again(330);
    self.draw_again(440);
  }

  fn draw_again(&mut self, offset: u32) {
    let row_num = 600 / 60;
    let col_num = 800 / 80; 

    for row in 0..row_num {
      for col in 0..col_num {
        let x: i32 = ((row_num * row) + offset).try_into().unwrap(); 
        let y: i32 = ((col_num * col) + 10).try_into().unwrap();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.fill_rect(Rect::new(x + 10, y + 10, 15 - 10, 15 - 10));
      }
    }
  }
}