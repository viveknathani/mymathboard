use evalexpr::eval_with_context_mut;
use evalexpr::ContextWithMutableFunctions;
use evalexpr::EvalexprError;
use evalexpr::Function;
use evalexpr::HashMapContext;
use evalexpr::Value;
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

#[derive(Debug)]
pub enum ReplError {
    InvalidType,
    InvalidArgument,
}

impl Repl {
    pub fn new() -> Self {
        let mut object = Repl {
            context: HashMapContext::new(),
        };
        object.setup_math_functions();
        object
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

    fn setup_math_functions(&mut self) {
        self.context
            .set_function(
                "sin".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.sin()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).sin()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "cos".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.cos()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).cos()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "tan".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.tan()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).tan()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "cosec".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(1.0 / x.sin()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float(1.0 / (*x as f64).sin()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "sec".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(1.0 / x.cos()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float(1.0 / (*x as f64).cos()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "cot".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(1.0 / x.tan()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float(1.0 / (*x as f64).tan()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "asin".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.asin()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).asin()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "acos".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.acos()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).acos()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "atan".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.atan()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).atan()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "acosec".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float((1.0 / x).asin()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((1.0 / (*x as f64)).asin()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "asec".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float((1.0 / x).acos()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((1.0 / (*x as f64)).acos()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "acot".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float((1.0 / x).atan()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((1.0 / (*x as f64)).atan()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "sinh".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.sinh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).sinh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "cosh".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.cosh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).cosh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "tanh".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.tanh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).tanh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "cosech".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(1.0 / x.sinh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float(1.0 / (*x as f64).sinh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "sech".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(1.0 / x.cosh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float(1.0 / (*x as f64).cosh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "coth".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(1.0 / x.tanh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float(1.0 / (*x as f64).tanh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "asinh".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.asinh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).asinh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "acosh".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.acosh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).acosh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "atanh".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float(x.atanh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((*x as f64).atanh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "acosech".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float((1.0 / x).asinh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((1.0 / (*x as f64)).asinh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "asech".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float((1.0 / x).acosh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((1.0 / (*x as f64)).acosh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "acoth".to_string(),
                Function::new(|args| {
                    if let Value::Float(x) = &args {
                        Ok(Value::Float((1.0 / x).atanh()))
                    } else if let Value::Int(x) = &args {
                        Ok(Value::Float((1.0 / (*x as f64)).atanh()))
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "log".to_string(),
                Function::new(|args| {
                    if let Value::Tuple(tuple) = &args {
                        if tuple.len() == 2 {
                            let value = &tuple[0];
                            let base = &tuple[1];

                            let base_value = match base {
                                Value::Float(x) => *x,
                                Value::Int(x) => *x as f64,
                                _ => {
                                    return Err(EvalexprError::ExpectedNumber {
                                        actual: Value::Empty,
                                    })
                                }
                            };

                            let value_value = match value {
                                Value::Float(x) => *x,
                                Value::Int(x) => *x as f64,
                                _ => {
                                    return Err(EvalexprError::ExpectedNumber {
                                        actual: Value::Empty,
                                    })
                                }
                            };

                            if base_value > 0.0 && value_value > 0.0 && base_value != 1.0 {
                                Ok(Value::Float(value_value.log(base_value)))
                            } else {
                                Err(EvalexprError::ExpectedNumber {
                                    actual: Value::Empty,
                                })
                            }
                        } else {
                            Err(EvalexprError::ExpectedNumber {
                                actual: Value::Empty,
                            })
                        }
                    } else {
                        Err(EvalexprError::ExpectedNumber {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "exp".to_string(),
                Function::new(|args| match args {
                    Value::Tuple(values) if values.len() == 1 => {
                        if let Value::Float(x) = &values[0] {
                            Ok(Value::Float(x.exp()))
                        } else if let Value::Int(x) = &values[0] {
                            Ok(Value::Float((*x as f64).exp()))
                        } else {
                            Err(EvalexprError::ExpectedNumber {
                                actual: Value::Empty,
                            })
                        }
                    }
                    _ => Err(EvalexprError::ExpectedTuple {
                        actual: Value::Empty,
                    }),
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "pow".to_string(),
                Function::new(|args| {
                    if let Value::Tuple(values) = args {
                        if values.len() == 2 {
                            let base = &values[0];
                            let exponent = &values[1];

                            let base_f = match base {
                                Value::Float(x) => *x,
                                Value::Int(x) => *x as f64,
                                _ => {
                                    return Err(EvalexprError::ExpectedNumber {
                                        actual: Value::Empty,
                                    })
                                }
                            };

                            let exp_f = match exponent {
                                Value::Float(x) => *x,
                                Value::Int(x) => *x as f64,
                                _ => {
                                    return Err(EvalexprError::ExpectedNumber {
                                        actual: Value::Empty,
                                    })
                                }
                            };

                            Ok(Value::Float(base_f.powf(exp_f)))
                        } else {
                            Err(EvalexprError::ExpectedTuple {
                                actual: Value::Empty,
                            })
                        }
                    } else {
                        Err(EvalexprError::ExpectedTuple {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "decimal_to_binary".to_string(),
                Function::new(|args| {
                    if let Value::String(x) = args {
                        let decimal: i64 =
                            x.parse().map_err(|_| EvalexprError::ExpectedNumber {
                                actual: Value::Empty,
                            })?;
                        let binary = format!("{:b}", decimal);
                        Ok(Value::String(binary))
                    } else {
                        Err(EvalexprError::ExpectedString {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "binary_to_decimal".to_string(),
                Function::new(|args| {
                    if let Value::String(x) = args {
                        let decimal = i64::from_str_radix(&x, 2).map_err(|_| {
                            EvalexprError::ExpectedNumber {
                                actual: Value::Empty,
                            }
                        })?;
                        Ok(Value::String(decimal.to_string()))
                    } else {
                        Err(EvalexprError::ExpectedString {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "decimal_to_hex".to_string(),
                Function::new(|args| {
                    if let Value::String(x) = args {
                        let decimal: i64 =
                            x.parse().map_err(|_| EvalexprError::ExpectedNumber {
                                actual: Value::Empty,
                            })?;
                        let hex = format!("{:X}", decimal); // Uppercase hexadecimal
                        Ok(Value::String(hex))
                    } else {
                        Err(EvalexprError::ExpectedString {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();

        self.context
            .set_function(
                "hex_to_decimal".to_string(),
                Function::new(|args| {
                    if let Value::String(x) = args {
                        let decimal = i64::from_str_radix(&x, 16).map_err(|_| {
                            EvalexprError::ExpectedNumber {
                                actual: Value::Empty,
                            }
                        })?;
                        Ok(Value::String(decimal.to_string()))
                    } else {
                        Err(EvalexprError::ExpectedString {
                            actual: Value::Empty,
                        })
                    }
                }),
            )
            .unwrap();
    }
}
