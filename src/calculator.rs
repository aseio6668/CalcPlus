#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    SquareRoot,
    Square,
    Reciprocal,
    Percentage,
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
    Factorial,
}

impl Operation {
    pub fn to_symbol(&self) -> &'static str {
        match self {
            Operation::Add => " + ",
            Operation::Subtract => " - ",
            Operation::Multiply => " × ",
            Operation::Divide => " ÷ ",
            Operation::Power => " ^ ",
            Operation::SquareRoot => "√",
            Operation::Square => "²",
            Operation::Reciprocal => "1/",
            Operation::Percentage => "%",
            Operation::Sin => "sin",
            Operation::Cos => "cos",
            Operation::Tan => "tan",
            Operation::Log => "log",
            Operation::Ln => "ln",
            Operation::Factorial => "!",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CalculatorMode {
    Standard,
    Scientific,
}

pub fn calculate(op: Operation, val1: f64, val2: Option<f64>) -> Result<f64, String> {
    match op {
        Operation::Add => {
            if let Some(v2) = val2 {
                Ok(val1 + v2)
            } else {
                Err("Addition requires two operands".to_string())
            }
        }
        Operation::Subtract => {
            if let Some(v2) = val2 {
                Ok(val1 - v2)
            } else {
                Err("Subtraction requires two operands".to_string())
            }
        }
        Operation::Multiply => {
            if let Some(v2) = val2 {
                Ok(val1 * v2)
            } else {
                Err("Multiplication requires two operands".to_string())
            }
        }
        Operation::Divide => {
            if let Some(v2) = val2 {
                if v2 == 0.0 {
                    Err("Cannot divide by zero".to_string())
                } else {
                    Ok(val1 / v2)
                }
            } else {
                Err("Division requires two operands".to_string())
            }
        }
        Operation::Power => {
            if let Some(v2) = val2 {
                Ok(val1.powf(v2))
            } else {
                Err("Power operation requires two operands".to_string())
            }
        }
        Operation::SquareRoot => {
            if val1 < 0.0 {
                Err("Cannot take square root of negative number".to_string())
            } else {
                Ok(val1.sqrt())
            }
        }
        Operation::Square => Ok(val1 * val1),
        Operation::Reciprocal => {
            if val1 == 0.0 {
                Err("Cannot take reciprocal of zero".to_string())
            } else {
                Ok(1.0 / val1)
            }
        }
        Operation::Percentage => Ok(val1 / 100.0),
        Operation::Sin => Ok(val1.to_radians().sin()),
        Operation::Cos => Ok(val1.to_radians().cos()),
        Operation::Tan => Ok(val1.to_radians().tan()),
        Operation::Log => {
            if val1 <= 0.0 {
                Err("Cannot take logarithm of non-positive number".to_string())
            } else {
                Ok(val1.log10())
            }
        }
        Operation::Ln => {
            if val1 <= 0.0 {
                Err("Cannot take natural logarithm of non-positive number".to_string())
            } else {
                Ok(val1.ln())
            }
        }
        Operation::Factorial => {
            if val1 < 0.0 || val1.fract() != 0.0 {
                Err("Factorial is only defined for non-negative integers".to_string())
            } else {
                let n = val1 as u64;
                if n > 20 {
                    Err("Factorial too large to compute".to_string())
                } else {
                    Ok(factorial(n) as f64)
                }
            }
        }
    }
}

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

pub fn format_number(num: f64) -> String {
    if num.is_infinite() {
        return "∞".to_string();
    }
    if num.is_nan() {
        return "Error".to_string();
    }
    
    // Remove trailing zeros and decimal point if not needed
    let formatted = format!("{:.10}", num);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    
    // Handle very large or very small numbers with scientific notation
    if num.abs() >= 1e15 || (num.abs() < 1e-10 && num != 0.0) {
        format!("{:.6e}", num)
    } else {
        trimmed.to_string()
    }
}
