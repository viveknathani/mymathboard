use evalexpr::eval_with_context_mut;
use evalexpr::HashMapContext;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Repl {
    context: HashMapContext,
}

#[derive(Debug, Clone, Copy)]
pub struct Array {}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub enum ReplResult {
    Empty,
    Boolean(bool),
    Number(f64),
    String(String),
    Point(Point),
    List(Array),
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            context: HashMapContext::new(),
        }
    }

    pub fn process_input(&mut self, input: &str) -> Result<ReplResult, Box<dyn Error>> {
        let evaluation_result = eval_with_context_mut(input, &mut self.context)?;
        match evaluation_result {
            evalexpr::Value::Boolean(value) => Ok(ReplResult::Boolean(value)),
            evalexpr::Value::Int(value) => Ok(ReplResult::Number(value as f64)),
            evalexpr::Value::Float(value) => Ok(ReplResult::Number(value)),
            evalexpr::Value::String(value) => Ok(ReplResult::String(value)),
            _ => Ok(ReplResult::Empty),
        }
    }
}
