use ratatui::prelude::*;

use crate::ui::theme::Theme;

/// Default width for inline progress bars (keeps bars compact — not full column bleed).
pub const BAR_WIDTH: usize = 14;

/// Chunky per-row bar using shade glyphs. Filled portion only — empty is blank so
/// rows never stack into one gray block (unlike background-colored tracks).
pub fn bar_line(width: usize, ratio: f64, fill: Color) -> Line<'static> {
    let width = width.max(1);
    let filled = ((ratio.clamp(0.0, 1.0) * width as f64).round() as usize).min(width);
    let empty = width.saturating_sub(filled);
    Line::from(vec![
        Span::styled("▓".repeat(filled), Style::default().fg(fill)),
        Span::raw(" ".repeat(empty)),
    ])
}

pub fn rate_color(theme: &Theme, pct: u32) -> Color {
    match pct {
        0..=35 => theme.danger,
        36..=65 => theme.warning,
        _ => theme.success,
    }
}
