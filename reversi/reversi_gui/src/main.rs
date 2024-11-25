mod board;

use board::BoardProgram;
use iced::{
    widget::{canvas, column, row, text},
    Element, Length, Theme,
};

#[derive(Default)]
struct State {}

#[derive(Debug)]
enum Message {
    CellClicked { row: usize, col: usize },
}

pub fn main() -> iced::Result {
    iced::application("Tempura Reversi", update, view)
        .theme(theme)
        .run()
}

fn update(_state: &mut State, _message: Message) {}

fn view(_state: &State) -> Element<Message> {
    row![
        canvas(BoardProgram::default())
            .width(Length::FillPortion(2))
            .height(Length::Fill),
        column![text!("Info Area").width(Length::FillPortion(1)),],
    ]
    .into()
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}
