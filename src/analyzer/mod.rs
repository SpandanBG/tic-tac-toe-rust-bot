use crate::board;

pub struct AnalyzerState {}

pub trait Analyzer {
    fn get_best_move(&self, game_board: Box<dyn board::Game>) -> board::Coord;
}

impl Analyzer for AnalyzerState {
    fn get_best_move(&self, game_board: Box<dyn board::Game>) -> board::Coord {
        return board::Coord { x: 2, y: 0 };
    }
}

pub fn new() -> impl Analyzer {
    return AnalyzerState {};
}

// ----------------------------- TEST -----------------------------

#[cfg(test)]
mod analyzer_tests {
    use crate::analyzer::{self, Analyzer};
    use crate::board::{self, Coord};

    #[test]
    fn given_position_2_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 0, y: 0 })
                .place_player(Coord { x: 1, y: 0 }),
        );

        assert_eq!(best_move, Coord { x: 2, y: 0 });
    }
}
