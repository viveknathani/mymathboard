use super::graph::Graph;
use super::types::MyMathBoardMessage;
use evalexpr::build_operator_tree;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Row;
use iced::widget::Space;
use iced::widget::Text;
use iced::Background;
use iced::Color;
use iced::Element;
use iced::Length;
use iced::Point;

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

        let vertical_divider = Container::new(Space::with_width(Length::Fixed(1.0))) // Small width for the line
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::WHITE)), // White line color
                ..Default::default()
            });

        let repl_pane = Container::new(Text::new("repl").color(Color::WHITE))
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .center_x(Length::Fill);

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
}
