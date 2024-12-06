mod board;

use board::BoardView;
use iced::{
    widget::{canvas, column, row, text},
    Element, Length, Settings, Subscription, Task, Theme,
};
use reversi::game::Game;

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
    pub game: Game,
}

#[derive(Debug, Clone)]
enum Message {
    CellClicked { row: usize, col: usize },
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
        match message {
            Message::CellClicked { row, col } => {
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
            Message::Updated() => todo!(),
        }
    }

    fn view(&self) -> Element<Message> {
        row![
            canvas(BoardView {
                stones_cache: &self.stones_cache,
                board: self.game.board().board_state(),
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
