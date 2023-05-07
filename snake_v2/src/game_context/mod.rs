use std::ops::Add; 
use rand::Rng; 

#[derive(Debug)]
pub enum GamePlayState { Playing, Paused }

#[derive(Debug)]
pub enum PlayerDirection { Up, Down, Right, Left }

#[derive(Copy, Clone, Debug)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
  type Ouput = Point; 
  fn add(self, rhs: Point) -> Self::Output {
    Point(self.0 + rhs.0, self.1 + rhs.1)
  }
}

#[derive(Debug)]
pub struct GameContext {
  pub player_position: Vec<Point>, 
  pub player_direction: PlayerDirection, 
  pub food: Point, 
  pub play_state: GamePlayState, 
  pub score: u32, 
  pub game_over: bool, 
}

impl GameContext {
  pub fn new() -> GameContext {
    GameContext {
      player_position: vec![Point(2, 2), Point(3, 2), Point(4, 2)], 
      player_direction: PlayerDirection::Left, 
      food: Point(10, 12),
      play_state: GamePlayState::Paused,
      score: 0, 
      game_over: false,
    }
  }

  pub fn tick(&mut self) {

  }

  pub fn toggle_pause(&mut self) { 
    match self.play_state {
      GamePlayState::Paused => { self.play_state = GamePlayState.Playing },
      GamePlayState::Playing => { self.play_state = GamePlayState.Paused },
    }
  }
}