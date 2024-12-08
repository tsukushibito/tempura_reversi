mod board;

use std::thread;

use board::BoardView;
use iced::{
    futures::{channel::mpsc, Stream},
    widget::{canvas, column, pick_list, row, text},
    Element, Length, Settings, Subscription, Task, Theme,
};
use reversi::{
    ai::{ai_player::AiPlayer, evaluate, player::Player},
    bit_board::BitBoard,
    board::Board,
    game::Game,
    BoardState,
};

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
}

#[derive(Debug, Clone)]
enum Message {
    AiWorkerAwaked(mpsc::Sender<Message>),
    RequestAiMove {
        board: BoardState,
        player: reversi::Color,
    },
    MoveMaked(reversi::Position),
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
            Message::MoveMaked(pos) => {
                println!("Clicked cell: ({}, {})", pos.x, pos.y);
                if self.game.is_game_over() {
                    return;
                }

                let player = self.game.current_player();
                let _ = self.game.progress(player, pos);
                self.stones_cache.clear();
                self.send_request_if_turn_is_ai();
            }
            Message::RequestAiMove {
                board: _,
                player: _,
            } => panic!(),
            Message::BlackPlayerTypeChanged(player_type) => {
                self.black_player_type = Some(player_type);
                self.send_request_if_turn_is_ai();
            }
            Message::WhitePlayerTypeChanged(player_type) => {
                self.white_player_type = Some(player_type);
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
                    ),
                ],
                row![
                    text("White player type: "),
                    pick_list(
                        PlayerType::ALL,
                        self.white_player_type,
                        Message::WhitePlayerTypeChanged,
                    ),
                ]
            ],
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
                    let _ = sender.try_send(Message::RequestAiMove {
                        board: self.game.board().board_state(),
                        player: self.game.current_player(),
                    });
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

        loop {
            let req = receiver_from_app.select_next_some().await;
            println!("[stream] received request");
            if let Message::RequestAiMove { board, player } = req {
                let (mut sender, mut receiver_from_thread) =
                    mpsc::channel::<reversi::Position>(100);
                thread::spawn(move || {
                    println!("[thread] begin");
                    let mut ai_player = AiPlayer::new(evaluate::mobility_evaluate, player);
                    let mut bit_board = BitBoard::new();
                    bit_board.set_board_state(&board);
                    let pos = ai_player.get_move(&bit_board, player);
                    let _ = sender.try_send(pos.unwrap());
                    println!("[thread] end");
                });
                let pos = receiver_from_thread.select_next_some().await;
                println!("[stream] pos: {:?}", pos);
                let _ = output.send(Message::MoveMaked(pos)).await;
                println!("[stream] send");
            };
        }
    })
}
