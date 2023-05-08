use std::ops::Add; 
use rand::Rng; 

#[derive(Debug)]
pub enum GamePlayState { Playing, Paused }

#[derive(Debug)]
pub enum PlayerDirection { Up, Down, Right, Left }

#[derive(Copy, Clone, Debug)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
  type Output = Point;
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
      player_position: vec![Point(2, 4), Point(2, 3), Point(2, 2)], 
      player_direction: PlayerDirection::Right, 
      food: Point(10, 12),
      play_state: GamePlayState::Paused,
      score: 0, 
      game_over: false,
    }
  }

  pub fn tick(&mut self) {
    if self.play_state == GamePlayState::Paused { return; }
    let next_head_position: Point = self.get_next_head_positon();

    // Check for collision with food 
    // Check for Pause state 
    // Check for collision with self

    self.player_position.pop();
    self.player_position.reverse();
    self.player_position.push(next_head_position);
    self.player_position.reverse();
  }

  fn check_food_collison(&mut self, next_head_position: &Point) {
    
  }

  pub fn toggle_pause(&mut self) { 
    match self.play_state {
      GamePlayState::Paused => { self.play_state = GamePlayState::Playing },
      GamePlayState::Playing => { self.play_state = GamePlayState::Paused },
    }
  }

  pub fn move_down(&mut self) {
    match self.player_direction {
      PlayerDirection::Up => {},
      PlayerDirection::Down => {}
      _ => { self.player_direction = PlayerDirection::Down }
    }
  }

  pub fn move_up(&mut self) {
    match self.player_direction {
      PlayerDirection::Up => {},
      PlayerDirection::Down => {},
      _ => { self.player_direction = PlayerDirection::Up }
    }
  }

  pub fn move_right(&mut self) {
    match self.player_direction {
      PlayerDirection::Left => {},
      PlayerDirection::Right => {},
      _ => { self.player_direction = PlayerDirection::Right }
    }
  }

  pub fn move_left(&mut self) {
    match self.player_direction {
      PlayerDirection::Left => {},
      PlayerDirection::Right => {},
      _ => { self.player_direction = PlayerDirection::Left }
    }
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
}