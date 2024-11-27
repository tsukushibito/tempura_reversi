mod board;

use board::Board;
use iced::{
    widget::{canvas, column, row, text},
    Element, Length, Settings, Theme,
};

#[derive(Default)]
struct State {
    pub stones_cache: canvas::Cache,
}

#[derive(Debug)]
enum Message {
    CellClicked { row: usize, col: usize },
}

pub fn main() -> iced::Result {
    iced::application("Tempura Reversi", update, view)
        .theme(theme)
        .settings(Settings {
            antialiasing: true,
            ..Default::default()
        })
        .run()
}

fn update(_state: &mut State, message: Message) {
    match message {
        Message::CellClicked { row, col } => {
            println!("Clicked cell: row = {}, col = {}", row, col);
            // ここにゲームロジックを追加
        }
    }
}

fn view(state: &State) -> Element<Message> {
    row![
        canvas(Board {
            stones_cache: &state.stones_cache,
            board_data: Default::default(),
        })
        .width(Length::FillPortion(2))
        .height(Length::Fill),
        column![text!("Info Area").width(Length::FillPortion(1)),],
    ]
    .into()
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}
