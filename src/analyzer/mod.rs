use crate::board::{self, Game};

pub struct Analyzer {
  board: Box<dyn Game>
}

fn setup() -> Analyzer {
  let board = board::new(); 

  return Analyzer {
    board
  }
}
