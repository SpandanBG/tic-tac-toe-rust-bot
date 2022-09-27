#[cfg(test)]
mod analyzer_tests {
    use crate::analyzer::{self, Analyzer};
    use crate::board::{self, CellType, Coord};

    #[test]
    fn given_position_2_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 0, y: 0 })
                .place_player(Coord { x: 1, y: 0 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 2, y: 0 }));
    }

    #[test]
    fn given_position_1_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 0, y: 0 })
                .place_player(Coord { x: 2, y: 0 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 1, y: 0 }));
    }

    #[test]
    fn given_position_0_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 1, y: 0 })
                .place_player(Coord { x: 2, y: 0 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 0, y: 0 }));
    }

    #[test]
    fn given_position_5_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 0, y: 1 })
                .place_player(Coord { x: 1, y: 1 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 2, y: 1 }));
    }

    #[test]
    fn given_position_4_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 0, y: 1 })
                .place_player(Coord { x: 2, y: 1 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 1, y: 1 }));
    }

    #[test]
    fn given_position_3_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 1, y: 1 })
                .place_player(Coord { x: 2, y: 1 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 0, y: 1 }));
    }

    #[test]
    fn given_position_8_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 0, y: 2 })
                .place_player(Coord { x: 1, y: 2 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 2, y: 2 }));
    }

    #[test]
    fn given_position_7_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 0, y: 2 })
                .place_player(Coord { x: 2, y: 2 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 1, y: 2 }));
    }

    #[test]
    fn given_position_6_winning_then_return_winning_coordinate() {
        let game_analyzer = analyzer::new();
        let game_board = board::new();

        let best_move = game_analyzer.get_best_move(
            game_board
                .place_player(Coord { x: 1, y: 2 })
                .place_player(Coord { x: 2, y: 2 }),
            CellType::PLAYER_1,
        );

        assert_eq!(best_move, Some(Coord { x: 0, y: 2 }));
    }
}
