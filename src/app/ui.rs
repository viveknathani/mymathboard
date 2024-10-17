use super::graph::Graph;
use super::types::MyMathBoardMessage;
use crate::repl::Repl;
use evalexpr::build_operator_tree;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::text_input;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Row;
use iced::widget::Scrollable;
use iced::widget::Space;
use iced::widget::Text;
use iced::widget::TextInput;
use iced::Background;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Point;
use iced::Task;

#[derive(Default)]
pub struct MyMathBoardApp {
    repl: Repl,
    graph: Graph,
    input: String,
    output_history: Vec<String>,
    focus_input: bool,
}

impl MyMathBoardApp {
    pub fn update(&mut self, message: MyMathBoardMessage) -> Task<MyMathBoardMessage> {
        match message {
            MyMathBoardMessage::Dragged(delta) => {
                self.graph.viewport_offset = self.graph.viewport_offset + delta;
                self.graph.last_cursor_position = Some(Point::new(
                    self.graph.last_cursor_position.unwrap().x + delta.x,
                    self.graph.last_cursor_position.unwrap().y + delta.y,
                ));
                Task::none()
            }
            MyMathBoardMessage::StartDrag(position) => {
                self.graph.is_dragging = true;
                self.graph.last_cursor_position = Some(position);
                Task::none()
            }
            MyMathBoardMessage::EndDrag => {
                self.graph.is_dragging = false;
                self.graph.last_cursor_position = None;
                Task::none()
            }
            MyMathBoardMessage::ZoomIn => {
                self.graph.cell_size = (self.graph.cell_size as f32 * 1.1).ceil() as u64;
                Task::none()
            }
            MyMathBoardMessage::ZoomOut => {
                self.graph.cell_size = (self.graph.cell_size as f32 * 0.9).ceil() as u64;
                Task::none()
            }
            MyMathBoardMessage::DrawEquation(equation) => {
                let node_formation = build_operator_tree(&equation);
                if node_formation.is_ok() {
                    self.graph.equations.push(node_formation.unwrap());
                }
                Task::none()
            }
            MyMathBoardMessage::InputChanged(new_input) => {
                self.input = new_input;
                Task::none()
            }
            MyMathBoardMessage::InputSubmitted => {
                let result = self.repl.process_input(&self.input);
                self.output_history
                    .push(format!(">>> {}\n=> {:?}", self.input, result));
                self.input.clear();
                self.focus_input = true;
                text_input::focus("1")
            }
        }
    }

    pub fn view(&self) -> Element<MyMathBoardMessage> {
        let control_bar = Container::new(Text::new("vivek's board").color(Color::WHITE))
            .height(Length::FillPortion(3))
            .width(Length::Fill);

        let horizontal_divider = Container::new(Space::with_height(Length::Fixed(1.0)))
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            });

        let graphing_pane = Container::new(Text::new("graph").color(Color::WHITE))
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .center_x(Length::Fill);

        let vertical_divider = Container::new(Space::with_width(Length::Fixed(1.0)))
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            });

        let mut repl_output = self.output_history.iter().fold(
            Column::new().spacing(5).width(Length::Fill),
            |column, entry| {
                column.push(
                    Container::new(Text::new(entry).color(Color::WHITE).size(16))
                        .width(Length::Fill),
                )
            },
        );

        let text_input_id = text_input::Id::new("1");

        repl_output = repl_output.push(
            Row::new()
                .push(Text::new(">>> ").color(Color::WHITE).size(16)) // Add the prompt prefix
                .push(
                    TextInput::new("", &self.input)
                        .on_input(MyMathBoardMessage::InputChanged)
                        .on_submit(MyMathBoardMessage::InputSubmitted)
                        .padding(0)
                        .size(16)
                        .width(Length::Fill)
                        .id(text_input_id.clone())
                        .style(|_, _| custom_text_input_style()),
                ),
        );

        let repl_pane = Scrollable::new(repl_output)
            .height(Length::Fill)
            .width(Length::Fill);

        let content = Column::new()
            .push(control_bar)
            .push(horizontal_divider)
            .push(
                Container::new(
                    Row::new()
                        .push(graphing_pane)
                        .push(vertical_divider)
                        .push(repl_pane),
                )
                .height(Length::FillPortion(95))
                .width(Length::Fill),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| Style {
                text_color: Some(Color::BLACK),
                background: Some(Background::from(Color::BLACK)),
                ..Default::default()
            })
            .into()
    }

    pub fn new() -> (Self, Task<MyMathBoardMessage>) {
        let mut app = MyMathBoardApp {
            input: String::new(),
            output_history: Vec::new(),
            focus_input: true,
            graph: Graph::default(),
            repl: Repl::new(),
        };

        let text_input_id = text_input::Id::new("1");
        app.focus_input = true;

        let initial_task = text_input::focus(text_input_id.clone());

        (app, initial_task)
    }
}

fn custom_text_input_style() -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::BLACK),
        border: iced::Border::default(),
        icon: Color::WHITE,
        placeholder: Color::WHITE,
        value: Color::WHITE,
        selection: Color::from_rgb(0.3, 0.3, 0.3),
    }
}
