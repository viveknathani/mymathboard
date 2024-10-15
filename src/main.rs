use iced::Size;
use iced::Theme;
use mymathboard::app::ui::MyMathBoardApp;

fn main() -> iced::Result {
    iced::application("mymathboard", MyMathBoardApp::update, MyMathBoardApp::view)
        .theme(|_| Theme::Light)
        .window_size(Size {
            width: 1000.0,
            height: 1000.0,
        })
        .run()
}
