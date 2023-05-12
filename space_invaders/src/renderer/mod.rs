use sdl2::pixels::Color; 
use sdl2::render::WindowCanvas; 
use sdl2::video::Window; 
use sdl2::rect::Rect; 
use sdl2::rect::Point; 
use sdl2::render::TextureCreator;
use sdl2::surface::Surface; 
use std::path::Path;
use crate::SPRITE_TILE_SIZE;

pub struct Renderer<T> { 
  canvas: WindowCanvas,
  texture: TextureCreator<T>,
}

impl Renderer<> {
  pub fn new(window: Window) -> Result<Renderer<T>, String> {
    let canvas = window
                .into_canvas()
                .accelerated()
                .build()
                .map_err(|e| e.to_string())?;

    let surface = Surface::load_bmp(Path::new("assets/characters.bmp"))?;
    let texture_creator = canvas.texture_creator()?;
    Ok(Renderer { canvas: canvas, texture: texture_creator })
  }

  pub fn initialize_window(&mut self) -> Result<(), String> {
    self.canvas.set_draw_color(Color::BLACK); 
    self.canvas.clear();
    self.canvas.present();
    Ok(())
  }
}