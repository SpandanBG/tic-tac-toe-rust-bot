use crate::board;

pub struct AnalyzerState {}

pub trait Analyzer {
  fn get_best_move(game_board: Box<dyn board::Game>) -> board::Coord;
}

impl Analyzer for AnalyzerState {

  fn get_best_move(game_board: Box<dyn board::Game>) -> board::Coord {
    return board::Coord { x: 1, y: 1 };
  }

}

pub fn new() -> Box<impl Analyzer> {
  return Box::new(AnalyzerState{});
}


// ----------------------------- TEST ----------------------------- 

#[cfg(test)]
mod tests {

  

}
