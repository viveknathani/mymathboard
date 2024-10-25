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
use iced::Pixels;
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
    pub width: f32,
    pub height: f32,
    pub is_dragging: bool,
    pub viewport_offset: Vector,
    pub last_cursor_position: Option<Point>,
    pub equations: Vec<Node>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph {
            width: 0.0,
            height: 0.0,
            cell_size: DEFAULT_CELL_SIZE,
            is_dragging: false,
            viewport_offset: Vector::new(0.0, 0.0),
            last_cursor_position: None,
            equations: Vec::new(),
        }
    }
}

impl Graph {
    // Convert graph coordinates (x, y) to screen coordinates (screen_x, screen_y)
    pub fn graph_to_screen(
        &self,
        x: f32,
        y: f32,
        offset_x: f32,
        offset_y: f32,
        width: f32,
        height: f32,
        cell_size: f32,
    ) -> (f32, f32) {
        let screen_x = (x - offset_x) * cell_size + width / 2.0;
        let screen_y = height / 2.0 - (y - offset_y) * cell_size; // Flip the y-axis
        (screen_x, screen_y)
    }

    // Convert screen coordinates (screen_x, screen_y) to graph coordinates (x, y)
    pub fn screen_to_graph(
        &self,
        screen_x: f32,
        screen_y: f32,
        offset_x: f32,
        offset_y: f32,
        width: f32,
        height: f32,
    ) -> (f32, f32) {
        let x = (screen_x - width / 2.0) + offset_x;
        let y = offset_y - (screen_y - height / 2.0); // Flip the y-axis
        (x, y)
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
        let screen_start_x = (self.viewport_offset.x / self.cell_size as f32).floor() as isize;
        let screen_start_y = (self.viewport_offset.y / self.cell_size as f32).floor() as isize;
        let screen_end_x = ((self.viewport_offset.x + bounds.width as f32) / self.cell_size as f32)
            .ceil() as isize;
        let screen_end_y = ((self.viewport_offset.y + bounds.height as f32) / self.cell_size as f32)
            .ceil() as isize;

        // Draw cells for the visible area.
        for y in screen_start_y..screen_end_y {
            for x in screen_start_x..screen_end_x {
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

        let (screen_center_x, screen_center_y) = self.graph_to_screen(
            0.0,
            0.0,
            self.viewport_offset.x,
            self.viewport_offset.y,
            bounds.width,
            bounds.height,
            cell_size.height,
        );

        // Draw the y-axis
        let y_axis = canvas::Path::line(
            Point::new(screen_center_x, 0.0),
            Point::new(screen_center_x, bounds.height),
        );
        frame.stroke(
            &y_axis,
            Stroke::default()
                .with_width(GRAPH_THIN_LINE_WIDTH)
                .with_color(Color::from_rgb(0.8, 0.8, 0.8)),
        );

        // Draw the x-axis
        let x_axis = canvas::Path::line(
            Point::new(0.0, screen_center_y),
            Point::new(bounds.width, screen_center_y),
        );
        frame.stroke(
            &x_axis,
            Stroke::default()
                .with_width(GRAPH_THICK_LINE_WIDTH)
                .with_color(Color::from_rgb(0.8, 0.8, 0.8)),
        );

        for x in screen_start_x..screen_end_x {
            let screen_x = (x as f32 * self.cell_size as f32) - self.viewport_offset.x;
            let (graph_x, _) = self.screen_to_graph(
                screen_x,
                0.0,
                self.viewport_offset.x,
                self.viewport_offset.y,
                bounds.width,
                bounds.height,
            );

            if screen_x >= 0.0 && screen_x <= bounds.width {
                frame.fill_text(Text {
                    content: format!("{}", graph_x),
                    position: Point::new(screen_x, screen_center_y + 5.0),
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    size: Pixels(12.0),
                    ..Text::default()
                });
            }
        }

        for y in screen_start_y..screen_end_y {
            let screen_y = (y as f32 * self.cell_size as f32) - self.viewport_offset.y;
            let (_, graph_y) = self.screen_to_graph(
                0.0,
                screen_y,
                self.viewport_offset.x,
                self.viewport_offset.y,
                bounds.width,
                bounds.height,
            );

            if screen_y >= 0.0 && screen_y <= bounds.height {
                frame.fill_text(Text {
                    content: format!("{}", graph_y),
                    position: Point::new(screen_center_x + 5.0, screen_y),
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    size: Pixels(12.0),
                    ..Text::default()
                });
            }
        }

        // Render equations in the visible area.
        for equation in &self.equations {
            let start_x = self
                .screen_to_graph(
                    0.0,
                    0.0,
                    self.viewport_offset.x,
                    self.viewport_offset.y,
                    bounds.width,
                    bounds.height,
                )
                .0;
            let end_x = self
                .screen_to_graph(
                    bounds.width,
                    0.0,
                    self.viewport_offset.x,
                    self.viewport_offset.y,
                    bounds.width,
                    bounds.height,
                )
                .0;

            let path = canvas::Path::new(|builder: &mut canvas::path::Builder| {
                let mut x = start_x;
                while x < end_x {
                    let calc =
                        equation.eval_with_context(&context_map! { "x" => x as f64 }.unwrap());
                    if let Ok(result) = calc {
                        let y = result.as_float().unwrap() as f32;
                        let (screen_x, screen_y) = self.graph_to_screen(
                            x,
                            y,
                            self.viewport_offset.x,
                            self.viewport_offset.y,
                            bounds.width,
                            bounds.height,
                            cell_size.height,
                        );

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
                    .with_color(Color::from_rgb(0.0, 255.0, 0.0)),
            );
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut (),
        event: Event,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> (event::Status, Option<MyMathBoardMessage>) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(last_position) = _cursor.position() {
                    return (
                        event::Status::Captured,
                        Some(MyMathBoardMessage::StartDrag(
                            last_position,
                            bounds.width,
                            bounds.height,
                        )),
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
