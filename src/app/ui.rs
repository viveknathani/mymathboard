use super::constants::APP_ICON;
use super::constants::DEFAULT_APP_WINDOW_HEIGHT;
use super::constants::DEFAULT_APP_WINDOW_WIDTH;
use super::constants::DEFAUTL_APP_NAME;
use super::constants::REPL_BACKGROUND_COLOR;
use super::constants::REPL_TEXT_INPUT_ID;
use super::graph::Graph;
use super::types::MyMathBoardMessage;
use super::types::OutputHistoryItem;
use super::types::OutputHistoryItemType;
use super::utils::get_board_name;
use crate::repl::Repl;
use evalexpr::build_operator_tree;
use iced::application;
use iced::widget::button;
use iced::widget::canvas;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::text_input;
use iced::widget::Button;
use iced::widget::Column;
use iced::widget::Container;
use iced::widget::Row;
use iced::widget::Scrollable;
use iced::widget::Space;
use iced::widget::Text;
use iced::widget::TextInput;
use iced::window;
use iced::window::Settings;
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Point;
use iced::Size;
use iced::Task;
use image::ImageFormat;
use image::RgbaImage;
use rfd::FileDialog;
use std::fs::File;
use std::io::{Read, Write};

/// The fundamental data structure used to run the app.
#[derive(Default)]
pub struct MyMathBoardApp {
    repl: Repl,
    graph: Graph,
    repl_input: String,
    repl_input_id: String,
    repl_input_history: Vec<String>,
    repl_output_history: Vec<OutputHistoryItem>,
    repl_should_input_be_in_focus: bool,
    current_open_file_path: Option<String>,
    board_has_unsaved_changes: bool,
}

impl MyMathBoardApp {
    /// Call this function to start the app
    pub fn start() -> iced::Result {
        application(
            DEFAUTL_APP_NAME,
            MyMathBoardApp::update,
            MyMathBoardApp::view,
        )
        .window(Settings {
            size: Size {
                height: DEFAULT_APP_WINDOW_HEIGHT,
                width: DEFAULT_APP_WINDOW_WIDTH,
            },
            position: window::Position::Centered,
            min_size: None,
            max_size: None,
            visible: true,
            decorations: true,
            transparent: false,
            level: window::Level::Normal,
            icon: Some(window::icon::from_file_data(APP_ICON, Some(ImageFormat::Ico)).unwrap()),
            ..Settings::default()
        })
        .run_with(|| MyMathBoardApp::new())
    }

    /// Get a new instance. You should prefer using the start() method.
    pub fn new() -> (Self, Task<MyMathBoardMessage>) {
        let app = MyMathBoardApp {
            graph: Graph::default(),
            repl: Repl::new(),
            repl_input: String::new(),
            repl_input_id: REPL_TEXT_INPUT_ID.to_string(),
            repl_input_history: Vec::new(),
            repl_output_history: Vec::new(),
            repl_should_input_be_in_focus: true,
            current_open_file_path: None,
            board_has_unsaved_changes: false,
        };

        let initial_task = text_input::focus(text_input::Id::new(app.repl_input_id.clone()));

        (app, initial_task)
    }

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
                self.repl_input = new_input;

                Task::none()
            }
            MyMathBoardMessage::InputSubmitted => {
                if self.current_open_file_path.is_some() {
                    self.board_has_unsaved_changes = true;
                }

                self.repl_input_history.push(self.repl_input.clone());

                self.process_repl_input();

                self.repl_input.clear();

                self.repl_should_input_be_in_focus = true;

                text_input::focus(self.repl_input_id.clone())
            }
            MyMathBoardMessage::ClearRepl => {
                if self.current_open_file_path.is_some() {
                    self.board_has_unsaved_changes = true;
                }

                self.repl_input_history.clear();
                self.repl_output_history.clear();

                Task::none()
            }
            MyMathBoardMessage::ExportGraph => iced::window::get_latest()
                .and_then(move |window| iced::window::screenshot(window))
                .then(move |screenshot| {
                    if let Some(path) = FileDialog::new().add_filter("png", &["png"]).save_file() {
                        let png = RgbaImage::from_raw(
                            screenshot.size.width,
                            screenshot.size.height,
                            screenshot.bytes.to_vec(),
                        )
                        .unwrap();
                        png.save_with_format(path, image::ImageFormat::Png).unwrap();
                    }
                    Task::none()
                }),
            MyMathBoardMessage::SavePressed => {
                if let Some(path) = &self.current_open_file_path {
                    let _ = self.save_to_file(path);
                } else if let Some(path) = FileDialog::new()
                    .add_filter("MyMathBoard", &["mymathboard"])
                    .save_file()
                {
                    let path_str = path.to_string_lossy().to_string();
                    self.current_open_file_path = Some(path_str.clone());
                    let _ = self.save_to_file(&path_str);
                }

                self.board_has_unsaved_changes = false;
                Task::none()
            }
            MyMathBoardMessage::OpenPressed => {
                if let Some(path) = FileDialog::new()
                    .add_filter("MyMathBoard", &["mymathboard"])
                    .pick_file()
                {
                    let path_str = path.to_string_lossy().to_string();

                    self.repl_input.clear();
                    self.repl_input_history.clear();
                    self.repl_output_history.clear();

                    if self.load_from_file(&path_str).is_ok() {
                        self.current_open_file_path = Some(path_str);
                        for command in &self.repl_input_history.clone() {
                            self.repl_input = command.clone();
                            self.process_repl_input();
                        }
                    }
                }
                Task::none()
            }
            MyMathBoardMessage::SaveAsPressed => {
                if let Some(path) = FileDialog::new()
                    .add_filter("MyMathBoard", &["mymathboard"])
                    .save_file()
                {
                    let path_str = path.to_string_lossy().to_string();
                    self.current_open_file_path = Some(path_str.clone());
                    let _ = self.save_to_file(&path_str);
                }
                Task::none()
            }
        }
    }

    pub fn process_repl_input(&mut self) {
        if self.repl_input.starts_with("draw(") && self.repl_input.ends_with(")") {
            let equation = self
                .repl_input
                .strip_prefix("draw(")
                .unwrap()
                .strip_suffix(")")
                .unwrap();

            let result = "";

            let node_formation = build_operator_tree(&equation);

            if node_formation.is_ok() {
                self.graph.equations.push(node_formation.unwrap());
            }

            self.repl_output_history.push(OutputHistoryItem {
                value: format!(">>> {}\n", self.repl_input),
                kind: OutputHistoryItemType::PreviousInput,
            });

            self.repl_output_history.push(OutputHistoryItem {
                value: format!("=> {:?}\n", result),
                kind: OutputHistoryItemType::OkOutput,
            });
        } else {
            let result = self.repl.process_input(&self.repl_input);

            self.repl_output_history.push(OutputHistoryItem {
                value: format!(">>> {}\n", self.repl_input),
                kind: OutputHistoryItemType::PreviousInput,
            });

            self.repl_output_history.push(OutputHistoryItem {
                value: format!("=> {:?}\n", result),
                kind: match result {
                    Ok(_) => OutputHistoryItemType::OkOutput,
                    Err(_) => OutputHistoryItemType::ErrOutput,
                },
            });
        }
    }

    pub fn view(&self) -> Element<MyMathBoardMessage> {
        // CONTROL BAR
        let open_button = Button::new(Text::new("OPEN").size(14))
            .on_press(MyMathBoardMessage::OpenPressed)
            .height(25)
            .padding(2)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb8(52, 134, 235))),
                border: Border::default(),
                text_color: Color::WHITE,
                ..Default::default()
            });
        let save_button = Button::new(Text::new("SAVE").size(14))
            .on_press(MyMathBoardMessage::SavePressed)
            .height(25)
            .padding(2)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb8(52, 134, 235))),
                border: Border::default(),
                text_color: Color::WHITE,
                ..Default::default()
            });
        let save_as_button = Button::new(Text::new("SAVE AS").size(14))
            .on_press(MyMathBoardMessage::SaveAsPressed)
            .height(25)
            .padding(2)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb8(52, 134, 235))),
                border: Border::default(),
                text_color: Color::WHITE,
                ..Default::default()
            });
        let control_bar = Row::new()
            .push(Space::with_width(Length::Fixed(10.0)))
            .push(
                Text::new(get_board_name(
                    self.current_open_file_path.clone(),
                    self.board_has_unsaved_changes,
                ))
                .color(Color::WHITE)
                .font(Font::MONOSPACE),
            )
            .push(Space::with_width(Length::Fill))
            .push(open_button)
            .push(Space::with_width(Length::Fixed(10.0)))
            .push(save_button)
            .push(Space::with_width(Length::Fixed(10.0)))
            .push(save_as_button)
            .push(Space::with_width(Length::Fixed(10.0)))
            .height(Length::FillPortion(4))
            .width(Length::Fill);

        // DIVIDER
        let horizontal_divider_top = Container::new(Space::with_height(Length::Fixed(1.0)))
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            });

        let horizontal_divider_repl = Container::new(Space::with_height(Length::Fixed(1.0)))
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            });

        let horizontal_divider_graph = Container::new(Space::with_height(Length::Fixed(1.0)))
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            });

        let canvas = canvas(self.graph.clone())
            .width(Length::Fill)
            .height(Length::Fill);

        let coords_text = "x: -, y: -".to_string();

        let coords_display = Text::new(coords_text).color(Color::WHITE).size(16);

        let zoom_in_button = Button::new(Text::new("+").color(Color::WHITE).size(16))
            .padding(5)
            .on_press(MyMathBoardMessage::ZoomIn)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                ..Default::default()
            });

        let zoom_reset_button = Button::new(Text::new("Reset").color(Color::WHITE).size(16))
            .padding(5)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                ..Default::default()
            });

        let zoom_out_button = Button::new(Text::new("-").color(Color::WHITE).size(16))
            .padding(5)
            .on_press(MyMathBoardMessage::ZoomOut)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                ..Default::default()
            });

        let graph_clear_button = Button::new(Text::new("Clear").color(Color::WHITE).size(16))
            .padding(5)
            .on_press(MyMathBoardMessage::ExportGraph)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                ..Default::default()
            });

        let graph_bottom_bar = Row::new()
            .push(coords_display)
            .push(Space::with_width(Length::Fill))
            .push(zoom_in_button)
            .push(Space::with_width(Length::Fill))
            .push(zoom_reset_button)
            .push(Space::with_width(Length::Fill))
            .push(zoom_out_button)
            .push(Space::with_width(Length::Fill))
            .push(graph_clear_button)
            .height(Length::Fixed(30.0))
            .padding(5);

        let graphing_pane = Column::new()
            .push(canvas)
            .push(horizontal_divider_graph)
            .push(graph_bottom_bar)
            .height(Length::Fill)
            .width(Length::Fill);

        let vertical_divider = Container::new(Space::with_width(Length::Fixed(1.0)))
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            });

        let clear_button = Button::new(Text::new("CLEAR").color(Color::WHITE).size(14))
            .on_press(MyMathBoardMessage::ClearRepl)
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(Color::from_rgb8(52, 134, 235))),
                border: Border::default(),
                text_color: Color::WHITE,
                ..Default::default()
            })
            .height(25)
            .padding(2);

        let bottom_bar = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(clear_button)
            .height(Length::Fixed(30.0))
            .padding(5);

        let mut repl_output = self.repl_output_history.iter().fold(
            Column::new().spacing(5).width(Length::Fill),
            |column, entry| {
                column.push(
                    Container::new(
                        Text::new(&entry.value)
                            .color(match entry.kind {
                                OutputHistoryItemType::PreviousInput => Color::from_rgb8(0, 200, 0),
                                OutputHistoryItemType::ErrOutput => Color::from_rgb8(255, 0, 0),
                                OutputHistoryItemType::OkOutput => Color::from_rgb8(255, 255, 255),
                            })
                            .size(16)
                            .font(Font::MONOSPACE),
                    )
                    .padding(2)
                    .width(Length::Fill),
                )
            },
        );

        let text_input_id = text_input::Id::new("1");

        repl_output = repl_output.push(
            Row::new()
                .push(
                    Container::new(
                        Text::new(">>> ")
                            .color(Color::from_rgb8(0, 200, 0))
                            .size(16)
                            .font(Font::MONOSPACE),
                    )
                    .padding(2),
                )
                .push(
                    TextInput::new("", &self.repl_input)
                        .on_input(MyMathBoardMessage::InputChanged)
                        .on_submit(MyMathBoardMessage::InputSubmitted)
                        .padding(0)
                        .size(16)
                        .width(Length::Fill)
                        .id(text_input_id.clone())
                        .style(|_, _| text_input::Style {
                            background: Background::Color(Color::from_rgb8(
                                REPL_BACKGROUND_COLOR.0,
                                REPL_BACKGROUND_COLOR.1,
                                REPL_BACKGROUND_COLOR.2,
                            )),
                            border: iced::Border::default(),
                            icon: Color::WHITE,
                            placeholder: Color::WHITE,
                            value: Color::from_rgb8(0, 200, 0),
                            selection: Color::from_rgb(0.3, 0.3, 0.3),
                        })
                        .font(Font::MONOSPACE),
                ),
        );

        let repl_pane = Container::new(
            Column::new()
                .push(
                    Scrollable::new(repl_output)
                        .height(Length::Fill)
                        .width(Length::Fill),
                )
                .push(horizontal_divider_repl)
                .push(bottom_bar)
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Background::Color(Color::from_rgb8(
                REPL_BACKGROUND_COLOR.0,
                REPL_BACKGROUND_COLOR.1,
                REPL_BACKGROUND_COLOR.2,
            ))),
            ..Default::default()
        });

        let content = Column::new()
            .push(Space::with_height(Length::Fixed(2.0)))
            .push(control_bar)
            .push(horizontal_divider_top)
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

    fn save_to_file(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?;

        let encoded_data = bincode::serialize(&self.repl_input_history).unwrap_or_default();

        file.write_all(&encoded_data)?;
        Ok(())
    }

    fn load_from_file(&mut self, file_path: &str) -> Result<(), std::io::Error> {
        let mut file = File::open(file_path)?;

        let mut encoded_data = Vec::new();

        file.read_to_end(&mut encoded_data)?;

        self.repl_input_history = bincode::deserialize(&encoded_data).unwrap_or_default();

        Ok(())
    }
}
