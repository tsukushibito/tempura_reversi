mod board;

use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use board::BoardView;
use iced::{
    stream::try_channel,
    widget::{canvas, column, row, text},
    Element, Length, Settings, Subscription, Task, Theme,
};
use reversi::{
    ai::{ai_player::AiPlayer, human_player::HumanPlayer, player::Player},
    game::{Game, GameCommand, GameEvent},
    Color,
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
    pub event_receiver: Arc<Mutex<Receiver<GameEvent>>>,
    pub command_sender: Sender<GameCommand>,
}

#[derive(Debug, Clone)]
enum Message {
    CellClicked { row: usize, col: usize },
    GameEvent(GameEvent),
}

impl Reversi {
    fn new() -> (Self, Task<Message>) {
        let (event_sender, event_receiver) = mpsc::channel();
        let (command_sender, command_receiver) = mpsc::channel();

        let black_player = Box::new(HumanPlayer) as Box<dyn Player + Send>;
        let white_player = Box::new(AiPlayer::new(
            reversi::ai::evaluate::simple_evaluate,
            Color::White,
        ));

        let mut game = Game::new(event_sender, command_receiver);

        // 別スレッドでゲームを実行
        thread::spawn(move || {
            let _ = game.run();
        });

        let event_receiver = Arc::new(Mutex::new(event_receiver));

        (
            Self {
                stones_cache: canvas::Cache::default(),
                event_receiver,
                command_sender,
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

struct GameSubscription {
    event_receiver: Arc<Mutex<Receiver<GameEvent>>>,
}

impl GameSubscription {
    fn new(event_receiver: Arc<Mutex<Receiver<GameEvent>>>) -> Subscription<Message> {
        //Subscription::from_recipe(GameSubscription { event_receiver })
        try_channel(size, f)
    }
}

impl Recipe<Output = Message> for GameSubscription {}
