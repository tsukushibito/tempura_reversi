use crate::{ai::Ai, bit_board::BitBoard, board::Board, Color, Move, Position};

#[derive(Copy, Clone, Debug)]
pub enum CellState {
    Empty,
    Black,
    White,
}

#[derive(Debug)]
pub struct Game {
    board: Box<dyn Board + Send>,
    current_player: Color,
    move_count: u32,
    is_game_over: bool,
    last_move: Option<Move>,
}

impl Game {
    pub fn new(
        board: Box<dyn Board + Send>,
        player: Color,
        move_count: u32,
        is_game_over: bool,
        last_move: Option<Move>,
    ) -> Self {
        Self {
            board,
            current_player: player,
            move_count,
            is_game_over,
            last_move,
        }
    }

    pub fn initial() -> Self {
        Self {
            board: Box::new(BitBoard::init_board()),
            current_player: Color::Black,
            move_count: 0,
            is_game_over: false,
            last_move: None,
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

    pub fn last_move(&self) -> Option<Move> {
        self.last_move
    }

    pub fn reset(&mut self) {
        self.board.init();
        self.current_player = Color::Black;
        self.move_count = 0;
        self.is_game_over = false;
        self.last_move = None;
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
            self.last_move = Some(Move {
                position: pos,
                color: player,
            })
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

    pub fn self_play() {
        let mut ai_1 = Ai::new();
        let mut ai_2 = Ai::new();
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
            last_move: self.last_move,
        }
    }
}

/// ゲーム結果を表す列挙型
#[derive(Debug, Clone)]
pub enum GameResult {
    BlackWins(usize, usize),
    WhiteWins(usize, usize),
    Draw(usize, usize),
}

#[derive(Debug, Clone)]
pub enum GameEvent {
    Turn(Game),
    GameOver(Game),
}
