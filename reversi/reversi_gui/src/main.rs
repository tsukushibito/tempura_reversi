mod board;

use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use board::BoardView;
use iced::{
    widget::{canvas, column, row, text},
    Element, Length, Settings, Subscription, Task, Theme,
};
use reversi::game::GameEvent;

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
}

#[derive(Debug, Clone)]
enum Message {
    CellClicked { row: usize, col: usize },
    GameEvent(GameEvent),
}

impl Reversi {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                stones_cache: canvas::Cache::default(),
            },
            iced::widget::focus_next(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CellClicked { row, col } => {
                println!("Clicked cell: row = {}, col = {}", row, col);
            }
            Message::GameEvent(game_event) => todo!(),
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

    fn subscription(&self) -> Subscription<Message> {
        todo!()
    }
}
