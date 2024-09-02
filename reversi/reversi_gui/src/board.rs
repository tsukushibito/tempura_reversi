use iced::event::Status;
use iced::widget::canvas::{Cache, Frame, Geometry, Path, Program, Stroke, Text};
use iced::{mouse, Color, Point, Rectangle, Size};
use reversi::CellState;

use crate::Message;

const BOARD_SIZE: usize = 8;
const MARGIN: f32 = 40.0;
const LABEL_SIZE: f32 = 20.0;
const CELL_STROKE_WIDTH: f32 = 2.0;
const STONE_RADIUS_FACTOR: f32 = 1.0 / 3.0;

pub struct BoardView<'a> {
    pub board: reversi::BoardState,
    pub stones_cache: &'a Cache,
    pub is_clickable: bool,
}

#[derive(Default)]
pub struct BoardViewState {
    board_cache: Cache,
}

impl<'a> Program<Message> for BoardView<'a> {
    type State = BoardViewState;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let layout = Layout::calculate(bounds);

        let background_geometry = state.board_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_board_background(frame, &layout);
            self.draw_grid(frame, &layout);
            self.draw_labels(frame, &layout);
        });

        let stones_geometry = self.stones_cache.draw(renderer, bounds.size(), |frame| {
            self.draw_stones(frame, &layout);
        });

        vec![background_geometry, stones_geometry]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: iced::widget::canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (Status, Option<Message>) {
        use iced::widget::canvas::Event;

        if !self.is_clickable {
            return (Status::Ignored, None);
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_position) = cursor.position_in(bounds) {
                    let layout = Layout::calculate(bounds);
                    if let Some((row, col)) = self.get_cell_from_position(cursor_position, &layout)
                    {
                        return (
                            Status::Captured,
                            Some(Message::MoveMaked {
                                pos: reversi::Position {
                                    x: col as u8,
                                    y: row as u8,
                                },
                                request_id: -1,
                            }),
                        );
                    }
                }

                (Status::Ignored, None)
            }
            _ => (Status::Ignored, None),
        }
    }
}

struct Layout {
    board_size: f32,
    cell_size: f32,
    x_offset: f32,
    y_offset: f32,
}

impl Layout {
    fn calculate(bounds: Rectangle) -> Self {
        let board_size = bounds.width.min(bounds.height) - MARGIN;
        let cell_size = board_size / BOARD_SIZE as f32;
        let x_offset = (bounds.width - board_size) / 2.0 + MARGIN / 2.0;
        let y_offset = (bounds.height - board_size) / 2.0 + MARGIN / 2.0;
        Self {
            board_size,
            cell_size,
            x_offset,
            y_offset,
        }
    }
}

impl<'a> BoardView<'a> {
    fn draw_board_background(&self, frame: &mut Frame, layout: &Layout) {
        let background = Path::rectangle(
            Point::new(layout.x_offset, layout.y_offset),
            Size::new(layout.board_size, layout.board_size),
        );
        frame.fill(&background, Color::from_rgb(0.0, 0.5, 0.0));
    }

    fn draw_grid(&self, frame: &mut Frame, layout: &Layout) {
        for i in 0..=BOARD_SIZE {
            // 縦線
            let x = layout.x_offset + i as f32 * layout.cell_size;
            let start = Point::new(x, layout.y_offset);
            let end = Point::new(x, layout.y_offset + layout.board_size);
            frame.stroke(
                &Path::line(start, end),
                Stroke::default()
                    .with_color(Color::BLACK)
                    .with_width(CELL_STROKE_WIDTH),
            );

            let y = layout.y_offset + i as f32 * layout.cell_size;
            let start = Point::new(layout.x_offset, y);
            let end = Point::new(layout.x_offset + layout.board_size, y);
            frame.stroke(
                &Path::line(start, end),
                Stroke::default()
                    .with_color(Color::BLACK)
                    .with_width(CELL_STROKE_WIDTH),
            );
        }
    }

    fn draw_labels(&self, frame: &mut Frame, layout: &Layout) {
        for i in 0..BOARD_SIZE {
            let label = Text {
                content: format!("{}", i + 1),
                position: Point::new(
                    layout.x_offset - LABEL_SIZE,
                    layout.y_offset + i as f32 * layout.cell_size + layout.cell_size / 2.0
                        - LABEL_SIZE / 2.0,
                ),
                color: Color::WHITE,
                size: iced::Pixels(LABEL_SIZE),
                ..Text::default()
            };
            frame.fill_text(label);
        }

        for i in 0..BOARD_SIZE {
            let label = Text {
                content: format!("{}", (b'A' + i as u8) as char),
                position: Point::new(
                    layout.x_offset + i as f32 * layout.cell_size + layout.cell_size / 2.0
                        - LABEL_SIZE / 2.0,
                    layout.y_offset - LABEL_SIZE * 1.25,
                ),
                color: Color::WHITE,
                size: iced::Pixels(LABEL_SIZE),
                ..Text::default()
            };
            frame.fill_text(label);
        }
    }

    fn draw_stones(&self, frame: &mut Frame, layout: &Layout) {
        for (i, cell) in self.board.cells.iter().enumerate() {
            let color = match cell {
                CellState::Disc(reversi::Color::Black) => Color::BLACK,
                CellState::Disc(reversi::Color::White) => Color::WHITE,
                CellState::Empty => continue,
            };
            let col = i % BOARD_SIZE;
            let row = i / BOARD_SIZE;
            let x = layout.x_offset + col as f32 * layout.cell_size + layout.cell_size / 2.0;
            let y = layout.y_offset + row as f32 * layout.cell_size + layout.cell_size / 2.0;
            let radius = layout.cell_size * STONE_RADIUS_FACTOR;
            let stone = Path::circle(Point::new(x, y), radius);
            frame.fill(&stone, color);
        }
    }

    fn get_cell_from_position(&self, position: Point, layout: &Layout) -> Option<(usize, usize)> {
        let relative_x = position.x - layout.x_offset;
        let relative_y = position.y - layout.y_offset;

        if relative_x >= 0.0
            && relative_x < layout.board_size
            && relative_y >= 0.0
            && relative_y < layout.board_size
        {
            let col = (relative_x / layout.cell_size).floor() as usize;
            let row = (relative_y / layout.cell_size).floor() as usize;
            if row < BOARD_SIZE && col < BOARD_SIZE {
                return Some((row, col));
            }
        }

        None
    }
}
