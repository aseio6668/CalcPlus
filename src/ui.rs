use crate::app::CalcsPlus;
use crate::calculator::{CalculatorMode, Operation};
use eframe::egui::{self, Color32, RichText, Stroke, Vec2};

const BUTTON_SIZE: Vec2 = Vec2::new(65.0, 45.0);
const LARGE_BUTTON_SIZE: Vec2 = Vec2::new(135.0, 45.0);
const DISPLAY_HEIGHT: f32 = 70.0;
const SPACING: f32 = 4.0;

pub fn draw_calculator(ctx: &egui::Context, app: &mut CalcsPlus) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            
            // Title bar with mode toggle
            draw_title_bar(ui, app);
            
            ui.add_space(8.0);
            
            // Display area
            draw_display(ui, app);
            
            ui.add_space(8.0);
            
            // Show history panel if enabled
            if app.is_showing_history() {
                draw_history_panel(ui, app);
                ui.add_space(8.0);
            }
            
            // Button layout based on mode
            match app.get_mode() {
                CalculatorMode::Standard => draw_standard_buttons(ui, app),
                CalculatorMode::Scientific => draw_scientific_buttons(ui, app),
            }
            
            ui.add_space(8.0);
        });
    });
}

fn draw_title_bar(ui: &mut egui::Ui, app: &mut CalcsPlus) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("CalcsPlus")
                .size(24.0)
                .color(Color32::from_rgb(70, 130, 180))
                .strong()
        );
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // History toggle button
            if ui.button(
                RichText::new("📋")
                    .size(18.0)
            ).clicked() {
                app.toggle_history();
            }
            
            // Mode toggle button
            let mode_text = match app.get_mode() {
                CalculatorMode::Standard => "Standard",
                CalculatorMode::Scientific => "Scientific",
            };
            
            if ui.button(
                RichText::new(mode_text)
                    .size(14.0)
                    .color(Color32::from_rgb(100, 149, 237))
            ).clicked() {
                app.toggle_mode();
            }
        });
    });
}

fn draw_display(ui: &mut egui::Ui, app: &CalcsPlus) {
    let frame = egui::Frame::none()
        .fill(Color32::from_rgb(248, 248, 255))
        .stroke(Stroke::new(2.0, Color32::from_rgb(200, 200, 200)))
        .inner_margin(egui::Margin::same(15.0))
        .rounding(8.0);
    
    frame.show(ui, |ui| {
        ui.set_min_height(DISPLAY_HEIGHT);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            // Error message or main display
            if let Some(error) = app.get_error() {
                ui.label(
                    RichText::new(error)
                        .size(20.0)
                        .color(Color32::from_rgb(220, 20, 60))
                        .monospace()
                );
            } else {
                ui.label(
                    RichText::new(app.get_display())
                        .size(32.0)
                        .color(Color32::from_rgb(25, 25, 112))
                        .monospace()
                        .strong()
                );
            }
        });
        
        // Memory indicator
        if app.has_memory() {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.label(
                    RichText::new("M")
                        .size(14.0)
                        .color(Color32::from_rgb(255, 140, 0))
                        .strong()
                );
            });
        }
    });
}

fn draw_history_panel(ui: &mut egui::Ui, app: &CalcsPlus) {
    let frame = egui::Frame::none()
        .fill(Color32::from_rgb(250, 250, 250))
        .stroke(Stroke::new(1.0, Color32::from_rgb(220, 220, 220)))
        .inner_margin(egui::Margin::same(10.0))
        .rounding(5.0);
    
    frame.show(ui, |ui| {
        ui.set_max_height(150.0);
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(
                RichText::new("History")
                    .size(14.0)
                    .strong()
                    .color(Color32::from_rgb(70, 130, 180))
            );
            ui.separator();
            
            for entry in app.get_history() {
                ui.label(
                    RichText::new(entry)
                        .size(12.0)
                        .color(Color32::from_rgb(60, 60, 60))
                        .monospace()
                );
            }
        });
    });
}

fn draw_standard_buttons(ui: &mut egui::Ui, app: &mut CalcsPlus) {
    ui.spacing_mut().item_spacing = Vec2::splat(SPACING);
    
    // Memory and Clear buttons row
    ui.horizontal(|ui| {
        if calc_button(ui, "MC", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_clear();
        }
        if calc_button(ui, "MR", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_recall();
        }
        if calc_button(ui, "M+", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_add();
        }
        if calc_button(ui, "M-", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_subtract();
        }
        if calc_button(ui, "MS", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_store();
        }
    });
    
    // Clear buttons row
    ui.horizontal(|ui| {
        if calc_button(ui, "CE", BUTTON_SIZE, Color32::from_rgb(255, 204, 204)).clicked() {
            app.clear_entry();
        }
        if calc_button(ui, "C", BUTTON_SIZE, Color32::from_rgb(255, 204, 204)).clicked() {
            app.clear();
        }
        if calc_button(ui, "⌫", BUTTON_SIZE, Color32::from_rgb(255, 204, 204)).clicked() {
            app.backspace();
        }
        if calc_button(ui, "±", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            if let Ok(mut val) = app.get_display().parse::<f64>() {
                val = -val;
                app.clear_entry();
                for ch in val.to_string().chars() {
                    if ch == '.' {
                        app.input_decimal();
                    } else {
                        app.input_digit(ch);
                    }
                }
            }
        }
    });
    
    // Function buttons row
    ui.horizontal(|ui| {
        if calc_button(ui, "√", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.perform_unary_operation(Operation::SquareRoot);
        }
        if calc_button(ui, "x²", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.perform_unary_operation(Operation::Square);
        }
        if calc_button(ui, "1/x", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.perform_unary_operation(Operation::Reciprocal);
        }
        if calc_button(ui, "÷", BUTTON_SIZE, Color32::from_rgb(173, 216, 230)).clicked() {
            app.set_operation(Operation::Divide);
        }
    });
    
    // Number row 7-9
    ui.horizontal(|ui| {
        if calc_button(ui, "7", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('7');
        }
        if calc_button(ui, "8", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('8');
        }
        if calc_button(ui, "9", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('9');
        }
        if calc_button(ui, "×", BUTTON_SIZE, Color32::from_rgb(173, 216, 230)).clicked() {
            app.set_operation(Operation::Multiply);
        }
    });
    
    // Number row 4-6
    ui.horizontal(|ui| {
        if calc_button(ui, "4", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('4');
        }
        if calc_button(ui, "5", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('5');
        }
        if calc_button(ui, "6", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('6');
        }
        if calc_button(ui, "-", BUTTON_SIZE, Color32::from_rgb(173, 216, 230)).clicked() {
            app.set_operation(Operation::Subtract);
        }
    });
    
    // Number row 1-3
    ui.horizontal(|ui| {
        if calc_button(ui, "1", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('1');
        }
        if calc_button(ui, "2", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('2');
        }
        if calc_button(ui, "3", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('3');
        }
        if calc_button(ui, "+", BUTTON_SIZE, Color32::from_rgb(173, 216, 230)).clicked() {
            app.set_operation(Operation::Add);
        }
    });
    
    // Bottom row
    ui.horizontal(|ui| {
        if calc_button(ui, "%", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.perform_unary_operation(Operation::Percentage);
        }
        if calc_button(ui, "0", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('0');
        }
        if calc_button(ui, ".", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_decimal();
        }
        if calc_button(ui, "=", BUTTON_SIZE, Color32::from_rgb(100, 149, 237)).clicked() {
            app.calculate_result();
        }
    });
}

fn draw_scientific_buttons(ui: &mut egui::Ui, app: &mut CalcsPlus) {
    ui.spacing_mut().item_spacing = Vec2::splat(SPACING);
    
    // First row - Memory and clear
    ui.horizontal(|ui| {
        if calc_button(ui, "MC", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_clear();
        }
        if calc_button(ui, "MR", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_recall();
        }
        if calc_button(ui, "M+", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_add();
        }
        if calc_button(ui, "M-", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_subtract();
        }
        if calc_button(ui, "MS", BUTTON_SIZE, Color32::from_rgb(255, 182, 193)).clicked() {
            app.memory_store();
        }
    });
    
    // Second row - Clear and backspace
    ui.horizontal(|ui| {
        if calc_button(ui, "CE", BUTTON_SIZE, Color32::from_rgb(255, 204, 204)).clicked() {
            app.clear_entry();
        }
        if calc_button(ui, "C", BUTTON_SIZE, Color32::from_rgb(255, 204, 204)).clicked() {
            app.clear();
        }
        if calc_button(ui, "⌫", BUTTON_SIZE, Color32::from_rgb(255, 204, 204)).clicked() {
            app.backspace();
        }
        if calc_button(ui, "±", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            if let Ok(mut val) = app.get_display().parse::<f64>() {
                val = -val;
                app.clear_entry();
                for ch in val.to_string().chars() {
                    if ch == '.' {
                        app.input_decimal();
                    } else {
                        app.input_digit(ch);
                    }
                }
            }
        }
        if calc_button(ui, "÷", BUTTON_SIZE, Color32::from_rgb(173, 216, 230)).clicked() {
            app.set_operation(Operation::Divide);
        }
    });
    
    // Third row - Scientific functions
    ui.horizontal(|ui| {
        if calc_button(ui, "sin", BUTTON_SIZE, Color32::from_rgb(221, 160, 221)).clicked() {
            app.perform_unary_operation(Operation::Sin);
        }
        if calc_button(ui, "cos", BUTTON_SIZE, Color32::from_rgb(221, 160, 221)).clicked() {
            app.perform_unary_operation(Operation::Cos);
        }
        if calc_button(ui, "tan", BUTTON_SIZE, Color32::from_rgb(221, 160, 221)).clicked() {
            app.perform_unary_operation(Operation::Tan);
        }
        if calc_button(ui, "log", BUTTON_SIZE, Color32::from_rgb(221, 160, 221)).clicked() {
            app.perform_unary_operation(Operation::Log);
        }
        if calc_button(ui, "×", BUTTON_SIZE, Color32::from_rgb(173, 216, 230)).clicked() {
            app.set_operation(Operation::Multiply);
        }
    });
    
    // Fourth row - More functions and numbers
    ui.horizontal(|ui| {
        if calc_button(ui, "ln", BUTTON_SIZE, Color32::from_rgb(221, 160, 221)).clicked() {
            app.perform_unary_operation(Operation::Ln);
        }
        if calc_button(ui, "x²", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.perform_unary_operation(Operation::Square);
        }
        if calc_button(ui, "√", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.perform_unary_operation(Operation::SquareRoot);
        }
        if calc_button(ui, "x^y", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.set_operation(Operation::Power);
        }
        if calc_button(ui, "-", BUTTON_SIZE, Color32::from_rgb(173, 216, 230)).clicked() {
            app.set_operation(Operation::Subtract);
        }
    });
    
    // Number rows
    ui.horizontal(|ui| {
        if calc_button(ui, "n!", BUTTON_SIZE, Color32::from_rgb(221, 160, 221)).clicked() {
            app.perform_unary_operation(Operation::Factorial);
        }
        if calc_button(ui, "7", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('7');
        }
        if calc_button(ui, "8", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('8');
        }
        if calc_button(ui, "9", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('9');
        }
        if calc_button(ui, "+", BUTTON_SIZE, Color32::from_rgb(173, 216, 230)).clicked() {
            app.set_operation(Operation::Add);
        }
    });
    
    ui.horizontal(|ui| {
        if calc_button(ui, "1/x", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.perform_unary_operation(Operation::Reciprocal);
        }
        if calc_button(ui, "4", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('4');
        }
        if calc_button(ui, "5", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('5');
        }
        if calc_button(ui, "6", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('6');
        }
        if calc_button(ui, "=", BUTTON_SIZE, Color32::from_rgb(100, 149, 237)).clicked() {
            app.calculate_result();
        }
    });
    
    // Bottom rows
    ui.horizontal(|ui| {
        if calc_button(ui, "%", BUTTON_SIZE, Color32::from_rgb(230, 230, 250)).clicked() {
            app.perform_unary_operation(Operation::Percentage);
        }
        if calc_button(ui, "1", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('1');
        }
        if calc_button(ui, "2", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('2');
        }
        if calc_button(ui, "3", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('3');
        }
        ui.add_space(BUTTON_SIZE.x + SPACING);
    });
    
    ui.horizontal(|ui| {
        ui.add_space(BUTTON_SIZE.x + SPACING);
        if calc_button(ui, "0", LARGE_BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_digit('0');
        }
        if calc_button(ui, ".", BUTTON_SIZE, Color32::WHITE).clicked() {
            app.input_decimal();
        }
        ui.add_space(BUTTON_SIZE.x + SPACING);
    });
}

fn calc_button(ui: &mut egui::Ui, text: &str, size: Vec2, color: Color32) -> egui::Response {
    let button = egui::Button::new(
        RichText::new(text)
            .size(16.0)
            .color(Color32::from_rgb(50, 50, 50))
            .strong()
    )
    .fill(color)
    .stroke(Stroke::new(1.0, Color32::from_rgb(180, 180, 180)))
    .min_size(size);
    
    ui.add(button)
}
