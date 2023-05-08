pub enum UiNumber {
  Zero,
  One, 
  Two, 
  Three, 
  Four, 
  Five, 
  Six, 
  Seven, 
  Eight, 
  Nine,
}

pub struct UiNumberDisplay {
  pub current_num: u8,
  pub matrix_representation: [[u8; 5]; 7]
}

impl UiNumberDisplay {
  pub fn new(number: UiNumber) -> UiNumberDisplay {
    get_number(number)
  }
}

fn get_number(number: UiNumber) -> UiNumberDisplay {
  match number {
    UiNumber::Zero => {
      UiNumberDisplay {
        current_num: 0, 
        matrix_representation: [
          [0, 1, 1, 1, 0],
          [1, 1, 0, 1, 1],
          [1, 1, 0, 1, 1],
          [1, 1, 0, 1, 1],
          [1, 1, 0, 1, 1],
          [1, 1, 0, 1, 1],
          [0, 1, 1, 1, 0],
        ]
      }
    },
    UiNumber::One => {
      UiNumberDisplay {
        current_num: 1, 
        matrix_representation: [
          [0, 0, 1, 1, 0],
          [0, 1, 1, 1, 0],
          [0, 0, 1, 1, 0],
          [0, 0, 1, 1, 0],
          [0, 0, 1, 1, 0],
          [0, 0, 1, 1, 0],
          [0, 1, 1, 1, 1],
        ]
      }
    },
    UiNumber::Two => {
      UiNumberDisplay {
        current_num: 2, 
        matrix_representation: [
          [0, 1, 1, 1, 0],
          [1, 1, 0, 1, 1],
          [0, 0, 0, 1, 1],
          [0, 1, 1, 1, 0],
          [1, 1, 0, 0, 0],
          [1, 1, 0, 0, 0],
          [0, 1, 1, 1, 1],
        ]
      }
    },
    UiNumber::Three => {
      UiNumberDisplay {
        current_num: 3, 
        matrix_representation: [
          [0, 1, 1, 1, 0],
          [1, 1, 0, 1, 1],
          [0, 0, 0, 1, 1],
          [0, 1, 1, 1, 0],
          [0, 0, 0, 1, 0],
          [1, 1, 0, 1, 1],
          [0, 1, 1, 1, 1],
        ]
      }
    },
    UiNumber::Four => {
      UiNumberDisplay {
        current_num: 4, 
        matrix_representation: [
          [1, 1, 0, 1, 1],
          [0, 1, 0, 0, 1],
          [0, 1, 0, 0, 1],
          [0, 1, 1, 1, 1],
          [0, 0, 0, 0, 1],
          [0, 0, 0, 0, 1],
          [0, 0, 0, 0, 1],
        ]
      }
    },
    UiNumber::Five => {
      UiNumberDisplay {
        current_num: 5, 
        matrix_representation: [
          [1, 1, 1, 1, 1],
          [1, 0, 0, 0, 0],
          [1, 0, 0, 0, 0],
          [1, 1, 1, 1, 0],
          [0, 0, 0, 0, 1],
          [0, 0, 0, 0, 1],
          [1, 1, 1, 1, 0],
        ]
      }
    },
    UiNumber::Six => {
      UiNumberDisplay {
        current_num: 6, 
        matrix_representation: [
          [0, 1, 1, 1, 0],
          [1, 0, 0, 0, 1],
          [1, 0, 0, 0, 0],
          [1, 1, 1, 1, 0],
          [1, 0, 0, 0, 1],
          [1, 0, 0, 0, 1],
          [0, 1, 1, 1, 0],
        ]
      }
    },
    UiNumber::Seven => {
      UiNumberDisplay {
        current_num: 7, 
        matrix_representation: [
          [1, 1, 1, 1, 1],
          [0, 0, 0, 1, 1],
          [0, 0, 0, 1, 0],
          [0, 0, 1, 1, 0],
          [0, 1, 1, 0, 0],
          [0, 1, 1, 0, 0],
          [0, 1, 1, 0, 0],
        ]
      }
    },
    UiNumber::Eight => {
      UiNumberDisplay {
        current_num: 0, 
        matrix_representation: [
          [0, 1, 1, 1, 0],
          [1, 1, 0, 1, 1],
          [1, 1, 0, 1, 1],
          [0, 1, 1, 1, 0],
          [1, 1, 0, 1, 1],
          [1, 1, 0, 1, 1],
          [0, 1, 1, 1, 0],
        ]
      }
    },
    UiNumber::Nine => {
      UiNumberDisplay {
        current_num: 0, 
        matrix_representation: [
          [0, 1, 1, 1, 0],
          [1, 1, 0, 1, 1],
          [1, 1, 0, 1, 1],
          [0, 1, 1, 1, 1],
          [0, 0, 0, 1, 1],
          [0, 0, 0, 1, 1],
          [1, 1, 1, 1, 0],
        ]
      }
    },
  }
}