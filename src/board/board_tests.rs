#[cfg(test)]
mod board_test {
    use crate::board::*;

    #[test]
    fn should_return_a_new_empty_board() {
        let game_state = new().get_board_state();

        assert_eq!(game_state.len(), 9);
        assert_eq!(
            game_state
                .iter()
                .find(|&cell| *cell == CellType::PLAYER_1 || *cell == CellType::PLAYER_2),
            None
        )
    }

    #[test]
    fn should_place_player_on_given_position() {
        let game_board = new();

        let update_game_board = game_board.place_player(Coord { x: 0, y: 1 });
        let game_state = update_game_board.get_board_state();

        assert_eq!(game_state[3], CellType::PLAYER_1);
    }

    #[test]
    fn should_not_place_bot_if_player_present_on_given_position() {
        let mut game_board = new();

        game_board = game_board.place_player(Coord { x: 0, y: 1 });

        assert_eq!(game_board.get_board_state()[3], CellType::PLAYER_1);

        game_board = game_board.place_bot(Coord { x: 0, y: 1 });

        assert_eq!(game_board.get_board_state()[3], CellType::PLAYER_1);
    }

    #[test]
    fn should_perform_valid_coord_to_position_translation() {
        assert_eq!(coord_to_position(&Coord { x: 0, y: 0 }), 0);
        assert_eq!(coord_to_position(&Coord { x: 1, y: 0 }), 1);
        assert_eq!(coord_to_position(&Coord { x: 2, y: 0 }), 2);
        assert_eq!(coord_to_position(&Coord { x: 0, y: 1 }), 3);
        assert_eq!(coord_to_position(&Coord { x: 1, y: 1 }), 4);
        assert_eq!(coord_to_position(&Coord { x: 2, y: 1 }), 5);
        assert_eq!(coord_to_position(&Coord { x: 0, y: 2 }), 6);
        assert_eq!(coord_to_position(&Coord { x: 1, y: 2 }), 7);
        assert_eq!(coord_to_position(&Coord { x: 2, y: 2 }), 8);
    }

    #[test]
    fn should_return_player_win_as_game_over_type() {
        let mut game_board = new();

        game_board = game_board.place_player(Coord { x: 0, y: 0 });
        game_board = game_board.place_player(Coord { x: 1, y: 1 });
        game_board = game_board.place_player(Coord { x: 2, y: 2 });

        let victor = game_board.is_game_over();

        assert_eq!(victor, GameOverType::PLAYER_1_WIN);
    }

    #[test]
    fn should_return_bot_win_as_game_over_type() {
        let mut game_board = new();

        game_board = game_board.place_bot(Coord { x: 2, y: 0 });
        game_board = game_board.place_bot(Coord { x: 1, y: 1 });
        game_board = game_board.place_bot(Coord { x: 0, y: 2 });

        let victor = game_board.is_game_over();

        assert_eq!(victor, GameOverType::PLAYER_2_WIN);
    }

    #[test]
    fn should_return_draw_as_game_over_type() {
        let mut game_board = new();

        game_board = game_board.place_player(Coord { x: 0, y: 0 });
        game_board = game_board.place_bot(Coord { x: 1, y: 0 });
        game_board = game_board.place_player(Coord { x: 2, y: 0 });

        game_board = game_board.place_player(Coord { x: 0, y: 1 });
        game_board = game_board.place_bot(Coord { x: 1, y: 1 });
        game_board = game_board.place_player(Coord { x: 2, y: 1 });

        game_board = game_board.place_bot(Coord { x: 0, y: 2 });
        game_board = game_board.place_player(Coord { x: 1, y: 2 });
        game_board = game_board.place_bot(Coord { x: 2, y: 2 });

        let victor = game_board.is_game_over();

        assert_eq!(victor, GameOverType::DRAW);
    }
}
