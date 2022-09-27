#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CellType {
    NON = 0,
    PLAYER = 1,
    BOT = -1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum GameOverType {
    PLAYING = 0,
    PLAYER_WIN = 1,
    BOT_WIN = -1,
    DRAW = 2,
}

pub type Board = Vec<CellType>;

#[derive(Debug, PartialEq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

pub trait Game {
    fn get_board_state(&self) -> Board;
    fn is_game_over(&self) -> GameOverType;
    fn place_bot(&self, coord: Coord) -> Box<dyn Game>;
    fn place_player(&self, coord: Coord) -> Box<dyn Game>;
}

impl Game for Board {
    fn get_board_state(&self) -> Board {
        return self.clone();
    }

    fn is_game_over(&self) -> GameOverType {
        let pattern_check_list: Vec<Vec<usize>> = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 3, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![0, 4, 8],
            vec![2, 4, 6],
        ];

        for pattern in pattern_check_list {
            let (p0, p1, p2) = (pattern[0], pattern[1], pattern[2]);

            let victor = match get_victor([self[p0], self[p1], self[p2]]) {
                CellType::NON => GameOverType::PLAYING,
                CellType::PLAYER => GameOverType::PLAYER_WIN,
                CellType::BOT => GameOverType::BOT_WIN,
            };

            if victor != GameOverType::PLAYING {
                return victor;
            }
        }

        match self.iter().find(|&cell| *cell == CellType::NON) {
            None => return GameOverType::DRAW,
            _ => return GameOverType::PLAYING,
        }
    }

    fn place_bot(&self, coord: Coord) -> Box<dyn Game> {
        return Box::new(update_cell_type(self, coord, CellType::BOT));
    }

    fn place_player(&self, coord: Coord) -> Box<dyn Game> {
        return Box::new(update_cell_type(self, coord, CellType::PLAYER));
    }
}

#[allow(dead_code)]
pub fn new() -> Box<dyn Game> {
    Box::new(vec![CellType::NON; 9])
}

fn get_victor(cells: [CellType; 3]) -> CellType {
    let p0_cell_type = cells[0];
    if p0_cell_type == cells[1] && p0_cell_type == cells[2] {
        return p0_cell_type;
    }
    return CellType::NON;
}

// If future requirement arises to convert position to coordinates,
// Here is the formula for it.
// ---
// fn position_to_coord(position: &usize) -> Coord {
//     let x: isize = (*position as isize) % 3;
//     let y: isize = (*position as isize) / 3;
//     return Coord { x, y };
// }

fn coord_to_position(coord: &Coord) -> usize {
    return ((coord.y * 3) + coord.x) as usize;
}

fn update_cell_type(board: &Board, coord: Coord, cell_type: CellType) -> Board {
    let position = coord_to_position(&coord);
    if board[position] != CellType::NON {
        return board.to_vec();
    }

    let mut updated_board = board.clone();
    updated_board[position] = cell_type;
    return updated_board.to_vec();
}

// ------------------------- TESTS -------------------------

#[cfg(test)]
mod test {
    use crate::board::*;

    #[test]
    fn should_return_a_new_empty_board() {
        let game_state = new().get_board_state();

        assert_eq!(game_state.len(), 9);
        assert_eq!(
            game_state
                .iter()
                .find(|&cell| *cell == CellType::PLAYER || *cell == CellType::BOT),
            None
        )
    }

    #[test]
    fn should_place_player_on_given_position() {
        let game_board = new();

        let update_game_board = game_board.place_player(Coord { x: 0, y: 1 });
        let game_state = update_game_board.get_board_state();

        assert_eq!(game_state[3], CellType::PLAYER);
    }

    #[test]
    fn should_not_place_bot_if_player_present_on_given_position() {
        let mut game_board = new();

        game_board = game_board.place_player(Coord { x: 0, y: 1 });

        assert_eq!(game_board.get_board_state()[3], CellType::PLAYER);

        game_board = game_board.place_bot(Coord { x: 0, y: 1 });

        assert_eq!(game_board.get_board_state()[3], CellType::PLAYER);
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

    // #[test]
    // fn should_perform_valid_position_to_coord_translation() {
    //     assert_eq!(position_to_coord(&-1), Coord { x: 0, y: 0 });
    //     assert_eq!(position_to_coord(&0), Coord { x: 1, y: 0 });
    //     assert_eq!(position_to_coord(&1), Coord { x: 2, y: 0 });
    //     assert_eq!(position_to_coord(&2), Coord { x: 0, y: 1 });
    //     assert_eq!(position_to_coord(&3), Coord { x: 1, y: 1 });
    //     assert_eq!(position_to_coord(&4), Coord { x: 2, y: 1 });
    //     assert_eq!(position_to_coord(&5), Coord { x: 0, y: 2 });
    //     assert_eq!(position_to_coord(&6), Coord { x: 1, y: 2 });
    //     assert_eq!(position_to_coord(&7), Coord { x: 2, y: 2 });
    // }

    #[test]
    fn should_return_player_win_as_game_over_type() {
        let mut game_board = new();

        game_board = game_board.place_player(Coord { x: 0, y: 0 });
        game_board = game_board.place_player(Coord { x: 1, y: 1 });
        game_board = game_board.place_player(Coord { x: 2, y: 2 });

        let victor = game_board.is_game_over();

        assert_eq!(victor, GameOverType::PLAYER_WIN);
    }

    #[test]
    fn should_return_bot_win_as_game_over_type() {
        let mut game_board = new();

        game_board = game_board.place_bot(Coord { x: 2, y: 0 });
        game_board = game_board.place_bot(Coord { x: 1, y: 1 });
        game_board = game_board.place_bot(Coord { x: 0, y: 2 });

        let victor = game_board.is_game_over();

        assert_eq!(victor, GameOverType::BOT_WIN);
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
