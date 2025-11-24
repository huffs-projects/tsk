use chrono::{DateTime, Duration, Local, Timelike};
use serde::{Deserialize, Serialize};
use crate::theme::{Theme, ThemeName};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    pub subtasks: Vec<Task>,
}

impl Task {
    pub fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
            subtasks: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PomodoroState {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone)]
pub struct PomodoroTimer {
    pub state: PomodoroState,
    pub timer_state: TimerState,
    pub duration: Duration,
    pub remaining: Duration,
    pub cycles: usize,
    pub start_time: Option<DateTime<Local>>,
}

impl PomodoroTimer {
    pub fn new() -> Self {
        Self {
            state: PomodoroState::Work,
            timer_state: TimerState::Stopped,
            duration: Duration::minutes(25),
            remaining: Duration::minutes(25),
            cycles: 0,
            start_time: None,
        }
    }

    pub fn start(&mut self) {
        if self.timer_state == TimerState::Stopped || self.timer_state == TimerState::Paused {
            self.start_time = Some(Local::now());
        }
        self.timer_state = TimerState::Running;
    }

    pub fn pause(&mut self) {
        if self.timer_state == TimerState::Running {
            if let Some(start) = self.start_time {
                let elapsed = Local::now() - start;
                self.remaining -= elapsed;
            }
            self.timer_state = TimerState::Paused;
            self.start_time = None;
        }
    }

    pub fn toggle(&mut self) {
        match self.timer_state {
            TimerState::Stopped | TimerState::Paused => self.start(),
            TimerState::Running => self.pause(),
        }
    }

    pub fn reset(&mut self) {
        // Ensure duration matches the current state (25 min = Work, 5 min = ShortBreak, 15 min = LongBreak)
        self.sync_duration_with_state();
        // Then ensure state matches duration (in case there was a mismatch)
        self.sync_state_with_duration();
        self.timer_state = TimerState::Stopped;
        self.start_time = None;
        self.remaining = self.duration;
    }

    fn sync_duration_with_state(&mut self) {
        match self.state {
            PomodoroState::Work => {
                self.duration = Duration::minutes(25);
            }
            PomodoroState::ShortBreak => {
                self.duration = Duration::minutes(5);
            }
            PomodoroState::LongBreak => {
                self.duration = Duration::minutes(15);
            }
        }
    }

    pub fn sync_state_with_duration(&mut self) {
        let duration_minutes = self.duration.num_minutes();
        match duration_minutes {
            25 => {
                if self.state != PomodoroState::Work {
                    self.state = PomodoroState::Work;
                }
            }
            5 => {
                if self.state != PomodoroState::ShortBreak {
                    self.state = PomodoroState::ShortBreak;
                }
            }
            15 => {
                if self.state != PomodoroState::LongBreak {
                    self.state = PomodoroState::LongBreak;
                }
            }
            _ => {
                // If duration doesn't match any known state, sync duration to state
                self.sync_duration_with_state();
            }
        }
    }

    pub fn update(&mut self) -> bool {
        if self.timer_state != TimerState::Running {
            return false;
        }

        if let Some(start) = self.start_time {
            let elapsed = Local::now() - start;
            if elapsed >= self.remaining {
                self.remaining = Duration::zero();
                self.timer_state = TimerState::Stopped;
                self.advance_cycle();
                return true;
            }
        }
        false
    }

    pub(crate) fn advance_cycle(&mut self) {
        match self.state {
            PomodoroState::Work => {
                self.cycles += 1;
                if self.cycles.is_multiple_of(4) {
                    self.state = PomodoroState::LongBreak;
                    self.duration = Duration::minutes(15);
                } else {
                    self.state = PomodoroState::ShortBreak;
                    self.duration = Duration::minutes(5);
                }
            }
            PomodoroState::ShortBreak | PomodoroState::LongBreak => {
                self.state = PomodoroState::Work;
                self.duration = Duration::minutes(25);
            }
        }
        self.remaining = self.duration;
        self.start_time = None;
    }

    pub fn get_remaining_seconds(&self) -> i64 {
        if self.timer_state == TimerState::Running {
            if let Some(start) = self.start_time {
                let elapsed = Local::now() - start;
                let remaining = self.remaining - elapsed;
                remaining.num_seconds().max(0)
            } else {
                self.remaining.num_seconds()
            }
        } else {
            self.remaining.num_seconds()
        }
    }

    pub fn get_progress(&self) -> f64 {
        let total_seconds = self.duration.num_seconds();
        if total_seconds == 0 {
            return 0.0;
        }
        
        let remaining_seconds = self.get_remaining_seconds();
        let elapsed_seconds = total_seconds - remaining_seconds;
        (elapsed_seconds as f64 / total_seconds as f64).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    AddingTask,
    AddingSubtask(usize),
    Menu,
    ConfirmingDelete,
    ConfirmingClear,
}

#[derive(Debug, Clone)]
pub struct App {
    pub tasks: Vec<Task>,
    pub selected_index: usize,
    pub selected_path: Vec<usize>,
    pub pomodoro: PomodoroTimer,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub next_task_id: usize,
    pub theme: Theme,
    pub theme_name: ThemeName,
    pub last_c_key_time: Option<std::time::Instant>,
    pub menu_selection: usize,
    pub save_notification_time: Option<std::time::Instant>,
}

impl App {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            selected_index: 0,
            selected_path: Vec::new(),
            pomodoro: PomodoroTimer::new(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            next_task_id: 1,
            theme: Theme::default(),
            theme_name: ThemeName::Default,
            last_c_key_time: None,
            menu_selection: 0,
            save_notification_time: None,
        }
    }

    pub fn get_menu_options() -> Vec<&'static str> {
        vec![
            "Close Menu",
            "Reset Pomodoro",
            "Save Tasks",
            "Clear All Tasks",
            "Change Theme",
            "Quit",
        ]
    }

    pub fn move_menu_selection_up(&mut self) {
        let options = Self::get_menu_options();
        if self.menu_selection > 0 {
            self.menu_selection -= 1;
        } else {
            self.menu_selection = options.len() - 1;
        }
    }

    pub fn move_menu_selection_down(&mut self) {
        let options = Self::get_menu_options();
        if self.menu_selection < options.len() - 1 {
            self.menu_selection += 1;
        } else {
            self.menu_selection = 0;
        }
    }

    pub fn set_theme(&mut self, theme_name: ThemeName) {
        self.theme_name = theme_name;
        self.theme = Theme::from_name(theme_name);
    }

    pub fn cycle_theme(&mut self) {
        let next_theme = match self.theme_name {
            ThemeName::Default => ThemeName::Dark,
            ThemeName::Dark => ThemeName::Light,
            ThemeName::Light => ThemeName::Monochrome,
            ThemeName::Monochrome => ThemeName::Ocean,
            ThemeName::Ocean => ThemeName::BlueRidge,
            ThemeName::BlueRidge => ThemeName::Dotrb,
            ThemeName::Dotrb => ThemeName::Everforest,
            ThemeName::Everforest => ThemeName::Mars,
            ThemeName::Mars => ThemeName::TokyoNight,
            ThemeName::TokyoNight => ThemeName::Vesper,
            ThemeName::Vesper => ThemeName::Default,
        };
        self.set_theme(next_theme);
    }

    pub fn get_current_time(&self) -> String {
        let now = Local::now();
        let hour24 = now.hour();
        let hour12 = if hour24 == 0 {
            12
        } else if hour24 > 12 {
            hour24 - 12
        } else {
            hour24
        };
        let am_pm = if hour24 >= 12 { "PM" } else { "AM" };
        let time_str = format!("{:02}:{:02} {}", hour12, now.minute(), am_pm);
        let date_str = now.format("%a, %b %d, %Y").to_string();
        format!("{}  {}", date_str, time_str)
    }

    pub fn add_task(&mut self, title: String) -> bool {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return false;
        }
        let limited_title = if trimmed.len() > 200 {
            &trimmed[..200]
        } else {
            trimmed
        };
        let task = Task::new(self.next_task_id, limited_title.to_string());
        self.next_task_id += 1;
        self.tasks.push(task);
        self.validate_selected_index();
        true
    }

    fn get_task_mut_at_path(&mut self, path: &[usize]) -> Option<&mut Task> {
        if path.is_empty() {
            if self.selected_index >= self.tasks.len() {
                return None;
            }
            return self.tasks.get_mut(self.selected_index);
        }
        if self.selected_index >= self.tasks.len() {
            return None;
        }
        let mut task = self.tasks.get_mut(self.selected_index)?;
        for &idx in path {
            if idx >= task.subtasks.len() {
                return None;
            }
            task = task.subtasks.get_mut(idx)?;
        }
        Some(task)
    }

    fn get_task_at_path(&self, path: &[usize]) -> Option<&Task> {
        if path.is_empty() {
            if self.selected_index >= self.tasks.len() {
                return None;
            }
            return self.tasks.get(self.selected_index);
        }
        if self.selected_index >= self.tasks.len() {
            return None;
        }
        let mut task = self.tasks.get(self.selected_index)?;
        for &idx in path {
            if idx >= task.subtasks.len() {
                return None;
            }
            task = task.subtasks.get(idx)?;
        }
        Some(task)
    }

    fn get_parent_path(&self) -> Option<Vec<usize>> {
        if self.selected_path.is_empty() {
            None
        } else {
            Some(self.selected_path[..self.selected_path.len() - 1].to_vec())
        }
    }

    pub fn add_subtask(&mut self, _parent_id: usize, title: String) -> bool {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return false;
        }
        let path_len = self.selected_path.len();
        if path_len >= 4 {
            return false; // Maximum depth reached
        }
        let limited_title = if trimmed.len() > 200 {
            &trimmed[..200]
        } else {
            trimmed
        };
        let path = self.selected_path.clone();
        let new_id = self.next_task_id;
        self.next_task_id += 1;
        if let Some(task) = self.get_task_mut_at_path(&path) {
            let subtask = Task::new(new_id, limited_title.to_string());
            task.subtasks.push(subtask);
            true
        } else {
            false
        }
    }

    fn toggle_completion_recursive(task: &mut Task, new_state: bool) {
        task.completed = new_state;
        for subtask in &mut task.subtasks {
            Self::toggle_completion_recursive(subtask, new_state);
        }
    }

    pub fn toggle_task_completion(&mut self) {
        let path = self.selected_path.clone();
        if let Some(task) = self.get_task_mut_at_path(&path) {
            let new_state = !task.completed;
            Self::toggle_completion_recursive(task, new_state);
        }
    }

    fn count_all_items(task: &Task) -> usize {
        1 + task.subtasks.iter().map(Self::count_all_items).sum::<usize>()
    }

    fn get_flat_index(&self, task_idx: usize, path: &[usize]) -> usize {
        let mut flat_idx = 0;
        for i in 0..task_idx {
            flat_idx += Self::count_all_items(&self.tasks[i]);
        }
        if let Some(task) = self.tasks.get(task_idx) {
            let mut current = task;
            for &p in path {
                for i in 0..p {
                    flat_idx += Self::count_all_items(&current.subtasks[i]);
                }
                flat_idx += 1;
                current = &current.subtasks[p];
            }
        }
        flat_idx
    }

    fn find_item_at_flat_index(&self, target_flat: &mut usize) -> Option<(usize, Vec<usize>)> {
        for (task_idx, task) in self.tasks.iter().enumerate() {
            if *target_flat == 0 {
                return Some((task_idx, Vec::new()));
            }
            *target_flat -= 1;
            if let Some((path, _remaining)) = Self::find_in_subtasks(task, &mut *target_flat, Vec::new()) {
                return Some((task_idx, path));
            }
        }
        None
    }

    fn find_in_subtasks(task: &Task, target_flat: &mut usize, mut path: Vec<usize>) -> Option<(Vec<usize>, usize)> {
        for (idx, subtask) in task.subtasks.iter().enumerate() {
            if *target_flat == 0 {
                path.push(idx);
                return Some((path, *target_flat));
            }
            *target_flat -= 1;
            let mut new_path = path.clone();
            new_path.push(idx);
            if let Some((found_path, _remaining)) = Self::find_in_subtasks(subtask, target_flat, new_path) {
                return Some((found_path, *target_flat));
            }
        }
        None
    }

    pub fn move_selection_up(&mut self) {
        let current_flat = self.get_flat_index(self.selected_index, &self.selected_path);
        if current_flat > 0 {
            let mut new_flat = current_flat - 1;
            if let Some((new_task_idx, new_path)) = self.find_item_at_flat_index(&mut new_flat) {
                self.selected_index = new_task_idx;
                self.selected_path = new_path;
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        let current_flat = self.get_flat_index(self.selected_index, &self.selected_path);
        let total_items: usize = self.tasks.iter().map(Self::count_all_items).sum();
        if current_flat < total_items - 1 {
            let mut new_flat = current_flat + 1;
            if let Some((new_task_idx, new_path)) = self.find_item_at_flat_index(&mut new_flat) {
                self.selected_index = new_task_idx;
                self.selected_path = new_path;
            }
        }
    }

    pub fn get_selected_parent_id(&self) -> Option<usize> {
        self.get_task_at_path(&self.selected_path).map(|t| t.id)
    }

    pub fn delete_selected_task(&mut self) {
        if self.selected_path.is_empty() {
            if !self.tasks.is_empty() {
                self.tasks.remove(self.selected_index);
                self.validate_selected_index();
            }
        } else {
            let parent_path = self.get_parent_path().unwrap_or_default();
            let idx_to_remove = match self.selected_path.last() {
                Some(&idx) => idx,
                None => return,
            };
            if let Some(parent) = self.get_task_mut_at_path(&parent_path) {
                if idx_to_remove < parent.subtasks.len() {
                    parent.subtasks.remove(idx_to_remove);
                    if idx_to_remove >= parent.subtasks.len() && !parent.subtasks.is_empty() {
                        let new_idx = parent.subtasks.len() - 1;
                        self.selected_path.pop();
                        self.selected_path.push(new_idx);
                    } else if parent.subtasks.is_empty() {
                        self.selected_path = parent_path;
                    } else {
                        self.selected_path.pop();
                    }
                }
            } else {
                self.selected_path.clear();
                self.validate_selected_index();
            }
        }
    }

    pub fn clear_all_tasks(&mut self) {
        self.tasks.clear();
        self.selected_index = 0;
        self.selected_path.clear();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SavedState {
    tasks: Vec<Task>,
    pomodoro_cycles: usize,
    pomodoro_state: String,
    pomodoro_timer_state: String,
    pomodoro_remaining_seconds: i64,
    next_task_id: usize,
    theme: Option<String>,
}

impl App {
    pub fn save_state(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("tui_pomo");
        std::fs::create_dir_all(&config_dir)?;

        let state = SavedState {
            tasks: self.tasks.clone(),
            pomodoro_cycles: self.pomodoro.cycles,
            pomodoro_state: format!("{:?}", self.pomodoro.state),
            pomodoro_timer_state: format!("{:?}", self.pomodoro.timer_state),
            pomodoro_remaining_seconds: self.pomodoro.get_remaining_seconds(),
            next_task_id: self.next_task_id,
            theme: Some(format!("{:?}", self.theme_name)),
        };

        let json = serde_json::to_string_pretty(&state)?;
        std::fs::write(config_dir.join("state.json"), json)?;
        Ok(())
    }

    pub fn show_save_notification(&mut self) {
        self.save_notification_time = Some(std::time::Instant::now());
    }

    pub fn save_tasks_to_txt(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory")?
            .join("tui_pomo");
        std::fs::create_dir_all(&config_dir)?;

        let mut output = String::new();
        
        fn format_task(task: &Task, indent: usize, output: &mut String) {
            let prefix = "  ".repeat(indent);
            let checkbox = if task.completed { "[x]" } else { "[ ]" };
            output.push_str(&format!("{}{} {}\n", prefix, checkbox, task.title));
            
            for subtask in &task.subtasks {
                format_task(subtask, indent + 1, output);
            }
        }

        if self.tasks.is_empty() {
            output.push_str("No tasks yet.\n");
        } else {
            for task in &self.tasks {
                format_task(task, 0, &mut output);
            }
        }

        std::fs::write(config_dir.join("tasks.txt"), output.clone())?;
        
        // Also save a copy to ~/browserpage/ as todo.txt
        if let Ok(home) = std::env::var("HOME") {
            let browserpage_dir = std::path::Path::new(&home).join("browserpage");
            match std::fs::create_dir_all(&browserpage_dir) {
                Ok(_) => {
                    let todo_path = browserpage_dir.join("todo.txt");
                    if let Err(e) = std::fs::write(&todo_path, output) {
                        eprintln!("Warning: Could not write to {}: {}", todo_path.display(), e);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Could not create browserpage directory: {}", e);
                }
            }
        } else {
            eprintln!("Warning: HOME environment variable not set");
        }
        
        Ok(())
    }

    pub fn load_state(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Could not find config directory. Please ensure your system has a valid config directory.")?
            .join("tui_pomo");
        let state_file = config_dir.join("state.json");

        if !state_file.exists() {
            return Ok(()); // No saved state is not an error
        }

        let json = std::fs::read_to_string(&state_file)
            .map_err(|e| format!("Failed to read state file: {}. Error: {}", state_file.display(), e))?;
        
        let state: SavedState = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse state file (invalid JSON): {}. Error: {}", state_file.display(), e))?;

        // Restore tasks
        self.tasks = state.tasks;
        
        // Restore Pomodoro state
        self.pomodoro.cycles = state.pomodoro_cycles;
        self.pomodoro.state = match state.pomodoro_state.as_str() {
            "Work" => PomodoroState::Work,
            "ShortBreak" => PomodoroState::ShortBreak,
            "LongBreak" => PomodoroState::LongBreak,
            _ => {
                eprintln!("Warning: Invalid Pomodoro state '{}', defaulting to Work", state.pomodoro_state);
                PomodoroState::Work
            },
        };
        
        // Restore timer state
        self.pomodoro.timer_state = match state.pomodoro_timer_state.as_str() {
            "Stopped" => TimerState::Stopped,
            "Running" => TimerState::Running,
            "Paused" => TimerState::Paused,
            _ => {
                eprintln!("Warning: Invalid timer state '{}', defaulting to Stopped", state.pomodoro_timer_state);
                TimerState::Stopped
            },
        };
        
        // Restore remaining time
        if state.pomodoro_remaining_seconds >= 0 {
            self.pomodoro.remaining = Duration::seconds(state.pomodoro_remaining_seconds);
        } else {
            eprintln!("Warning: Invalid remaining time '{}', resetting timer", state.pomodoro_remaining_seconds);
            self.pomodoro.reset();
        }
        
        // Infer duration and state from remaining time if it matches a known duration
        // This fixes cases where state and remaining time don't match
        let remaining_minutes = self.pomodoro.remaining.num_minutes();
        match remaining_minutes {
            25 => {
                self.pomodoro.duration = Duration::minutes(25);
                self.pomodoro.state = PomodoroState::Work;
            }
            5 => {
                self.pomodoro.duration = Duration::minutes(5);
                self.pomodoro.state = PomodoroState::ShortBreak;
            }
            15 => {
                self.pomodoro.duration = Duration::minutes(15);
                self.pomodoro.state = PomodoroState::LongBreak;
            }
            _ => {
                // If remaining doesn't match a known duration, sync based on current state
                self.pomodoro.sync_duration_with_state();
                // Then ensure state matches duration (in case there was a mismatch)
                self.pomodoro.sync_state_with_duration();
            }
        }
        
        // Restore next_task_id with validation
        if state.next_task_id == 0 {
            eprintln!("Warning: Invalid next_task_id '{}', defaulting to 1", state.next_task_id);
            self.next_task_id = 1.max(self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1);
        } else {
            self.next_task_id = state.next_task_id;
        }

        // Restore theme
        if let Some(theme_str) = state.theme {
            let theme_name = match theme_str.as_str() {
                "Default" => ThemeName::Default,
                "Dark" => ThemeName::Dark,
                "Light" => ThemeName::Light,
                "Monochrome" => ThemeName::Monochrome,
                "Ocean" => ThemeName::Ocean,
                "BlueRidge" => ThemeName::BlueRidge,
                "Dotrb" => ThemeName::Dotrb,
                "Everforest" => ThemeName::Everforest,
                "Mars" => ThemeName::Mars,
                "TokyoNight" => ThemeName::TokyoNight,
                "Vesper" => ThemeName::Vesper,
                _ => {
                    eprintln!("Warning: Invalid theme '{}', defaulting to Default", theme_str);
                    ThemeName::Default
                },
            };
            self.set_theme(theme_name);
        }

        // Validate and fix selected_index
        self.validate_selected_index();

        Ok(())
    }
    
    fn validate_selected_index(&mut self) {
        if self.tasks.is_empty() {
            self.selected_index = 0;
            self.selected_path.clear();
        } else if self.selected_index >= self.tasks.len() {
            self.selected_index = self.tasks.len() - 1;
            self.selected_path.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut app = App::new();
        assert!(app.add_task("Test Task".to_string()));
        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].title, "Test Task");
    }

    #[test]
    fn test_add_task_empty() {
        let mut app = App::new();
        assert!(!app.add_task("   ".to_string()));
        assert_eq!(app.tasks.len(), 0);
    }

    #[test]
    fn test_add_task_long_title() {
        let mut app = App::new();
        let long_title = "a".repeat(300);
        assert!(app.add_task(long_title.clone()));
        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].title.len(), 200);
    }

    #[test]
    fn test_toggle_task_completion() {
        let mut app = App::new();
        app.add_task("Test Task".to_string());
        assert!(!app.tasks[0].completed);
        
        app.toggle_task_completion();
        assert!(app.tasks[0].completed);
        
        app.toggle_task_completion();
        assert!(!app.tasks[0].completed);
    }

    #[test]
    fn test_toggle_task_completion_with_subtasks() {
        let mut app = App::new();
        app.add_task("Parent Task".to_string());
        app.selected_index = 0;
        app.selected_path = vec![];
        app.add_subtask(1, "Subtask".to_string());
        
        // Toggle parent task
        app.selected_index = 0;
        app.selected_path = vec![];
        app.toggle_task_completion();
        
        assert!(app.tasks[0].completed);
        assert!(app.tasks[0].subtasks[0].completed);
    }

    #[test]
    fn test_delete_task() {
        let mut app = App::new();
        app.add_task("Task 1".to_string());
        app.add_task("Task 2".to_string());
        app.selected_index = 0;
        
        app.delete_selected_task();
        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].title, "Task 2");
    }

    #[test]
    fn test_clear_all_tasks() {
        let mut app = App::new();
        app.add_task("Task 1".to_string());
        app.add_task("Task 2".to_string());
        
        app.clear_all_tasks();
        assert_eq!(app.tasks.len(), 0);
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_add_subtask_max_depth() {
        let mut app = App::new();
        app.add_task("Level 0".to_string());
        app.selected_path = vec![0];
        app.add_subtask(1, "Level 1".to_string());
        app.selected_path = vec![0, 0];
        app.add_subtask(2, "Level 2".to_string());
        app.selected_path = vec![0, 0, 0];
        app.add_subtask(3, "Level 3".to_string());
        app.selected_path = vec![0, 0, 0, 0];
        
        // Should fail at level 4
        assert!(!app.add_subtask(4, "Level 4".to_string()));
    }

    #[test]
    fn test_pomodoro_timer_reset() {
        let mut timer = PomodoroTimer::new();
        timer.remaining = Duration::minutes(10);
        timer.reset();
        assert_eq!(timer.remaining, Duration::minutes(25));
        assert_eq!(timer.timer_state, TimerState::Stopped);
    }

    #[test]
    fn test_pomodoro_timer_advance_cycle() {
        let mut timer = PomodoroTimer::new();
        timer.cycles = 2;
        timer.state = PomodoroState::Work;
        timer.advance_cycle();
        assert_eq!(timer.state, PomodoroState::ShortBreak);
        assert_eq!(timer.cycles, 3);
        
        timer.advance_cycle();
        assert_eq!(timer.state, PomodoroState::Work);
    }

    #[test]
    fn test_pomodoro_timer_long_break() {
        let mut timer = PomodoroTimer::new();
        timer.cycles = 3;
        timer.state = PomodoroState::Work;
        timer.advance_cycle();
        assert_eq!(timer.state, PomodoroState::LongBreak);
        assert_eq!(timer.cycles, 4);
        assert_eq!(timer.duration, Duration::minutes(15));
    }

    #[test]
    fn test_validate_selected_index_empty() {
        let mut app = App::new();
        app.selected_index = 5;
        app.validate_selected_index();
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn test_validate_selected_index_out_of_bounds() {
        let mut app = App::new();
        app.add_task("Task 1".to_string());
        app.selected_index = 5;
        app.validate_selected_index();
        assert_eq!(app.selected_index, 0);
    }
}
