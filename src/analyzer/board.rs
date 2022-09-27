mod board {
    #[derive(Debug, Clone, PartialEq)]
    pub enum CellType {
        NON = 0,
        PLAYER = 1,
        BOT = -1,
    }

    #[derive(PartialEq)]
    pub enum GameOverType {
        PLAYING = 0,
        VICTORY = 1,
        DEFEAT = -1,
    }

    pub type Board = Vec<CellType>;

    #[derive(Debug, PartialEq)]
    pub struct Coord {
        pub x: isize,
        pub y: isize,
    }

    impl Coord {
        fn is_valid(&self) -> bool {
            let valid_x = !(self.x < 0 || self.x > 2);
            let valid_y = !(self.y < 0 || self.y > 2);
            return valid_x && valid_y;
        }
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
            let mut point_stack: Vec<CellType> = Vec::new();
            let mut game_over_type = GameOverType::PLAYING;

            for position in [0, 1, 2, 3, 6] {
                let cell_type: &CellType = &self[position];
                let coord: Coord = position_to_coord(&position);
                game_over_type = match find_victor(self, coord, cell_type, 1) {
                    CellType::PLAYER => GameOverType::VICTORY,
                    CellType::BOT => GameOverType::DEFEAT,
                    CellType::NON => GameOverType::PLAYING,
                };

                if game_over_type != GameOverType::PLAYING {
                    break;
                }

                point_stack.pop();
            }

            return game_over_type;
        }

        fn place_bot(&self, coord: Coord) -> Box<dyn Game> {
            return Box::new(update_cell_type(self, coord, CellType::BOT));
        }

        fn place_player(&self, coord: Coord) -> Box<dyn Game> {
            return Box::new(update_cell_type(self, coord, CellType::PLAYER));
        }
    }

    pub fn new() -> Box<dyn Game> {
        Box::new(vec![CellType::NON; 9])
    }

    fn find_victor(board: &Board, coord: Coord, cell_type: &CellType, depth: u8) -> CellType {
        if !coord.is_valid() {
            return CellType::NON;
        }
        let position: usize = coord_to_position(&coord);

        let position_cell_type: &CellType = &board[position];
        if *position_cell_type != *cell_type {
            return CellType::NON;
        }

        if depth == 3 {
            return cell_type.clone();
        }

        let victor: CellType = get_prioritized_cell_type(vec![
            find_victor(
                board,
                Coord {
                    x: coord.x + 1,
                    ..coord
                },
                cell_type,
                depth + 1,
            ),
            find_victor(
                board,
                Coord {
                    y: coord.y + 1,
                    ..coord
                },
                cell_type,
                depth + 1,
            ),
            find_victor(
                board,
                Coord {
                    x: coord.x + 1,
                    y: coord.y + 1,
                },
                cell_type,
                depth + 1,
            ),
            find_victor(
                board,
                Coord {
                    x: coord.x + 1,
                    y: coord.y - 1,
                },
                cell_type,
                depth + 1,
            ),
        ]);

        return victor;
    }

    fn position_to_coord(position: &usize) -> Coord {
        let x: isize = (*position as isize) % 3;
        let y: isize = (*position as isize) / 3;
        return Coord { x, y };
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

    fn get_prioritized_cell_type(cells: Vec<CellType>) -> CellType {
        return match cells.iter().find(|&cell| *cell != CellType::NON) {
            Some(cell_type) => cell_type.clone(),
            _ => CellType::NON,
        };
    }


    #[cfg(test)]
    mod test {
        use super::*;

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

            let update_game_board = game_board.place_player(Coord { x: 1, y: 1 });
            let game_state = update_game_board.get_board_state();

            assert_eq!(game_state[4], CellType::PLAYER);
        }

        #[test]
        fn should_not_place_bot_if_player_present_on_given_position() {
            let mut game_board = new();

            game_board = game_board.place_player(Coord { x: 1, y: 1 });

            assert_eq!(game_board.get_board_state()[4], CellType::PLAYER);

            game_board = game_board.place_bot(Coord { x: 1, y: 1 });

            assert_eq!(game_board.get_board_state()[4], CellType::PLAYER);
        }

        #[test]
        fn should_perform_valid_coord_to_position_translation() {
            assert_eq!(coord_to_position(&Coord{ x: 0, y: 0 }), 0);
            assert_eq!(coord_to_position(&Coord{ x: 1, y: 0 }), 1);
            assert_eq!(coord_to_position(&Coord{ x: 2, y: 0 }), 2);
            assert_eq!(coord_to_position(&Coord{ x: 0, y: 1 }), 3);
            assert_eq!(coord_to_position(&Coord{ x: 1, y: 1 }), 4);
            assert_eq!(coord_to_position(&Coord{ x: 2, y: 1 }), 5);
            assert_eq!(coord_to_position(&Coord{ x: 0, y: 2 }), 6);
            assert_eq!(coord_to_position(&Coord{ x: 1, y: 2 }), 7);
            assert_eq!(coord_to_position(&Coord{ x: 2, y: 2 }), 8);
        }

        #[test]
        fn should_perform_valid_position_to_coord_translation() {
            assert_eq!(position_to_coord(&0), Coord { x: 0, y: 0 });
            assert_eq!(position_to_coord(&1), Coord { x: 1, y: 0 });
            assert_eq!(position_to_coord(&2), Coord { x: 2, y: 0 });
            assert_eq!(position_to_coord(&3), Coord { x: 0, y: 1 });
            assert_eq!(position_to_coord(&4), Coord { x: 1, y: 1 });
            assert_eq!(position_to_coord(&5), Coord { x: 2, y: 1 });
            assert_eq!(position_to_coord(&6), Coord { x: 0, y: 2 });
            assert_eq!(position_to_coord(&7), Coord { x: 1, y: 2 });
            assert_eq!(position_to_coord(&8), Coord { x: 2, y: 2 });
        }

        #[test]
        fn should_return_the_first_non_empty_cell() {
            let board = vec![CellType::NON, CellType::PLAYER, CellType::BOT];

            assert_eq!(get_prioritized_cell_type(board), CellType::PLAYER);
        }

    }
}
