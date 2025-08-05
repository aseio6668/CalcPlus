use crate::calculator::{calculate, format_number, CalculatorMode, Operation};
use crate::ui;
use eframe::egui;
use std::collections::VecDeque;

const MAX_HISTORY_SIZE: usize = 100;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CalcsPlus {
    display: String,
    current_value: f64,
    previous_value: f64,
    operation: Option<Operation>,
    mode: CalculatorMode,
    history: VecDeque<String>,
    memory: f64,
    should_clear_display: bool,
    error_message: Option<String>,
    #[serde(skip)]
    show_history: bool,
    equation_display: String,
    in_equation: bool,
}

impl Default for CalcsPlus {
    fn default() -> Self {
        Self {
            display: "0".to_string(),
            current_value: 0.0,
            previous_value: 0.0,
            operation: None,
            mode: CalculatorMode::Standard,
            history: VecDeque::new(),
            memory: 0.0,
            should_clear_display: false,
            error_message: None,
            show_history: false,
            equation_display: String::new(),
            in_equation: false,
        }
    }
}

impl CalcsPlus {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn input_digit(&mut self, digit: char) {
        if self.should_clear_display {
            self.display.clear();
            self.should_clear_display = false;
        }
        
        // If we're showing an operation, start entering the second number
        if self.operation.is_some() && !self.in_equation {
            self.display.clear();
            self.in_equation = true;
        }
        
        if self.display == "0" && digit != '.' {
            self.display = digit.to_string();
        } else {
            self.display.push(digit);
        }
        
        // Update equation display
        if self.in_equation && self.operation.is_some() {
            self.equation_display = format!("{}{}{}",
                format_number(self.previous_value),
                self.operation.unwrap().to_symbol(),
                self.display
            );
        } else {
            self.equation_display = self.display.clone();
        }
        
        self.error_message = None;
    }

    pub fn input_decimal(&mut self) {
        if self.should_clear_display {
            self.display = "0.".to_string();
            self.should_clear_display = false;
        } else {
            // If we're showing an operation, start entering the second number
            if self.operation.is_some() && !self.in_equation {
                self.display = "0.".to_string();
                self.in_equation = true;
            } else if !self.display.contains('.') {
                self.display.push('.');
            }
        }
        
        // Update equation display
        if self.in_equation && self.operation.is_some() {
            self.equation_display = format!("{}{}{}",
                format_number(self.previous_value),
                self.operation.unwrap().to_symbol(),
                self.display
            );
        } else {
            self.equation_display = self.display.clone();
        }
        
        self.error_message = None;
    }

    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.equation_display = "0".to_string();
        self.current_value = 0.0;
        self.previous_value = 0.0;
        self.operation = None;
        self.should_clear_display = false;
        self.error_message = None;
        self.in_equation = false;
    }

    pub fn clear_entry(&mut self) {
        self.display = "0".to_string();
        
        // Update equation display based on current state
        if self.operation.is_some() && !self.in_equation {
            self.equation_display = format!("{}{}",
                format_number(self.previous_value),
                self.operation.unwrap().to_symbol()
            );
        } else {
            self.equation_display = "0".to_string();
        }
        
        self.error_message = None;
    }

    pub fn backspace(&mut self) {
        if self.display.len() > 1 {
            self.display.pop();
        } else {
            self.display = "0".to_string();
        }
        self.error_message = None;
    }

    pub fn set_operation(&mut self, op: Operation) {
        if let Ok(value) = self.display.parse::<f64>() {
            if self.operation.is_some() && self.in_equation {
                // Complete the current operation first
                self.calculate_result();
                self.previous_value = self.current_value;
            } else {
                self.previous_value = value;
            }
            
            self.operation = Some(op);
            self.in_equation = false;
            
            // Update equation display to show the operation
            self.equation_display = format!("{}{}",
                format_number(self.previous_value),
                op.to_symbol()
            );
        }
    }

    pub fn perform_unary_operation(&mut self, op: Operation) {
        if let Ok(value) = self.display.parse::<f64>() {
            match calculate(op, value, None) {
                Ok(result) => {
                    let operation_str = match op {
                        Operation::SquareRoot => format!("√({})", value),
                        Operation::Square => format!("({})²", value),
                        Operation::Reciprocal => format!("1/({})", value),
                        Operation::Percentage => format!("{}%", value),
                        Operation::Sin => format!("sin({})", value),
                        Operation::Cos => format!("cos({})", value),
                        Operation::Tan => format!("tan({})", value),
                        Operation::Log => format!("log({})", value),
                        Operation::Ln => format!("ln({})", value),
                        Operation::Factorial => format!("{}!", value),
                        _ => format!("{:?}({})", op, value),
                    };
                    
                    self.add_to_history(&format!("{} = {}", operation_str, format_number(result)));
                    self.display = format_number(result);
                    self.current_value = result;
                    self.should_clear_display = true;
                    self.error_message = None;
                }
                Err(err) => {
                    self.error_message = Some(err);
                    self.display = "Error".to_string();
                }
            }
        }
    }

    pub fn calculate_result(&mut self) {
        if let Some(op) = self.operation {
            if let Ok(current) = self.display.parse::<f64>() {
                match calculate(op, self.previous_value, Some(current)) {
                    Ok(result) => {
                        let operation_str = format!("{} {} {} = {}",
                            format_number(self.previous_value),
                            op.to_symbol().trim(),
                            format_number(current),
                            format_number(result)
                        );
                        
                        self.add_to_history(&operation_str);
                        
                        self.current_value = result;
                        self.display = format_number(result);
                        self.equation_display = format_number(result);
                        self.operation = None;
                        self.in_equation = false;
                        self.should_clear_display = true;
                        self.error_message = None;
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                        self.equation_display = "Error".to_string();
                        self.operation = None;
                        self.in_equation = false;
                    }
                }
            }
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            CalculatorMode::Standard => CalculatorMode::Scientific,
            CalculatorMode::Scientific => CalculatorMode::Standard,
        };
    }

    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
    }

    pub fn memory_recall(&mut self) {
        self.display = format_number(self.memory);
        self.should_clear_display = true;
    }

    pub fn memory_store(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            self.memory = value;
        }
    }

    pub fn memory_add(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            self.memory += value;
        }
    }

    pub fn memory_subtract(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            self.memory -= value;
        }
    }

    fn add_to_history(&mut self, entry: &str) {
        self.history.push_front(entry.to_string());
        if self.history.len() > MAX_HISTORY_SIZE {
            self.history.pop_back();
        }
    }

    pub fn toggle_history(&mut self) {
        self.show_history = !self.show_history;
    }

    pub fn get_display(&self) -> &str {
        if self.equation_display.is_empty() {
            &self.display
        } else {
            &self.equation_display
        }
    }

    pub fn get_mode(&self) -> CalculatorMode {
        self.mode
    }

    pub fn get_error(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn has_memory(&self) -> bool {
        self.memory != 0.0
    }

    pub fn get_history(&self) -> &VecDeque<String> {
        &self.history
    }

    pub fn is_showing_history(&self) -> bool {
        self.show_history
    }
}

impl eframe::App for CalcsPlus {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::draw_calculator(ctx, self);
    }
}
