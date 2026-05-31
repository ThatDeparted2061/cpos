use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, BorderType, Borders, Padding};

/// A color theme for the whole UI. Only the accent family changes between
/// presets; the background, foreground and semantic colors stay constant so
/// the app always reads cleanly.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub dim: Color,
    pub border: Color,
    pub accent: Color,
    pub accent_dim: Color,
    pub highlight_bg: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::from_name("purple")
    }
}

impl Theme {
    pub const NAMES: [&'static str; 5] = ["purple", "cyan", "green", "amber", "mono"];

    pub fn from_name(name: &str) -> Theme {
        let base = Theme {
            bg: Color::Rgb(13, 13, 20),
            fg: Color::Rgb(214, 214, 224),
            dim: Color::Rgb(108, 108, 132),
            border: Color::Rgb(58, 58, 82),
            accent: Color::Rgb(180, 142, 255),
            accent_dim: Color::Rgb(124, 92, 191),
            highlight_bg: Color::Rgb(38, 32, 64),
            success: Color::Rgb(126, 231, 135),
            warning: Color::Rgb(227, 179, 65),
            danger: Color::Rgb(247, 118, 142),
        };

        match name {
            "cyan" => Theme {
                accent: Color::Rgb(86, 182, 194),
                accent_dim: Color::Rgb(58, 130, 140),
                highlight_bg: Color::Rgb(20, 46, 52),
                ..base
            },
            "green" => Theme {
                accent: Color::Rgb(126, 231, 135),
                accent_dim: Color::Rgb(82, 160, 92),
                highlight_bg: Color::Rgb(22, 46, 28),
                ..base
            },
            "amber" => Theme {
                accent: Color::Rgb(240, 180, 80),
                accent_dim: Color::Rgb(170, 122, 48),
                highlight_bg: Color::Rgb(48, 38, 16),
                ..base
            },
            "mono" => Theme {
                accent: Color::Rgb(200, 200, 214),
                accent_dim: Color::Rgb(130, 130, 150),
                highlight_bg: Color::Rgb(40, 40, 50),
                ..base
            },
            _ => base,
        }
    }

    pub fn next_name(current: &str) -> &'static str {
        let idx = Self::NAMES.iter().position(|n| *n == current).unwrap_or(0);
        Self::NAMES[(idx + 1) % Self::NAMES.len()]
    }

    /// A rounded panel with a dim border, an accented title, and a little inner
    /// breathing room so content never sits flush against the border.
    pub fn panel(&self, title: &str) -> Block<'static> {
        Block::default()
            .title(Span::styled(
                format!(" {title} "),
                Style::default()
                    .fg(self.accent)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.border))
            .style(Style::default().bg(self.bg))
            .padding(Padding::horizontal(1))
    }

    /// A panel whose border is accented (used for focused / important panels).
    pub fn panel_accent(&self, title: &str) -> Block<'static> {
        self.panel(title)
            .border_style(Style::default().fg(self.accent_dim))
    }

    pub fn selection(&self) -> Style {
        Style::default()
            .bg(self.highlight_bg)
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn title_style(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    pub fn dim_style(&self) -> Style {
        Style::default().fg(self.dim)
    }

    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.accent_dim)
            .add_modifier(Modifier::BOLD)
    }

    /// Codeforces-style rating colors. These are intentionally NOT themed:
    /// they carry meaning that competitive programmers already recognize.
    pub fn rating_color(rating: Option<u32>) -> Color {
        match rating {
            Some(r) if r >= 2400 => Color::Rgb(255, 76, 76),
            Some(r) if r >= 2100 => Color::Rgb(255, 140, 60),
            Some(r) if r >= 1900 => Color::Rgb(195, 130, 240),
            Some(r) if r >= 1600 => Color::Rgb(110, 150, 255),
            Some(r) if r >= 1400 => Color::Rgb(80, 200, 215),
            Some(r) if r >= 1200 => Color::Rgb(126, 211, 135),
            Some(_) => Color::Rgb(160, 160, 170),
            None => Color::Rgb(120, 120, 132),
        }
    }
}
