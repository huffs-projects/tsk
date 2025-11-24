use ratatui::style::Color;
use serde::{Deserialize, Serialize};

pub struct ColorPalette;

impl ColorPalette {
    pub fn get_color(num: u8) -> Color {
        match num {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::White,
            8 => Color::DarkGray,
            9 => Color::LightRed,
            10 => Color::LightGreen,
            11 => Color::LightYellow,
            12 => Color::LightBlue,
            13 => Color::LightMagenta,
            14 => Color::LightCyan,
            15 => Color::Reset,
            16 => Color::Rgb(126, 200, 200), // Blue Ridge cyan
            17 => Color::Rgb(126, 166, 124), // Blue Ridge green
            18 => Color::Rgb(107, 140, 174), // Blue Ridge blue
            19 => Color::Rgb(155, 138, 160), // Blue Ridge magenta
            20 => Color::Rgb(212, 175, 55),  // Blue Ridge gold
            21 => Color::Rgb(196, 181, 160), // Blue Ridge beige
            22 => Color::Rgb(74, 85, 104),   // Blue Ridge dark gray
            23 => Color::Rgb(244, 228, 188), // Blue Ridge light beige
            24 => Color::Rgb(181, 139, 173), // Dotrb purple
            25 => Color::Rgb(143, 150, 104), // Dotrb green
            26 => Color::Rgb(139, 123, 155), // Dotrb blue
            27 => Color::Rgb(204, 107, 141), // Dotrb magenta
            28 => Color::Rgb(212, 163, 115), // Dotrb tan
            29 => Color::Rgb(221, 204, 204), // Dotrb light pink
            30 => Color::Rgb(90, 74, 74),   // Dotrb dark
            31 => Color::Rgb(244, 195, 147), // Dotrb peach
            32 => Color::Rgb(131, 192, 146), // Everforest green
            33 => Color::Rgb(167, 192, 128), // Everforest light green
            34 => Color::Rgb(127, 187, 179), // Everforest teal
            35 => Color::Rgb(214, 153, 182), // Everforest pink
            36 => Color::Rgb(219, 188, 127), // Everforest yellow
            37 => Color::Rgb(211, 198, 170), // Everforest beige
            38 => Color::Rgb(75, 86, 92),   // Everforest dark
            39 => Color::Rgb(181, 165, 165), // Mars gray
            40 => Color::Rgb(139, 144, 100), // Mars green
            41 => Color::Rgb(139, 123, 123), // Mars blue-gray
            42 => Color::Rgb(201, 149, 144), // Mars pink
            43 => Color::Rgb(221, 197, 181), // Mars beige
            44 => Color::Rgb(77, 46, 46),   // Mars dark
            45 => Color::Rgb(125, 207, 255), // Tokyo Night cyan
            46 => Color::Rgb(158, 206, 106), // Tokyo Night green
            47 => Color::Rgb(122, 162, 247), // Tokyo Night blue
            48 => Color::Rgb(187, 154, 247), // Tokyo Night purple
            49 => Color::Rgb(224, 175, 104), // Tokyo Night gold
            50 => Color::Rgb(169, 177, 214), // Tokyo Night light blue
            51 => Color::Rgb(86, 95, 137),   // Tokyo Night dark
            52 => Color::Rgb(140, 255, 255), // Vesper cyan
            53 => Color::Rgb(160, 249, 160), // Vesper green
            54 => Color::Rgb(140, 175, 255), // Vesper blue
            55 => Color::Rgb(255, 136, 255), // Vesper magenta
            56 => Color::Rgb(255, 255, 138), // Vesper yellow
            57 => Color::Rgb(197, 197, 197), // Vesper gray
            58 => Color::Rgb(86, 86, 86),   // Vesper dark
            _ => Color::Reset,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeName {
    Default,
    Dark,
    Light,
    Monochrome,
    Ocean,
    BlueRidge,
    Dotrb,
    Everforest,
    Mars,
    TokyoNight,
    Vesper,
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub clock: u8,
    pub pomodoro_work: u8,
    pub pomodoro_short_break: u8,
    pub pomodoro_long_break: u8,
    pub task_selected: u8,
    pub task_normal: u8,
    pub task_completed: u8,
    pub input_prompt: u8,
    pub secondary: u8,
}

impl Theme {
    pub fn get_clock(&self) -> Color {
        ColorPalette::get_color(self.clock)
    }

    pub fn get_pomodoro_work(&self) -> Color {
        ColorPalette::get_color(self.pomodoro_work)
    }

    pub fn get_pomodoro_short_break(&self) -> Color {
        ColorPalette::get_color(self.pomodoro_short_break)
    }

    pub fn get_pomodoro_long_break(&self) -> Color {
        ColorPalette::get_color(self.pomodoro_long_break)
    }

    pub fn get_task_selected(&self) -> Color {
        ColorPalette::get_color(self.task_selected)
    }

    pub fn get_task_normal(&self) -> Color {
        ColorPalette::get_color(self.task_normal)
    }

    pub fn get_task_completed(&self) -> Color {
        ColorPalette::get_color(self.task_completed)
    }

    pub fn get_input_prompt(&self) -> Color {
        ColorPalette::get_color(self.input_prompt)
    }

    pub fn get_secondary(&self) -> Color {
        ColorPalette::get_color(self.secondary)
    }
}

impl Theme {
    pub fn default() -> Self {
        Self {
            clock: 6,      // Cyan
            pomodoro_work: 2,  // Green
            pomodoro_short_break: 4,  // Blue
            pomodoro_long_break: 5,   // Magenta
            task_selected: 3,  // Yellow
            task_normal: 7,    // White
            task_completed: 8,  // DarkGray
            input_prompt: 6,   // Cyan
            secondary: 11,     // LightYellow
        }
    }

    pub fn dark() -> Self {
        Self {
            clock: 14,    // LightCyan
            pomodoro_work: 10,  // LightGreen
            pomodoro_short_break: 12,  // LightBlue
            pomodoro_long_break: 13,   // LightMagenta
            task_selected: 3,  // Yellow
            task_normal: 7,    // White
            task_completed: 8,  // DarkGray
            input_prompt: 14,   // LightCyan
            secondary: 3,      // Yellow
        }
    }

    pub fn light() -> Self {
        Self {
            clock: 4,     // Blue
            pomodoro_work: 2,  // Green
            pomodoro_short_break: 6,  // Cyan
            pomodoro_long_break: 5,   // Magenta
            task_selected: 1,  // Red
            task_normal: 0,    // Black
            task_completed: 8,  // DarkGray
            input_prompt: 4,   // Blue
            secondary: 8,     // DarkGray
        }
    }

    pub fn monochrome() -> Self {
        Self {
            clock: 7,     // White
            pomodoro_work: 7,  // White
            pomodoro_short_break: 8,  // DarkGray
            pomodoro_long_break: 7,  // White
            task_selected: 7,  // White
            task_normal: 7,    // White
            task_completed: 8,  // DarkGray
            input_prompt: 7,   // White
            secondary: 8,     // DarkGray
        }
    }

    pub fn ocean() -> Self {
        Self {
            clock: 6,     // Cyan
            pomodoro_work: 2,  // Green
            pomodoro_short_break: 4,  // Blue
            pomodoro_long_break: 12,  // LightBlue
            task_selected: 14,  // LightCyan
            task_normal: 6,    // Cyan
            task_completed: 8,  // DarkGray
            input_prompt: 12,  // LightBlue
            secondary: 14,     // LightCyan
        }
    }

    pub fn blue_ridge() -> Self {
        Self {
            clock: 16,    // Blue Ridge cyan
            pomodoro_work: 17,  // Blue Ridge green
            pomodoro_short_break: 18,  // Blue Ridge blue
            pomodoro_long_break: 19,   // Blue Ridge magenta
            task_selected: 20,  // Blue Ridge gold
            task_normal: 21,    // Blue Ridge beige
            task_completed: 22,  // Blue Ridge dark gray
            input_prompt: 16,   // Blue Ridge cyan
            secondary: 23,      // Blue Ridge light beige
        }
    }

    pub fn dotrb() -> Self {
        Self {
            clock: 24,    // Dotrb purple
            pomodoro_work: 25,  // Dotrb green
            pomodoro_short_break: 26,  // Dotrb blue
            pomodoro_long_break: 27,   // Dotrb magenta
            task_selected: 28,  // Dotrb tan
            task_normal: 29,    // Dotrb light pink
            task_completed: 30,  // Dotrb dark
            input_prompt: 24,   // Dotrb purple
            secondary: 31,      // Dotrb peach
        }
    }

    pub fn everforest() -> Self {
        Self {
            clock: 32,    // Everforest green
            pomodoro_work: 33,  // Everforest light green
            pomodoro_short_break: 34,  // Everforest teal
            pomodoro_long_break: 35,   // Everforest pink
            task_selected: 36,  // Everforest yellow
            task_normal: 37,    // Everforest beige
            task_completed: 38,  // Everforest dark
            input_prompt: 32,   // Everforest green
            secondary: 36,      // Everforest yellow
        }
    }

    pub fn mars() -> Self {
        Self {
            clock: 39,    // Mars gray
            pomodoro_work: 40,  // Mars green
            pomodoro_short_break: 41,  // Mars blue-gray
            pomodoro_long_break: 42,   // Mars pink
            task_selected: 28,  // Mars tan (reuse Dotrb tan)
            task_normal: 43,    // Mars beige
            task_completed: 44,  // Mars dark
            input_prompt: 39,   // Mars gray
            secondary: 31,     // Mars peach (reuse Dotrb peach)
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            clock: 45,    // Tokyo Night cyan
            pomodoro_work: 46,  // Tokyo Night green
            pomodoro_short_break: 47,  // Tokyo Night blue
            pomodoro_long_break: 48,   // Tokyo Night purple
            task_selected: 49,  // Tokyo Night gold
            task_normal: 50,    // Tokyo Night light blue
            task_completed: 51,  // Tokyo Night dark
            input_prompt: 45,   // Tokyo Night cyan
            secondary: 49,      // Tokyo Night gold
        }
    }

    pub fn vesper() -> Self {
        Self {
            clock: 52,    // Vesper cyan
            pomodoro_work: 53,  // Vesper green
            pomodoro_short_break: 54,  // Vesper blue
            pomodoro_long_break: 55,   // Vesper magenta
            task_selected: 56,  // Vesper yellow
            task_normal: 57,    // Vesper gray
            task_completed: 58,  // Vesper dark
            input_prompt: 52,   // Vesper cyan
            secondary: 56,      // Vesper yellow
        }
    }

    pub fn from_name(name: ThemeName) -> Self {
        match name {
            ThemeName::Default => Self::default(),
            ThemeName::Dark => Self::dark(),
            ThemeName::Light => Self::light(),
            ThemeName::Monochrome => Self::monochrome(),
            ThemeName::Ocean => Self::ocean(),
            ThemeName::BlueRidge => Self::blue_ridge(),
            ThemeName::Dotrb => Self::dotrb(),
            ThemeName::Everforest => Self::everforest(),
            ThemeName::Mars => Self::mars(),
            ThemeName::TokyoNight => Self::tokyo_night(),
            ThemeName::Vesper => Self::vesper(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default()
    }
}

