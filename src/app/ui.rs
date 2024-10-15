use super::graph::Graph;
use super::types::MyMathBoardMessage;
use evalexpr::build_operator_tree;
use iced::widget::canvas;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Row;
use iced::widget::TextInput;
use iced::Element;
use iced::Length;
use iced::Point;
use iced::Theme;

#[derive(Default)]
pub struct MyMathBoardApp {
    graph: Graph,
}

impl MyMathBoardApp {
    pub fn update(&mut self, message: MyMathBoardMessage) {
        match message {
            MyMathBoardMessage::Dragged(delta) => {
                self.graph.viewport_offset = self.graph.viewport_offset + delta;
                self.graph.last_cursor_position = Some(Point::new(
                    self.graph.last_cursor_position.unwrap().x + delta.x,
                    self.graph.last_cursor_position.unwrap().y + delta.y,
                ));
            }
            MyMathBoardMessage::StartDrag(position) => {
                self.graph.is_dragging = true;
                self.graph.last_cursor_position = Some(position);
            }
            MyMathBoardMessage::EndDrag => {
                self.graph.is_dragging = false;
                self.graph.last_cursor_position = None;
            }
            MyMathBoardMessage::ZoomIn => {
                self.graph.cell_size = (self.graph.cell_size as f32 * 1.1).ceil() as u64;
            }
            MyMathBoardMessage::ZoomOut => {
                self.graph.cell_size = (self.graph.cell_size as f32 * 0.9).ceil() as u64;
            }
            MyMathBoardMessage::DrawEquation(equation) => {
                let node_formation = build_operator_tree(&equation);
                if node_formation.is_ok() {
                    self.graph.equations.push(node_formation.unwrap());
                }
            }
        }
    }

    pub fn view(&self) -> Element<MyMathBoardMessage> {
        let zoom_in_button = Button::new("+")
            .on_press(MyMathBoardMessage::ZoomIn)
            .width(50)
            .height(50);

        let zoom_out_button = Button::new("-")
            .on_press(MyMathBoardMessage::ZoomOut)
            .width(50)
            .height(50);

        let canvas = canvas(self.graph.clone()).width(1000.0).height(1000.0);

        let controls = Row::new()
            .spacing(10)
            .push(zoom_in_button)
            .push(zoom_out_button)
            .padding(10);

        let equation_input =
            TextInput::<MyMathBoardMessage, Theme, iced::Renderer>::new("Enter equation", "")
                .on_input(|input| MyMathBoardMessage::DrawEquation(input.to_string()));

        Column::new()
            .push(
                Container::new(canvas)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(
                Container::new(equation_input)
                    .width(Length::Fill)
                    .height(Length::Shrink),
            )
            .push(
                Container::new(controls)
                    .width(Length::Shrink)
                    .height(Length::Shrink),
            )
            .into()
    }
}
