mod board;

use std::thread;

use board::BoardView;
use iced::{
    futures::channel::mpsc,
    widget::{canvas, column, row, text},
    Element, Length, Settings, Subscription, Task, Theme,
};
use reversi::{
    ai::{ai_player::AiPlayer, evaluate, player::Player},
    bit_board::BitBoard,
    game::Game,
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

struct Reversi {
    pub stones_cache: canvas::Cache,
    pub game: Game,
}

#[derive(Debug, Clone)]
enum Message {
    MoveMaked { row: usize, col: usize },
    Updated(),
}

impl Reversi {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                stones_cache: canvas::Cache::default(),
                game: Game::initial(),
            },
            iced::widget::focus_next(),
        )
    }

    fn update(&mut self, message: Message) {
        println!("update()");
        match message {
            Message::MoveMaked { row, col } => {
                println!("Clicked cell: row = {}, col = {}", row, col);
                if self.game.is_game_over() {
                    return;
                }

                let player = self.game.current_player();
                let _ = self.game.progress(
                    player,
                    reversi::Position {
                        x: col as i8,
                        y: row as i8,
                    },
                );
                self.stones_cache.clear();
            }
            Message::Updated() => {
                self.stones_cache.clear();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        println!("view()");
        row![
            canvas(BoardView {
                stones_cache: &self.stones_cache,
                board: self.game.board().board_state(),
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
            ],
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        println!("subscription()");
        let game = self.game.clone();
        Subscription::run_with_id(
            0,
            iced::stream::channel(100, |mut output| async move {
                println!("stream function");
                use iced::futures::SinkExt;
                use iced::futures::StreamExt;

                let (mut sender, mut receiver) = mpsc::channel::<reversi::Position>(100);
                thread::spawn(move || {
                    let mut ai_player =
                        AiPlayer::new(evaluate::mobility_evaluate, game.current_player());
                    let pos = ai_player
                        .get_move(&BitBoard::from_board(game.board()), game.current_player());
                    let _ = sender.try_send(pos.unwrap());
                });

                // Read next input sent from `Application`
                let pos = receiver.select_next_some().await;
                let _ = output
                    .send(Message::MoveMaked {
                        row: pos.y as usize,
                        col: pos.x as usize,
                    })
                    .await;
            }),
        )
    }
}
