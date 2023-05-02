use std::ops::Add;
use rand::Rng; 

use crate::GRID_X_SIZE; 
use crate::GRID_Y_SIZE;

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
      food: Point(10, 12)
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
    let x = rand::thread_rng().gen_range(1..=GRID_X_SIZE - 1);
    let y = rand::thread_rng().gen_range(1..=GRID_Y_SIZE - 1);
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