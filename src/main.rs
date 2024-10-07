use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::Stroke;
use iced::Element;
use iced::Point;
use iced::Rectangle;
use iced::Renderer;
use iced::Size;
use iced::Theme;

#[derive(Debug, Clone, Copy)]
enum Message {}

pub type State = ();

#[derive(Debug, Default)]
struct Grid {
    width: u64,
    height: u64,
    cell_size: u64,
}

impl<Message> canvas::Program<Message> for Grid {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let cell_size = Size {
            width: self.cell_size as f32,
            height: self.cell_size as f32,
        };

        for y in (0..self.height).step_by(self.cell_size as usize) {
            for x in (0..self.width).step_by(self.cell_size as usize) {
                let cell = canvas::Path::rectangle(Point::new(x as f32, y as f32), cell_size);
                frame.stroke(&cell, Stroke::default().with_width(1.0));
            }
        }

        vec![frame.into_geometry()]
    }
}

#[derive(Default)]
struct MyMathBoardApp {}

impl MyMathBoardApp {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        canvas(Grid {
            height: 1000,
            width: 1000,
            cell_size: 50,
        })
        .height(1000)
        .width(1000)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("mymathboard", MyMathBoardApp::update, MyMathBoardApp::view)
        .theme(|_| Theme::Light)
        .run()
}
