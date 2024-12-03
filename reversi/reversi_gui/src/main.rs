mod board;

use std::{
    sync::mpsc::{self, Receiver},
    thread,
};

use board::BoardView;
use iced::{
    widget::{canvas, column, row, text},
    Element, Length, Settings, Subscription, Task, Theme,
};
use reversi::{
    ai::{ai_player::AiPlayer, human_player::HumanPlayer, player::Player},
    bit_board::BitBoard,
    board::Board,
    game::{Game, GameEvent, GameState},
    Color,
};

pub fn main() -> iced::Result {
    iced::application("Tempura Reversi", Reversi::update, Reversi::view)
        .theme(Reversi::theme)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .run_with(Reversi::new)
}

struct Reversi {
    pub stones_cache: canvas::Cache,
    // pub event_receiver: Receiver<GameEvent<BitBoard>>,
    // pub last_event: Option<GameEvent<BitBoard>>,
}

#[derive(Debug, Clone)]
enum Message {
    CellClicked { row: usize, col: usize },
}

impl Reversi {
    fn new() -> (Self, Task<Message>) {
        let (event_sender, event_receiver) = mpsc::channel();

        let black_player = Box::new(HumanPlayer) as Box<dyn Player + Send>;
        let white_player = Box::new(AiPlayer::new(
            reversi::ai::evaluate::simple_evaluate,
            Color::White,
        ));

        let game = Game::new(black_player, white_player, event_sender.clone());

        // 別スレッドでゲームを実行
        thread::spawn(move || {
            game.play();
        });

        (
            Self {
                stones_cache: canvas::Cache::default(),
                // event_receiver,
                // last_event: None,
            },
            iced::widget::focus_next(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CellClicked { row, col } => {
                println!("Clicked cell: row = {}, col = {}", row, col);
                // ここにゲームロジックを追加
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            canvas(BoardView {
                stones_cache: &self.stones_cache,
                board_data: Default::default(),
            })
            .width(Length::FillPortion(2))
            .height(Length::Fill),
            column![text!("Info Area").width(Length::FillPortion(1)),],
        ]
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
