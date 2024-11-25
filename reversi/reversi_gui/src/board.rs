use iced::widget::canvas::{Cache, Frame, Geometry, Path, Program};
use iced::{mouse, Color, Point, Rectangle};

use crate::Message;

#[derive(Default)]
pub struct BoardProgram {
    board_data: [[Option<bool>; 8]; 8],
}

pub struct BoardProgramState {
    cache: Cache, // 背景とグリッド線をキャッシュ
}

impl Default for BoardProgramState {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
        }
    }
}

impl Program<Message> for BoardProgram {
    type State = BoardProgramState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let background = state.cache.draw(renderer, bounds.size(), |frame| {
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

        let mut frame = Frame::new(renderer, bounds.size());
        let cell_size = bounds.width.min(bounds.height) / 8.0;

        // 石の描画
        for row in 0..8 {
            for col in 0..8 {
                if let Some(is_black) = self.board_data[row][col] {
                    let x = col as f32 * cell_size;
                    let y = row as f32 * cell_size;

                    let color = if is_black { Color::BLACK } else { Color::WHITE };
                    frame.fill(
                        &Path::circle(
                            Point::new(x + cell_size / 2.0, y + cell_size / 2.0),
                            cell_size / 3.0,
                        ),
                        color,
                    );
                }
            }
        }

        vec![background, frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: iced::widget::canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (iced::widget::canvas::event::Status, Option<Message>) {
        if let iced::widget::canvas::Event::Mouse(mouse::Event::ButtonPressed(
            mouse::Button::Left,
        )) = event
        {
            if let Some(cursor_position) = cursor.position_in(bounds) {
                let cell_size = bounds.width.min(bounds.height) / 8.0;
                let col = (cursor_position.x / cell_size).floor() as usize;
                let row = (cursor_position.y / cell_size).floor() as usize;

                if col < 8 && row < 8 {
                    return (
                        iced::widget::canvas::event::Status::Captured,
                        Some(Message::CellClicked { row, col }),
                    );
                }
            }
        }
        (iced::widget::canvas::event::Status::Ignored, None)
    }
}
