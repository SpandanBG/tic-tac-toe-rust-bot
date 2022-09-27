mod board;

#[derive(Debug)]
pub enum GameOverType {
  PLAYING = 0,
  DEFEAT = -1,
  VICTORY = 1,
}

pub struct GameState {
  game_over: GameOverType,
}
