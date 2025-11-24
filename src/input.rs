use crate::app::{App, InputMode};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Instant;

pub fn handle_input(app: &mut App) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            match app.input_mode {
                InputMode::Normal => {
                    return handle_normal_input(app, key);
                }
                InputMode::AddingTask | InputMode::AddingSubtask(_) => {
                    return handle_input_mode(app, key);
                }
                InputMode::Menu => {
                    return handle_menu_input(app, key);
                }
                InputMode::ConfirmingDelete | InputMode::ConfirmingClear => {
                    return handle_confirmation_input(app, key);
                }
            }
        }
    }
    Ok(false)
}

fn handle_normal_input(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') => {
            app.save_state()?;
            let _ = app.save_tasks_to_txt();
            return Ok(true);
        }
        KeyCode::Char('a') => {
            app.input_mode = InputMode::AddingTask;
            app.input_buffer.clear();
        }
        KeyCode::Char('s') => {
            if let Some(parent_id) = app.get_selected_parent_id() {
                app.input_mode = InputMode::AddingSubtask(parent_id);
                app.input_buffer.clear();
            }
        }
        KeyCode::Char('x') => {
            app.toggle_task_completion();
            let _ = app.save_state();
            let _ = app.save_tasks_to_txt();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.move_selection_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.move_selection_down();
        }
        KeyCode::Char('p') => {
            app.pomodoro.toggle();
        }
        KeyCode::Char('r') => {
            app.pomodoro.reset();
        }
        KeyCode::Char('t') => {
            app.cycle_theme();
            let _ = app.save_state();
        }
        KeyCode::Char('w') => {
            let _ = app.save_state();
            let _ = app.save_tasks_to_txt();
            app.show_save_notification();
        }
        KeyCode::Char('c') => {
            let now = Instant::now();
            if let Some(last_time) = app.last_c_key_time {
                if now.duration_since(last_time).as_millis() < 500 {
                    app.input_mode = InputMode::ConfirmingClear;
                    app.last_c_key_time = None;
                } else {
                    app.input_mode = InputMode::ConfirmingDelete;
                    app.last_c_key_time = Some(now);
                }
            } else {
                app.input_mode = InputMode::ConfirmingDelete;
                app.last_c_key_time = Some(now);
            }
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Menu;
            app.menu_selection = 0;
        }
        _ => {
            app.last_c_key_time = None;
        }
    }
    Ok(false)
}

fn handle_input_mode(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Enter => {
            match &app.input_mode {
                InputMode::AddingTask => {
                    if app.add_task(app.input_buffer.clone()) {
                        app.input_mode = InputMode::Normal;
                        app.input_buffer.clear();
                        let _ = app.save_state();
                        let _ = app.save_tasks_to_txt();
                    }
                }
                InputMode::AddingSubtask(parent_id) => {
                    if app.add_subtask(*parent_id, app.input_buffer.clone()) {
                        app.input_mode = InputMode::Normal;
                        app.input_buffer.clear();
                        let _ = app.save_state();
                        let _ = app.save_tasks_to_txt();
                    }
                }
                _ => {}
            }
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
            app.input_buffer.clear();
        }
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                app.input_mode = InputMode::Normal;
                app.input_buffer.clear();
            } else {
                app.input_buffer.push(c);
            }
        }
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        _ => {}
    }
    Ok(false)
}

fn handle_menu_input(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.move_menu_selection_up();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.move_menu_selection_down();
        }
        KeyCode::Enter => {
            let options = App::get_menu_options();
            if app.menu_selection >= options.len() {
                return Ok(false);
            }
            match options[app.menu_selection] {
                "Close Menu" => {
                    app.input_mode = InputMode::Normal;
                }
                "Reset Pomodoro" => {
                    app.pomodoro.reset();
                    app.input_mode = InputMode::Normal;
                }
                "Save Tasks" => {
                    let _ = app.save_state();
                    let _ = app.save_tasks_to_txt();
                    app.show_save_notification();
                    app.input_mode = InputMode::Normal;
                }
                "Clear All Tasks" => {
                    app.input_mode = InputMode::ConfirmingClear;
                }
                "Change Theme" => {
                    app.cycle_theme();
                    app.input_mode = InputMode::Normal;
                    let _ = app.save_state();
                }
                "Quit" => {
                    app.save_state()?;
                    let _ = app.save_tasks_to_txt();
                    return Ok(true);
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_confirmation_input(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            match app.input_mode {
                InputMode::ConfirmingDelete => {
                    app.delete_selected_task();
                    let _ = app.save_state();
                    let _ = app.save_tasks_to_txt();
                }
                InputMode::ConfirmingClear => {
                    app.clear_all_tasks();
                    let _ = app.save_state();
                    let _ = app.save_tasks_to_txt();
                }
                _ => {}
            }
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
    Ok(false)
}

