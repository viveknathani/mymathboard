use super::constants::DEFAULT_CELL_SIZE;
use super::constants::GRAPH_THICK_LINE_WIDTH;
use super::constants::GRAPH_THIN_LINE_WIDTH;
use super::types::MyMathBoardMessage;
use evalexpr::context_map;
use evalexpr::Node;
use iced::event;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::Event;
use iced::widget::canvas::Stroke;
use iced::widget::canvas::Text;
use iced::Color;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Size;
use iced::Theme;
use iced::Vector;

/// The fundamental data structure used to draw a 2D graph on the screen.
#[derive(Debug, Clone)]
pub struct Graph {
    pub cell_size: u64,
    pub is_dragging: bool,
    pub viewport_offset: Vector,
    pub last_cursor_position: Option<Point>,
    pub equations: Vec<Node>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            cell_size: DEFAULT_CELL_SIZE,
            is_dragging: false,
            viewport_offset: Vector::new(0.0, 0.0),
            last_cursor_position: None,
            equations: Vec::new(),
        }
    }
}

impl canvas::Program<MyMathBoardMessage> for Graph {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // Get a new frame.
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Prepare the cell. Our cell is a square.
        let cell_size = Size {
            width: self.cell_size as f32,
            height: self.cell_size as f32,
        };

        // Calculate the visible area.
        let start_x = (self.viewport_offset.x / self.cell_size as f32).floor() as isize;
        let start_y = (self.viewport_offset.y / self.cell_size as f32).floor() as isize;
        let end_x = ((self.viewport_offset.x + bounds.width as f32) / self.cell_size as f32).ceil()
            as isize;
        let end_y = ((self.viewport_offset.y + bounds.height as f32) / self.cell_size as f32).ceil()
            as isize;

        // Draw cells for the visible area.
        for y in start_y..end_y {
            for x in start_x..end_x {
                let cell = canvas::Path::rectangle(
                    Point::new(
                        (x * self.cell_size as isize) as f32 - self.viewport_offset.x,
                        (y * self.cell_size as isize) as f32 - self.viewport_offset.y,
                    ),
                    cell_size,
                );
                frame.stroke(
                    &cell,
                    Stroke::default()
                        .with_width(GRAPH_THIN_LINE_WIDTH)
                        .with_color(Color::from_rgb8(50, 50, 50)),
                );
            }
        }

        // Render equations in the visible area.
        for equation in &self.equations {
            let start_x = (self.viewport_offset.x / self.cell_size as f32).floor() as f32;
            let end_x = ((self.viewport_offset.x + bounds.width as f32) / self.cell_size as f32)
                .ceil() as f32;

            let path = canvas::Path::new(|builder: &mut canvas::path::Builder| {
                let mut x = start_x;
                while x < end_x {
                    let calc =
                        equation.eval_with_context(&context_map! { "x" => x as f64 }.unwrap());
                    if calc.is_ok() {
                        let y = calc.unwrap().as_float().unwrap();
                        let screen_x = x as f32 * self.cell_size as f32 - self.viewport_offset.x;
                        let screen_y = y as f32 * self.cell_size as f32 - self.viewport_offset.y;

                        if (x - start_x).abs() < f32::EPSILON {
                            builder.move_to(Point::new(screen_x, screen_y));
                        } else {
                            builder.line_to(Point::new(screen_x, screen_y));
                        }
                    }
                    x += 0.01;
                }
            });
            frame.stroke(
                &path,
                Stroke::default()
                    .with_width(GRAPH_THIN_LINE_WIDTH)
                    .with_color(Color::from_rgb(255.0, 0.0, 0.0)),
            );
        }

        let middle_x = -self.viewport_offset.x;
        let middle_y = -self.viewport_offset.y;

        let font_size = iced::Pixels(14.0);

        for x in start_x..end_x {
            let screen_x = (x * self.cell_size as isize) as f32 - self.viewport_offset.x;
            let number = (x * self.cell_size as isize).to_string();

            if screen_x >= 0.0 && screen_x <= bounds.width {
                frame.fill_text(Text {
                    content: number.clone(),
                    position: Point::new(screen_x, middle_y + font_size.0),
                    color: Color::WHITE,
                    size: font_size,
                    font: iced::Font::default(),
                    ..Default::default()
                });
            }
        }

        for y in start_y..end_y {
            let screen_y = (y * self.cell_size as isize) as f32 - self.viewport_offset.y;
            let number = (y * self.cell_size as isize).to_string();

            if screen_y >= 0.0 && screen_y <= bounds.height {
                frame.fill_text(Text {
                    content: number.clone(),
                    position: Point::new(middle_x + font_size.0, screen_y),
                    color: Color::WHITE,
                    size: font_size,
                    font: iced::Font::default(),
                    ..Default::default()
                });
            }
        }

        let zero_x = -self.viewport_offset.x;
        let zero_y = -self.viewport_offset.y;
        let vertical_axis =
            canvas::Path::line(Point::new(zero_x, 0.0), Point::new(zero_x, bounds.height));
        let horizontal_axis =
            canvas::Path::line(Point::new(0.0, zero_y), Point::new(bounds.width, zero_y));
        frame.stroke(
            &vertical_axis,
            Stroke::default()
                .with_width(GRAPH_THICK_LINE_WIDTH)
                .with_color(Color::WHITE),
        );
        frame.stroke(
            &horizontal_axis,
            Stroke::default()
                .with_width(GRAPH_THICK_LINE_WIDTH)
                .with_color(Color::WHITE),
        );

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut (),
        event: Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> (event::Status, Option<MyMathBoardMessage>) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(last_position) = _cursor.position() {
                    return (
                        event::Status::Captured,
                        Some(MyMathBoardMessage::StartDrag(last_position)),
                    );
                }
                (event::Status::Ignored, None)
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                (event::Status::Captured, Some(MyMathBoardMessage::EndDrag))
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if self.is_dragging {
                    if let Some(last_position) = self.last_cursor_position {
                        let delta =
                            Vector::new(position.x - last_position.x, position.y - last_position.y);
                        return (
                            event::Status::Captured,
                            Some(MyMathBoardMessage::Dragged(delta)),
                        );
                    }
                }
                (event::Status::Ignored, None)
            }
            _ => (event::Status::Ignored, None),
        }
    }
}
