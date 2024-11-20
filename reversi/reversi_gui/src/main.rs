use iced::widget::canvas::{Cache, Frame, Geometry, Path, Program};
use iced::widget::{canvas, column, row, text};
use iced::{window, Color, Element, Length, Size, Subscription, Task, Theme};

#[derive(Default)]
struct State {
    // board: Board,
}

#[derive(Debug)]
enum Message {
    WindowResized(Size),
}

pub fn main() -> iced::Result {
    iced::application("Tempura Reversi", update, view)
        .subscription(subscription)
        .theme(theme)
        .run()
}

fn subscription(_state: &State) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| Message::WindowResized(size))
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::WindowResized(size) => println!("size: {}, {}", size.width, size.height),
    }
    Task::none()
}

fn view(state: &State) -> Element<Message> {
    row![
        canvas(Board {
            cache: Cache::new(),
        })
        .width(Length::FillPortion(2))
        .height(Length::Fill),
        text!("Inspector Area").width(Length::FillPortion(1)),
    ]
    .into()
    // row![Canvas::new(&state.board)
    //     .width(Length::FillPortion(1))
    //     .height(Length::Fill),]
    // .into()
}

fn theme(state: &State) -> Theme {
    Theme::Dark
}

#[derive(Default)]
struct Board {
    cache: Cache,
}

impl<Message> Program<Message> for Board {
    type State = State;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self
            .cache
            .draw(renderer, bounds.size(), |frame: &mut Frame| {
                const BOARD_SIZE: usize = 8; // リバーシのボードは8x8のグリッド
                let margin = 40.0; // 数字とアルファベットを表示するためのマージン
                let board_size = bounds.width.min(bounds.height) - margin; // ボードのサイズは幅と高さの小さい方に合わせる（マージンを除く）
                let cell_size = board_size / BOARD_SIZE as f32; // 各セルのサイズを計算

                // ボード全体を中央に配置するためのオフセットを計算（マージンを考慮）
                let x_offset = (bounds.width - board_size) / 2.0 + margin / 2.0;
                let y_offset = (bounds.height - board_size) / 2.0 + margin / 2.0;

                // ボードの背景を描画（緑色）
                let board_background =
                    Path::rectangle([x_offset, y_offset].into(), [board_size, board_size].into());
                frame.fill(&board_background, Color::from_rgb(0.0, 0.5, 0.0)); // リバーシのボードの緑色

                // 8x8のセルを描画
                for row in 0..BOARD_SIZE {
                    for col in 0..BOARD_SIZE {
                        // 各セルの左上の座標を計算
                        let x = x_offset + col as f32 * cell_size;
                        let y = y_offset + row as f32 * cell_size;

                        // セルの描画（枠線）
                        let cell = Path::rectangle([x, y].into(), [cell_size, cell_size].into());
                        frame.stroke(
                            &cell,
                            iced::widget::canvas::Stroke::default()
                                .with_color(Color::BLACK)
                                .with_width(2.0),
                        ); // 枠線は黒色
                    }
                }

                const LABEL_SIZE: f32 = 20.0;
                // 数字（1-8）を左側に描画
                for i in 0..BOARD_SIZE {
                    let text = iced::widget::canvas::Text {
                        content: format!("{}", i + 1),
                        position: iced::Point::new(
                            x_offset - LABEL_SIZE,
                            y_offset + i as f32 * cell_size + cell_size / 2.0 - LABEL_SIZE / 2.0,
                        ),
                        color: Color::WHITE,
                        size: iced::Pixels(LABEL_SIZE),
                        ..iced::widget::canvas::Text::default()
                    };
                    frame.fill_text(text);
                }

                // アルファベット（A-H）を上側に描画
                for i in 0..BOARD_SIZE {
                    let text = iced::widget::canvas::Text {
                        content: format!("{}", (b'A' + i as u8) as char),
                        position: iced::Point::new(
                            x_offset + i as f32 * cell_size + cell_size / 2.0 - LABEL_SIZE / 2.0,
                            y_offset - LABEL_SIZE * 1.25,
                        ),
                        color: Color::WHITE,
                        size: iced::Pixels(LABEL_SIZE),
                        ..iced::widget::canvas::Text::default()
                    };
                    frame.fill_text(text);
                }
            });

        vec![geometry]
    }
}
