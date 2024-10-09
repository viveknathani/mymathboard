use iced::border::Radius;
use iced::event;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::Event;
use iced::widget::canvas::Stroke;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Row;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Size;
use iced::Theme;
use iced::Vector;

#[derive(Debug, Clone, Copy)]
enum MyMathBoardMessage {
    Dragged(Vector),
    StartDrag(Point),
    EndDrag,
    ZoomIn,
    ZoomOut,
}

pub type State = ();

#[derive(Debug, Copy, Clone)]
struct Grid {
    cell_size: u64,
    viewport_offset: Vector,
    last_cursor_position: Option<Point>,
    is_dragging: bool,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            is_dragging: false,
            cell_size: 100,
            viewport_offset: Vector::new(0.0, 0.0),
            last_cursor_position: None,
        }
    }
}

impl canvas::Program<MyMathBoardMessage> for Grid {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let start_x = (self.viewport_offset.x / self.cell_size as f32).floor() as isize;
        let start_y = (self.viewport_offset.y / self.cell_size as f32).floor() as isize;
        let end_x = ((self.viewport_offset.x + bounds.width as f32) / self.cell_size as f32).ceil()
            as isize;
        let end_y = ((self.viewport_offset.y + bounds.height as f32) / self.cell_size as f32).ceil()
            as isize;

        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let cell_size = Size {
            width: self.cell_size as f32,
            height: self.cell_size as f32,
        };

        for y in start_y..end_y {
            for x in start_x..end_x {
                let cell = canvas::Path::rectangle(
                    Point::new(
                        (x * self.cell_size as isize) as f32 - self.viewport_offset.x,
                        (y * self.cell_size as isize) as f32 - self.viewport_offset.y,
                    ),
                    cell_size,
                );
                frame.stroke(&cell, Stroke::default().with_width(1.0));
            }
        }

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
                return (event::Status::Captured, Some(MyMathBoardMessage::EndDrag));
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

fn button_style() -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(Color::BLACK.into()),
        text_color: Color::WHITE,
        border: Border {
            radius: Radius {
                top_left: 5.0,
                top_right: 5.0,
                bottom_left: 5.0,
                bottom_right: 5.0,
            },
            ..Default::default()
        },
        ..iced::widget::button::Style::default()
    }
}

#[derive(Default)]
struct MyMathBoardApp {
    grid: Grid,
}

impl MyMathBoardApp {
    fn update(&mut self, message: MyMathBoardMessage) {
        match message {
            MyMathBoardMessage::Dragged(delta) => {
                self.grid.viewport_offset = self.grid.viewport_offset + delta;
                self.grid.last_cursor_position = Some(Point::new(
                    self.grid.last_cursor_position.unwrap().x + delta.x,
                    self.grid.last_cursor_position.unwrap().y + delta.y,
                ));
            }
            MyMathBoardMessage::StartDrag(position) => {
                self.grid.is_dragging = true;
                self.grid.last_cursor_position = Some(position);
            }
            MyMathBoardMessage::EndDrag => {
                self.grid.is_dragging = false;
                self.grid.last_cursor_position = None;
            }
            MyMathBoardMessage::ZoomIn => {
                self.grid.cell_size = (self.grid.cell_size as f32 * 1.1).ceil() as u64;
            }
            MyMathBoardMessage::ZoomOut => {
                self.grid.cell_size = (self.grid.cell_size as f32 * 0.9).ceil() as u64;
            }
        }
    }

    fn view(&self) -> Element<MyMathBoardMessage> {
        let zoom_in_button = Button::new("+")
            .on_press(MyMathBoardMessage::ZoomIn)
            .width(50)
            .height(50)
            .style(|_theme, _| button_style());

        let zoom_out_button = Button::new("-")
            .on_press(MyMathBoardMessage::ZoomOut)
            .width(50)
            .height(50)
            .style(|_theme, _| button_style());

        let canvas = canvas(self.grid).width(1000.0).height(1000.0);

        let controls = Row::new()
            .spacing(10)
            .push(zoom_in_button)
            .push(zoom_out_button)
            .padding(10);

        Column::new()
            .push(
                Container::new(canvas)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(
                Container::new(controls)
                    .width(Length::Shrink)
                    .height(Length::Shrink),
            )
            .into()
    }
}

fn main() -> iced::Result {
    iced::application("mymathboard", MyMathBoardApp::update, MyMathBoardApp::view)
        .theme(|_| Theme::Light)
        .window_size(Size {
            width: 1000.0,
            height: 1000.0,
        })
        .run()
}
