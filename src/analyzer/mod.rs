mod analyzer_tests;

use crate::board::{self, CellType, Coord};

pub struct AnalyzerState {}

pub trait Analyzer {
    fn get_best_move(
        &self,
        game_board: Box<dyn board::Game>,
        for_cell_type: CellType,
    ) -> Option<Coord>;
}

impl Analyzer for AnalyzerState {
    fn get_best_move(
        &self,
        game_board: Box<dyn board::Game>,
        for_cell_type: CellType,
    ) -> Option<Coord> {
        let board_state = game_board.get_board_state();
        let pattern_check_list: Vec<[usize; 3]> = vec![
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for pattern in pattern_check_list {
            match get_winning_position(&board_state, for_cell_type, pattern) {
                Some(position) => return Some(position_to_coord(&position)),
                _ => (),
            };

            match get_defending_position(&board_state, for_cell_type, pattern) {
                Some(position) => return Some(position_to_coord(&position)),
                _ => (),
            }
        }

        return None;
    }
}

pub fn new() -> impl Analyzer {
    return AnalyzerState {};
}

fn get_winning_position(
    board: &Vec<CellType>,
    for_cell_type: CellType,
    positions: [usize; 3],
) -> Option<usize> {
    let mut selected_position = None;
    let mut cell_type_match_count: u8 = 0;

    for position in positions {
        if board[position] != for_cell_type && board[position] != CellType::NON {
            return None;
        }

        if board[position] == CellType::NON {
            selected_position = Some(position);
            continue;
        }

        if board[position] == for_cell_type {
            cell_type_match_count += 1;
        }
    }

    if cell_type_match_count != 2 {
        return None;
    }
    return selected_position;
}

fn get_defending_position(
    board: &Vec<CellType>,
    for_cell_type: CellType,
    positions: [usize; 3],
) -> Option<usize> {
    let mut selected_position = None;
    let mut cell_type_match_count: u8 = 0;

    for position in positions {
        if board[position] == for_cell_type {
            return None;
        }

        if board[position] == CellType::NON {
            selected_position = Some(position);
            continue;
        }

        if board[position] != for_cell_type {
            cell_type_match_count += 1;
        }
    }

    if cell_type_match_count != 2 {
        return None;
    }
    return selected_position;
}
fn position_to_coord(position: &usize) -> board::Coord {
    let x: isize = (*position as isize) % 3;
    let y: isize = (*position as isize) / 3;
    return board::Coord { x, y };
}
