use iced::Point;
use iced::Vector;

#[derive(Debug, Clone)]
pub enum MyMathBoardMessage {
    Dragged(Vector),
    StartDrag(Point),
    EndDrag,
    ZoomIn,
    ZoomOut,
    DrawEquation(String),
    InputChanged(String),
    InputSubmitted,
    ClearRepl,
    ExportGraph,
    SavePressed,
    OpenPressed,
    SaveAsPressed,
}
