#[cfg(test)]
mod analyzer_tests {
    use crate::analyzer::{self, Analyzer};
    use crate::board::{self, CellType, Coord};

    struct TwoPositionTest {
        position_1: Coord,
        position_2: Coord,
        player_cell_type: CellType,
        expected_position_to_play: Coord,
    }

    macro_rules! two_position_played_table_tests {
        ( $( $name:ident: $value:expr ),* ) => {
        $(
            #[test]
            fn $name() {
                let game_analyzer = analyzer::new();
                let game_board = board::new();
                let test_data: TwoPositionTest = $value;

                let best_move = game_analyzer.get_best_move(
                    game_board
                        .set_player(test_data.position_1, test_data.player_cell_type)
                        .set_player(test_data.position_2, test_data.player_cell_type),
                    test_data.player_cell_type
                );

                assert_eq!(best_move, Some(test_data.expected_position_to_play));
            }
        )*
        };
    }

    two_position_played_table_tests! {
        given_position_2_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 0, y: 0},
            position_2: Coord { x: 1, y: 0},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 2, y: 0 }
        },

        given_position_1_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 0, y: 0},
            position_2: Coord { x: 2, y: 0},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 1, y: 0 }
        },

        given_position_0_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 1, y: 0},
            position_2: Coord { x: 2, y: 0},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 0, y: 0 }
        },

        given_position_5_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 0, y: 1},
            position_2: Coord { x: 1, y: 1},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 2, y: 1 }
        },

        given_position_4_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 0, y: 1},
            position_2: Coord { x: 2, y: 1},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 1, y: 1 }
        },

        given_position_3_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 1, y: 1},
            position_2: Coord { x: 2, y: 1},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 0, y: 1 }
        },

        given_position_8_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 0, y: 2},
            position_2: Coord { x: 1, y: 2},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 2, y: 2 }
        },

        given_position_7_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 0, y: 2},
            position_2: Coord { x: 2, y: 2},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 1, y: 2 }
        },

        given_position_6_winning_then_return_winning_coordinate: TwoPositionTest {
            position_1: Coord { x: 1, y: 2},
            position_2: Coord { x: 2, y: 2},
            player_cell_type: CellType::PLAYER_1,
            expected_position_to_play: Coord { x: 0, y: 2 }
        }
    }
}
