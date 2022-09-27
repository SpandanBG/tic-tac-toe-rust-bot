mod board_tests;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CellType {
    NON = 0,
    PLAYER_1 = 1,
    PLAYER_2 = -1,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum GameOverType {
    PLAYING = 0,
    PLAYER_1_WIN = 1,
    PLAYER_2_WIN = -1,
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
    fn set_player(&self, coord: Coord, cell_type: CellType) -> Box<dyn Game>;
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
                CellType::PLAYER_1 => GameOverType::PLAYER_1_WIN,
                CellType::PLAYER_2 => GameOverType::PLAYER_2_WIN,
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

    fn set_player(&self, coord: Coord, cell_type: CellType) -> Box<dyn Game> {
        return Box::new(update_cell_type(self, coord, cell_type));
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
