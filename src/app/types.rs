use iced::Point;
use iced::Vector;

#[derive(Debug, Clone)]
pub enum MyMathBoardMessage {
    Dragged(Vector),
    StartDrag(Point, f32, f32),
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

#[derive(Debug, Clone, PartialEq)]
pub enum OutputHistoryItemType {
    PreviousInput,
    OkOutput,
    ErrOutput,
}

#[derive(Debug, Clone)]
pub struct OutputHistoryItem {
    pub value: String,
    pub kind: OutputHistoryItemType,
}
