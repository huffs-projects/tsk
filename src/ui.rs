use crate::app::{App, InputMode, PomodoroState, Task};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn render(app: &App, f: &mut Frame) {
    if app.input_mode == InputMode::Menu {
        render_menu(app, f);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    render_clock(app, f, chunks[0]);
    render_pomodoro(app, f, chunks[1]);
    render_tasks(app, f, chunks[2]);
    render_input_prompt(app, f, chunks[3]);
    
    // Render version in bottom right corner
    render_version(f);
    
    // Render save notification if active
    if app.save_notification_time.is_some() {
        render_save_notification(app, f);
    }
}

fn render_clock(app: &App, f: &mut Frame, area: Rect) {
    let time = app.get_current_time();
    let clock = Paragraph::new(time)
        .block(Block::default().borders(Borders::ALL).title("Clock"))
        .style(Style::default().fg(app.theme.get_clock()))
        .alignment(Alignment::Center);
    f.render_widget(clock, area);
}

fn render_pomodoro(app: &App, f: &mut Frame, area: Rect) {
    let timer = &app.pomodoro;
    let remaining_seconds = timer.get_remaining_seconds();
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    let state_text = match timer.state {
        PomodoroState::Work => "Work",
        PomodoroState::ShortBreak => "Short Break",
        PomodoroState::LongBreak => "Long Break",
    };

    let timer_text = format!("{:02}:{:02}", minutes, seconds);
    let status_text = match timer.timer_state {
        crate::app::TimerState::Running => "Running",
        crate::app::TimerState::Paused => "Paused",
        crate::app::TimerState::Stopped => "Stopped",
    };

    let cycles_text = format!("Cycles: {}", timer.cycles);
    let progress = timer.get_progress();

    let progress_color = match timer.state {
        PomodoroState::Work => app.theme.get_pomodoro_work(),
        PomodoroState::ShortBreak => app.theme.get_pomodoro_short_break(),
        PomodoroState::LongBreak => app.theme.get_pomodoro_long_break(),
    };

    let progress_label = format!("{:.0}%", progress * 100.0);
    let title_left = format!("Pomodoro Timer ({}) | {} | {}", cycles_text, timer_text, progress_label);
    let title_right = format!("{} | {}", state_text, status_text);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(title_left).alignment(Alignment::Left))
        .title(Title::from(title_right).alignment(Alignment::Right))
        .border_style(Style::default().fg(progress_color));
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let text_color = app.theme.get_secondary();
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(text_color))
        .percent((progress * 100.0) as u16)
        .label("");
    f.render_widget(gauge, inner_area);
}

fn is_path_selected(task_idx: usize, path: &[usize], selected_idx: usize, selected_path: &[usize]) -> bool {
    task_idx == selected_idx && path == selected_path
}

struct TaskRenderContext {
    task_idx: usize,
    path: Vec<usize>,
    level: usize,
    selected_idx: usize,
    selected_path: Vec<usize>,
    theme: crate::theme::Theme,
}

fn render_task_recursive(
    task: &Task,
    ctx: &TaskRenderContext,
    items: &mut Vec<ListItem>,
) {
    let indent = "  ".repeat(ctx.level);
    let is_selected = is_path_selected(ctx.task_idx, &ctx.path, ctx.selected_idx, &ctx.selected_path);
    let prefix = if task.completed { "[x]" } else { "[ ]" };
    let style = if is_selected {
        Style::default()
            .fg(ctx.theme.get_task_selected())
            .add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else if task.completed {
        Style::default().fg(ctx.theme.get_task_completed())
    } else {
        Style::default().fg(ctx.theme.get_task_normal())
    };

    let text = vec![
        Span::styled(format!("{}{}", indent, prefix), style),
        Span::raw(" "),
        Span::styled(task.title.clone(), style),
    ];

    items.push(ListItem::new(Line::from(text)));

    if ctx.level < 4 {
        for (sub_idx, subtask) in task.subtasks.iter().enumerate() {
            let mut new_path = ctx.path.clone();
            new_path.push(sub_idx);
            let new_ctx = TaskRenderContext {
                task_idx: ctx.task_idx,
                path: new_path,
                level: ctx.level + 1,
                selected_idx: ctx.selected_idx,
                selected_path: ctx.selected_path.clone(),
                theme: ctx.theme,
            };
            render_task_recursive(subtask, &new_ctx, items);
        }
    }
}

fn render_tasks(app: &App, f: &mut Frame, area: Rect) {
    let mut items = Vec::new();

    for (idx, task) in app.tasks.iter().enumerate() {
        let ctx = TaskRenderContext {
            task_idx: idx,
            path: Vec::new(),
            level: 0,
            selected_idx: app.selected_index,
            selected_path: app.selected_path.clone(),
            theme: app.theme,
        };
        render_task_recursive(task, &ctx, &mut items);
    }

    if items.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "No tasks yet. Press 'a' to add a task.",
            Style::default().fg(app.theme.get_task_completed()),
        ))));
    }

    let tasks_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Tasks"));

    f.render_widget(tasks_list, area);
}

fn render_input_prompt(app: &App, f: &mut Frame, area: Rect) {
    let prompt_text = match &app.input_mode {
        InputMode::Normal => {
            "Commands: a=add task, s=add subtask, x=toggle, ↑↓/jk=navigate, p=play/pause, r=reset, t=theme, w=save, c=delete, cc=clear all, Esc=menu, q=quit"
        }
        InputMode::AddingTask => "Enter task name (Enter to confirm, Esc to cancel):",
        InputMode::AddingSubtask(_) => "Enter subtask name (Enter to confirm, Esc to cancel):",
        InputMode::Menu => "↑↓/jk=navigate, Enter=select, Esc/q=close",
        InputMode::ConfirmingDelete => "Delete selected task/subtask? (y/n):",
        InputMode::ConfirmingClear => "Clear all tasks? (y/n):",
    };

    let input_display = match &app.input_mode {
        InputMode::Normal => String::new(),
        InputMode::ConfirmingDelete | InputMode::ConfirmingClear => String::new(),
        _ => app.input_buffer.clone(),
    };

    let content = match &app.input_mode {
        InputMode::Normal => prompt_text.to_string(),
        InputMode::ConfirmingDelete | InputMode::ConfirmingClear => prompt_text.to_string(),
        _ => format!("{} {}", prompt_text, input_display),
    };

    let prompt = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .style(Style::default().fg(app.theme.get_input_prompt()))
        .wrap(Wrap { trim: true });

    f.render_widget(prompt, area);

    if app.input_mode != InputMode::Normal && app.input_mode != InputMode::ConfirmingDelete && app.input_mode != InputMode::ConfirmingClear {
        let cursor_pos = prompt_text.len() + 1 + app.input_buffer.len();
        f.set_cursor(
            area.x + (cursor_pos as u16 % area.width) + 1,
            area.y + 1 + (cursor_pos as u16 / area.width),
        );
    }
}

fn render_menu(app: &App, f: &mut Frame) {
    let options = App::get_menu_options();
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(idx, option)| {
            let style = if idx == app.menu_selection {
                Style::default()
                    .fg(app.theme.get_task_selected())
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(app.theme.get_task_normal())
            };
            ListItem::new(Line::from(Span::styled(*option, style)))
        })
        .collect();

    let menu_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Settings Menu")
        );

    let area = centered_rect(40, options.len() as u16 + 2, f.size());
    f.render_widget(menu_list, area);
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Length(height),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_save_notification(app: &App, f: &mut Frame) {
    let area = centered_rect(30, 3, f.size());
    let notification = Paragraph::new("Saved")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(app.theme.get_secondary()).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(notification, area);
}

fn render_version(f: &mut Frame) {
    let version = env!("CARGO_PKG_VERSION");
    let version_text = format!("v{}", version);
    let area = f.size();
    
    // Position in bottom right corner
    let version_area = Rect {
        x: area.x + area.width.saturating_sub(version_text.len() as u16 + 1),
        y: area.y + area.height.saturating_sub(1),
        width: (version_text.len() + 1) as u16,
        height: 1,
    };
    
    let version_para = Paragraph::new(version_text)
        .style(Style::default().fg(ratatui::style::Color::DarkGray));
    f.render_widget(version_para, version_area);
}

