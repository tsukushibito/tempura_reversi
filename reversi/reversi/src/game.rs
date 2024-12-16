use crate::{BitBoard, Board, Color, Move, Position};

#[derive(Debug)]
pub struct Game {
    board: Box<dyn Board + Send>,
    current_player: Color,
    move_count: u32,
    is_game_over: bool,
    move_history: Vec<Move>,
}

impl Game {
    pub fn new(
        board: Box<dyn Board + Send>,
        player: Color,
        move_count: u32,
        is_game_over: bool,
        move_history: Vec<Move>,
    ) -> Self {
        Self {
            board,
            current_player: player,
            move_count,
            is_game_over,
            move_history,
        }
    }

    pub fn initial() -> Self {
        Self {
            board: Box::new(BitBoard::init_board()),
            current_player: Color::Black,
            move_count: 0,
            is_game_over: false,
            move_history: Default::default(),
        }
    }

    pub fn board(&self) -> &(dyn Board + Send) {
        &*self.board
    }

    pub fn current_player(&self) -> Color {
        self.current_player
    }

    pub fn move_count(&self) -> u32 {
        self.move_count
    }

    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    pub fn move_history(&self) -> Vec<Move> {
        self.move_history.clone()
    }

    pub fn black_score(&self) -> usize {
        self.board().black_count()
    }

    pub fn white_score(&self) -> usize {
        self.board().white_count()
    }

    pub fn reset(&mut self) {
        self.board.init();
        self.current_player = Color::Black;
        self.move_count = 0;
        self.is_game_over = false;
        self.move_history.clear();
    }

    pub fn get_current_players_valid_moves(&self) -> Vec<Position> {
        self.board.get_valid_moves(self.current_player)
    }

    pub fn progress(&mut self, player: Color, pos: Position) -> Result<GameEvent, String> {
        if self.is_game_over {
            return Err("Already game over".to_string());
        }

        if player != self.current_player {
            return Err("Invalid player".to_string());
        }

        let mut board = self.board.clone_as_board();
        let success = board.make_move(player, &pos);
        if success {
            self.switch_turn();
            self.board = board;
            self.move_history.push(Move {
                position: pos,
                color: player,
            });
        } else {
            return Err("Invalid pos".to_string());
        }

        let valid_moves = self.get_current_players_valid_moves();
        if valid_moves.is_empty() {
            // パスなのでプレイヤー交代
            self.switch_turn();

            let valid_moves = self.board.get_valid_moves(self.current_player);
            if valid_moves.is_empty() {
                self.is_game_over = true;
                // 双方パスなので終了
                return Ok(GameEvent::GameOver(self.clone()));
            }
        }

        Ok(GameEvent::Turn(self.clone()))
    }

    fn switch_turn(&mut self) {
        self.current_player = self.current_player.opponent();
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self {
            board: self.board.clone_as_board(),
            current_player: self.current_player,
            move_count: self.move_count,
            is_game_over: self.is_game_over,
            move_history: self.move_history.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameEvent {
    Turn(Game),
    GameOver(Game),
}
