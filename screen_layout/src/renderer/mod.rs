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
    let row_num = 600 / 6;
    let col_num = 800 / 8; 
    println!("r{row_num} c{col_num}");
    for row in 0..row_num {
      for col in 0..col_num {
        println!("{col}");
        let x: i32 = ((row_num * row) + 10).try_into().unwrap(); 
        let y: i32 = ((col_num * col) + 10).try_into().unwrap();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.fill_rect(Rect::new(x, y, 100 - 10, 100 - 10));
      }
    }
  }
}