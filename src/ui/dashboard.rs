use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::ui::theme::Theme;

const C: [&str; 5] = [" ████ ", "█     ", "█     ", "█     ", " ████ "];
const P: [&str; 5] = ["█████ ", "█    █", "█████ ", "█     ", "█     "];
const O: [&str; 5] = [" ████ ", "█    █", "█    █", "█    █", " ████ "];
const S: [&str; 5] = [" █████", "█     ", " ████ ", "     █", "█████ "];

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(6),
            Constraint::Min(6),
        ])
        .split(area);

    draw_banner(frame, app, chunks[0]);
    draw_stat_cards(frame, app, chunks[1]);
    draw_lower(frame, app, chunks[2]);
}

fn draw_banner(frame: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for r in 0..5 {
        let row = format!("{} {} {} {}", C[r], P[r], O[r], S[r]);
        let color = if r < 2 {
            t.accent
        } else if r < 4 {
            t.accent
        } else {
            t.accent_dim
        };
        lines.push(
            Line::from(Span::styled(
                row,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ))
            .alignment(Alignment::Center),
        );
    }

    let cf_handle = app.config.handles.get("codeforces").cloned().unwrap_or_default();
    let (cf_label, cf_color) = if cf_handle.is_empty() {
        ("not set".to_string(), t.dim)
    } else {
        (cf_handle, t.success)
    };
    let (cses_label, cses_color) = match app.cses_solved.len() {
        n if n > 0 => (format!("connected ✓  ({n} solved)"), t.success),
        _ if !app.cses_attempted.is_empty() => {
            (format!("connected ({}) attempted", app.cses_attempted.len()), t.warning)
        }
        _ if app.config.cses_session.is_some() => {
            ("cookie set — visit problemset list in browser".to_string(), t.warning)
        }
        _ => (
            "log in on cses.fi + visit problemset list".to_string(),
            t.dim,
        ),
    };
    lines.push(
        Line::from(vec![
            Span::styled("Codeforces ", Style::default().fg(t.dim)),
            Span::styled(cf_label, Style::default().fg(cf_color).add_modifier(Modifier::BOLD)),
            Span::styled("     CSES ", Style::default().fg(t.dim)),
            Span::styled(cses_label, Style::default().fg(cses_color).add_modifier(Modifier::BOLD)),
        ])
        .alignment(Alignment::Center),
    );

    let capture_label = if let Some(port) = app.capture_port {
        format!("listening on :{port}")
    } else {
        "off".to_string()
    };
    let capture_color = if app.capture_port.is_some() { t.success } else { t.dim };
    lines.push(
        Line::from(vec![
            Span::styled("Browser companion ", Style::default().fg(t.dim)),
            Span::styled(capture_label, Style::default().fg(capture_color).add_modifier(Modifier::BOLD)),
            Span::styled("  — run ", Style::default().fg(t.dim)),
            Span::styled("cpos setup-browser", Style::default().fg(t.accent_dim).add_modifier(Modifier::BOLD)),
            Span::styled(" to install", Style::default().fg(t.dim)),
        ])
        .alignment(Alignment::Center),
    );

    let root = crate::engine::workspace::root(&app.config);
    lines.push(
        Line::from(vec![
            Span::styled("your solutions live in  ", Style::default().fg(t.dim)),
            Span::styled(
                root.display().to_string(),
                Style::default().fg(t.accent_dim).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  — open this folder in your editor", Style::default().fg(t.dim)),
        ])
        .alignment(Alignment::Center),
    );

    let banner = Paragraph::new(lines).style(Style::default().bg(t.bg));
    frame.render_widget(banner, area);
}

fn draw_stat_cards(frame: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
            Constraint::Ratio(1, 4),
        ])
        .split(area);

    let rating = app
        .current_rating()
        .map(|r| r.to_string())
        .unwrap_or_else(|| "—".to_string());
    let rating_color = Theme::rating_color(app.current_rating());

    stat_card(frame, app, cols[0], "PROBLEMS", &app.problems.len().to_string(), app.theme.accent);
    stat_card(
        frame,
        app,
        cols[1],
        "SOLVED",
        &app.solved_count().to_string(),
        app.theme.success,
    );
    stat_card(frame, app, cols[2], "RATING", &rating, rating_color);
    stat_card(
        frame,
        app,
        cols[3],
        "STREAK",
        &format!("{}d", app.current_streak()),
        app.theme.warning,
    );
}

fn stat_card(frame: &mut Frame, app: &App, area: Rect, label: &str, value: &str, color: Color) {
    let t = &app.theme;
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(t.border))
        .style(Style::default().bg(t.bg));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let para = Paragraph::new(vec![
        Line::from(Span::styled(
            value.to_string(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(label, Style::default().fg(t.dim))).alignment(Alignment::Center),
    ]);
    frame.render_widget(para, inner);
}

fn draw_lower(frame: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    draw_weak_tags(frame, app, cols[0]);
    draw_up_next(frame, app, cols[1]);
}

fn draw_weak_tags(frame: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let block = t.panel("Weakest Topics");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let weak: Vec<_> = app
        .tag_stats
        .iter()
        .filter(|s| s.solved + s.attempted >= 2)
        .take(inner.height as usize)
        .collect();

    if weak.is_empty() {
        let hint = Paragraph::new("  Sync your account to see where to focus.")
            .style(Style::default().fg(t.dim))
            .wrap(Wrap { trim: true });
        frame.render_widget(hint, inner);
        return;
    }

    let lines: Vec<Line> = weak
        .iter()
        .map(|s| {
            let total = s.solved + s.attempted;
            let rate = if total > 0 {
                (s.solved as f64 / total as f64 * 100.0) as u32
            } else {
                0
            };
            let bar_w = 12usize;
            let filled = (rate as usize * bar_w / 100).min(bar_w);
            let color = match rate {
                0..=35 => t.danger,
                36..=65 => t.warning,
                _ => t.success,
            };
            Line::from(vec![
                Span::styled(format!(" {:<16}", truncate(&s.tag, 16)), Style::default().fg(t.fg)),
                Span::styled("█".repeat(filled), Style::default().fg(color)),
                Span::styled(" ".repeat(bar_w - filled), Style::default()),
                Span::styled(format!(" {rate:>3}%"), Style::default().fg(color)),
            ])
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), inner);
}

fn draw_up_next(frame: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let block = t.panel("Recommended Next");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.recommendations.is_empty() {
        let hint = Paragraph::new("  Sync to get problems picked for your level.")
            .style(Style::default().fg(t.dim))
            .wrap(Wrap { trim: true });
        frame.render_widget(hint, inner);
        return;
    }

    let lines: Vec<Line> = app
        .recommendations
        .iter()
        .take(inner.height as usize)
        .map(|rec| {
            let p = &rec.problem;
            Line::from(vec![
                Span::styled(
                    format!(" {:>6} ", p.display_id()),
                    Style::default()
                        .fg(Theme::rating_color(p.rating))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(truncate(&p.name, 26), Style::default().fg(t.fg)),
                Span::styled(
                    format!("  {}", p.difficulty_label()),
                    Style::default().fg(t.dim),
                ),
            ])
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), inner);
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{truncated}…")
    }
}
