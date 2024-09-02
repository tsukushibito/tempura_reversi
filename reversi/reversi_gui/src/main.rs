mod board;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use board::BoardView;
use iced::{
    alignment::Vertical,
    futures::{channel::mpsc, Stream},
    widget::{button, canvas, column, pick_list, row, text},
    Element, Length, Settings, Subscription, Task, Theme,
};
use reversi::{Ai, BitBoard, Board, BoardState, Game};

pub fn main() -> iced::Result {
    iced::application("Tempura Reversi", Reversi::update, Reversi::view)
        .theme(Reversi::theme)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .subscription(Reversi::subscription)
        .run_with(Reversi::new)
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    #[default]
    Human,
    Ai,
}

impl PlayerType {
    pub const ALL: [PlayerType; 2] = [PlayerType::Human, PlayerType::Ai];
}
impl std::fmt::Display for PlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayerType::Human => "Human",
                PlayerType::Ai => "AI",
            }
        )
    }
}

struct Reversi {
    pub stones_cache: canvas::Cache,
    pub game: Game,
    pub sender_to_ai_worker: Option<mpsc::Sender<Message>>,
    pub black_player_type: Option<PlayerType>,
    pub white_player_type: Option<PlayerType>,
    pub next_request_ai_move_id: i32,
    pub waiting_requests: Vec<AiMoveRequest>,
}

#[derive(Debug, Clone, Copy)]
struct AiMoveRequest {
    pub id: i32,
    pub board: BoardState,
    pub player: reversi::Color,
}

#[derive(Debug, Clone)]
enum Message {
    AiWorkerAwaked(mpsc::Sender<Message>),
    AiMove(AiMoveRequest),
    MoveMaked {
        pos: reversi::Position,
        request_id: i32,
    },
    Reset,
    BlackPlayerTypeChanged(PlayerType),
    WhitePlayerTypeChanged(PlayerType),
}

impl Reversi {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                stones_cache: canvas::Cache::default(),
                game: Game::initial(),
                sender_to_ai_worker: None,
                black_player_type: Some(PlayerType::Human),
                white_player_type: Some(PlayerType::Ai),
                next_request_ai_move_id: 0,
                waiting_requests: vec![],
            },
            iced::widget::focus_next(),
        )
    }

    fn update(&mut self, message: Message) {
        println!("update()");
        match message {
            Message::AiWorkerAwaked(sender) => {
                println!("AiWorkerAwaked");
                self.sender_to_ai_worker = Some(sender);
                self.send_request_if_turn_is_ai();
            }
            Message::MoveMaked { pos, request_id } => {
                println!("[MoveMaked] move: ({}, {})", pos.x, pos.y);
                if self.game.is_game_over() {
                    return;
                }

                if request_id < 0 {
                    // GUIからの着手なのでそのまま反映
                } else if self.waiting_requests.iter().any(|req| req.id == request_id) {
                    // 応答待ちリクエストからの着手なので待ちリストから削除して反映
                    if let Some(index) = self
                        .waiting_requests
                        .iter()
                        .position(|req| req.id == request_id)
                    {
                        self.waiting_requests.remove(index);
                    }
                } else {
                    // 応答待ち以外のリクエストからの着手なので反映しない
                    return;
                }

                let player = self.game.current_player();
                let _ = self.game.progress(player, pos);
                self.stones_cache.clear();
                self.send_request_if_turn_is_ai();
            }
            Message::AiMove(_) => panic!(),
            Message::BlackPlayerTypeChanged(player_type) => {
                self.black_player_type = Some(player_type);
                if player_type == PlayerType::Human {
                    self.waiting_requests
                        .retain(|&req| req.player == reversi::Color::White)
                }
                self.send_request_if_turn_is_ai();
            }
            Message::WhitePlayerTypeChanged(player_type) => {
                self.white_player_type = Some(player_type);
                if player_type == PlayerType::Human {
                    self.waiting_requests
                        .retain(|&req| req.player == reversi::Color::Black)
                }
                self.send_request_if_turn_is_ai();
            }
            Message::Reset => {
                self.game.reset();
                self.stones_cache.clear();
                self.send_request_if_turn_is_ai();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let player = self.game.current_player();
        let player_type = match player {
            reversi::Color::Black => self.black_player_type,
            reversi::Color::White => self.white_player_type,
        };
        let is_human_turn = match player_type {
            Some(PlayerType::Human) => true,
            Some(PlayerType::Ai) => false,
            None => true,
        };
        row![
            canvas(BoardView {
                stones_cache: &self.stones_cache,
                board: self.game.board().board_state(),
                is_clickable: is_human_turn,
            })
            .width(Length::FillPortion(2))
            .height(Length::Fill),
            column![
                text(format!("Black: {}", self.game.board().black_count()))
                    .width(Length::FillPortion(1)),
                text(format!("White: {}", self.game.board().white_count()))
                    .width(Length::FillPortion(1)),
                text(format!("Turn: {:?}", self.game.current_player()))
                    .width(Length::FillPortion(1)),
                row![
                    text("Black player type: "),
                    pick_list(
                        PlayerType::ALL,
                        self.black_player_type,
                        Message::BlackPlayerTypeChanged,
                    )
                    .padding(10),
                ]
                .align_y(Vertical::Center),
                row![
                    text("White player type: "),
                    pick_list(
                        PlayerType::ALL,
                        self.white_player_type,
                        Message::WhitePlayerTypeChanged,
                    )
                    .padding(10),
                ]
                .align_y(Vertical::Center),
                button("Reset").padding(10).on_press(Message::Reset),
            ] // .padding(10),
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        println!("subscription()");
        Subscription::run(ai_worker)
    }

    fn send_request_if_turn_is_ai(&mut self) {
        let player = self.game.current_player();
        let player_type = match player {
            reversi::Color::Black => self.black_player_type,
            reversi::Color::White => self.white_player_type,
        };
        if let Some(t) = player_type {
            if t == PlayerType::Ai {
                if let Some(mut sender) = self.sender_to_ai_worker.take() {
                    let req = AiMoveRequest {
                        id: self.next_request_ai_move_id,
                        board: self.game.board().board_state(),
                        player: self.game.current_player(),
                    };
                    let _ = sender.try_send(Message::AiMove(req));
                    self.waiting_requests.push(req);
                    self.next_request_ai_move_id += 1;
                    if self.next_request_ai_move_id < 0 {
                        self.next_request_ai_move_id = 0;
                    }
                    self.sender_to_ai_worker = Some(sender);
                }
            }
        };
    }
}

fn ai_worker() -> impl Stream<Item = Message> {
    println!("ai_worker()");
    iced::stream::channel(100, |mut output| async move {
        use iced::futures::SinkExt;
        use iced::futures::StreamExt;

        let (sender, mut receiver_from_app) = mpsc::channel::<Message>(100);
        let _ = output.send(Message::AiWorkerAwaked(sender)).await;
        println!("[stream] ai worker awaked");

        let ai = Arc::new(Mutex::new(Ai::default()));

        loop {
            let msg = receiver_from_app.select_next_some().await;
            println!("[stream] received request");
            if let Message::AiMove(req) = msg {
                let (mut sender, mut receiver_from_thread) =
                    mpsc::channel::<Option<reversi::Position>>(100);
                let ai = ai.clone();
                let handle = thread::spawn(move || {
                    println!("[thread] begin");
                    let mut bit_board = BitBoard::new();
                    bit_board.set_board_state(&req.board);

                    // let mut searcher = Negaalpha::new(evaluate::test_evaluate);
                    // let search_result =
                    //     searcher.search(&bit_board, req.player, 8, i32::MIN + 1, i32::MAX);
                    // let pos = search_result.best_move.map(|mv| mv.position);

                    if let Ok(mut ai) = ai.lock() {
                        let pos = ai.decide_move(&bit_board, req.player);
                        let _ = sender.try_send(pos);
                    } else {
                        let _ = sender.try_send(None);
                    }
                    println!("[thread] end");
                });
                let pos_or_none = receiver_from_thread.select_next_some().await;
                let _ = handle.join();
                println!("[stream] pos: {:?}", pos_or_none);
                if let Some(pos) = pos_or_none {
                    let _ = output
                        .send(Message::MoveMaked {
                            pos,
                            request_id: req.id,
                        })
                        .await;
                }
                println!("[stream] send");
            };
        }
    })
}
