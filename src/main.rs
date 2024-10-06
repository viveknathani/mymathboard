use iced::widget::button;
use iced::widget::column;
use iced::widget::text;
use iced::widget::Column;
use iced::Center;
use iced::Theme;

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}

fn main() -> iced::Result {
    iced::application("mymathboard", Counter::update, Counter::view)
        .theme(|_| Theme::Light)
        .run()
}
