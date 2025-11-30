# tsk - Pomodoro Timer + Task Manager TUI

A terminal user interface (TUI) application built with Rust and ratatui that combines a real-time clock, Pomodoro timer, and task manager.

## Installation

Install the application as a command-line tool:

```bash
cargo install --path .
```

This will install `tsk` to `~/.cargo/bin/`, which should already be in your PATH if you have Rust installed via rustup.

Run it from anywhere:

```bash
tsk
```

## Features

- **Real-time Clock**: Displays current time at the top
- **Pomodoro Timer**: 25-minute work sessions, 5-minute breaks, 15-minute long breaks after 4 cycles
- **Task Management**: Hierarchical task list with up to 4 levels of nested subtasks
- **Theming**: 11 color themes including Default, Dark, Light, Monochrome, Ocean, Blue Ridge, Dotrb, Everforest, Mars, Tokyo Night, and Vesper
- **Persistent State**: Saves tasks, Pomodoro progress, and theme preference to `~/.config/tui_pomo/state.json`

## Controls

- `a`: Add a new task
- `s`: Add a subtask to the selected task
- `x`: Toggle completion of selected task/subtask
- `↑` / `↓`: Navigate between tasks and subtasks
- `p`: Start/pause Pomodoro timer
- `r`: Reset Pomodoro timer
- `t`: Cycle through themes
- `c`: Delete selected task/subtask
- `cc`: Clear all tasks (press 'c' twice quickly)
- `Esc`: Open settings menu (in normal mode)
- `q`: Quit (saves state automatically)
- `Enter`: Confirm input when adding tasks/subtasks
- `Esc`: Cancel input mode (when adding tasks) or open menu (in normal mode)

## Settings Menu

Press `Esc` in normal mode to open the settings menu. The menu provides quick access to:
- Close Menu
- Reset Pomodoro
- Clear All Tasks
- Change Theme
- Quit

Navigate with `↑`/`↓` or `j`/`k`, select with `Enter`, and close with `Esc` or `q`.

## Project Structure

- `src/main.rs`: Event loop and terminal setup
- `src/app.rs`: Application state and business logic
- `src/ui.rs`: Rendering with ratatui
- `src/input.rs`: Keyboard input handling
- `src/theme.rs`: Theme system with multiple color schemes

## State Persistence

The application automatically saves and loads state from `~/.config/tui_pomo/state.json` on startup and shutdown.

