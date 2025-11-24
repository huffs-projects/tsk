mod app;
mod input;
mod theme;
mod ui;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.load_state().unwrap_or_else(|e| {
        eprintln!("Warning: Could not load saved state: {}", e);
    });
    // Save tasks to txt file on startup
    let _ = app.save_tasks_to_txt();

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Ensure pomodoro state matches duration before rendering
        app.pomodoro.sync_state_with_duration();
        
        // Hide notification after 1 second
        if let Some(notif_time) = app.save_notification_time {
            if notif_time.elapsed().as_secs() >= 1 {
                app.save_notification_time = None;
            }
        }
        
        terminal.draw(|f| ui::render(app, f))?;

        if input::handle_input(app)? {
            break;
        }

        app.pomodoro.update();
    }

    Ok(())
}

