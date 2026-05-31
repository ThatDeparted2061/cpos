use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::data::models::Verdict;
use crate::ui::theme::Theme;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(8),
            Constraint::Length(11),
        ])
        .split(area);

    draw_rating_graph(frame, app, chunks[0]);
    draw_tag_breakdown(frame, app, chunks[1]);
    draw_heatmap(frame, app, chunks[2]);
}

fn draw_rating_graph(frame: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let block = t.panel("Rating History");

    if app.rating_history.is_empty() {
        let msg = Paragraph::new("  No rating data yet. Set your Codeforces handle in Config, then press 'r'.")
            .style(Style::default().fg(t.dim))
            .block(block);
        frame.render_widget(msg, area);
        return;
    }

    let ratings: Vec<u64> = app.rating_history.iter().map(|r| r.new_rating as u64).collect();
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let max_r = ratings.iter().copied().max().unwrap_or(1600);
    let min_r = ratings.iter().copied().min().unwrap_or(800);
    let range = (max_r - min_r).max(1);
    let height = inner.height.saturating_sub(1) as u64;
    let width = inner.width as usize;

    let step = if ratings.len() > width { ratings.len() / width } else { 1 };
    let sampled: Vec<u64> = ratings.iter().step_by(step).take(width).copied().collect();

    for (i, &rating) in sampled.iter().enumerate() {
        let bar_height = ((rating - min_r) * height / range).min(height) as u16;
        let x = inner.x + i as u16;
        let color = Theme::rating_color(Some(rating as u32));
        for dy in 0..bar_height {
            let y = inner.y + inner.height - 1 - dy;
            if x < inner.x + inner.width && y >= inner.y {
                if let Some(cell) = frame.buffer_mut().cell_mut(Position::new(x, y)) {
                    cell.set_symbol("▮").set_fg(color);
                }
            }
        }
    }

    let current = ratings.last().copied().unwrap_or(0);
    let peak = max_r;
    let label = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" {current} "),
            Style::default()
                .fg(Theme::rating_color(Some(current as u32)))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!("peak {peak}"), Style::default().fg(t.dim)),
    ]));
    let label_area = Rect::new(inner.x, inner.y, inner.width.min(24), 1);
    frame.render_widget(label, label_area);
}

fn draw_tag_breakdown(frame: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let block = t.panel("Topic Breakdown — weakest first");

    if app.tag_stats.is_empty() {
        let msg = Paragraph::new("  No submission data. Sync to analyze your topic strengths.")
            .style(Style::default().fg(t.dim))
            .block(block);
        frame.render_widget(msg, area);
        return;
    }

    let inner = block.inner(area);
    let visible = inner.height.saturating_sub(1) as usize;

    let rows: Vec<Row> = app
        .tag_stats
        .iter()
        .filter(|s| s.solved + s.attempted > 0)
        .take(visible)
        .map(|s| {
            let total = s.solved + s.attempted;
            let rate = if total > 0 {
                (s.solved as f64 / total as f64 * 100.0) as u32
            } else {
                0
            };
            let bar_w = 18;
            let filled = (rate as usize * bar_w / 100).min(bar_w);
            // Clean solid bar + plain spaces — no dotted track (renders the same
            // in every terminal/font).
            let bar = format!("{}{}", "█".repeat(filled), " ".repeat(bar_w - filled));
            let color = match rate {
                0..=35 => t.danger,
                36..=65 => t.warning,
                _ => t.success,
            };

            Row::new(vec![
                Cell::from(format!(" {:<20}", s.tag)).style(Style::default().fg(t.fg)),
                Cell::from(format!("{}/{}", s.solved, total)).style(Style::default().fg(t.dim)),
                Cell::from(format!("{rate:>3}%")).style(Style::default().fg(color)),
                Cell::from(bar).style(Style::default().fg(color)),
                Cell::from(
                    s.avg_rating
                        .map(|r| format!("{r:.0}"))
                        .unwrap_or_else(|| "—".to_string()),
                )
                .style(Style::default().fg(t.dim)),
            ])
        })
        .collect();

    let header = Row::new(vec![
        Cell::from(" Topic"),
        Cell::from("Solved"),
        Cell::from("Rate"),
        Cell::from("Progress"),
        Cell::from("Avg"),
    ])
    .style(t.header_style());

    let widths = [
        Constraint::Length(22),
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Length(20),
        Constraint::Length(8),
    ];

    let table = Table::new(rows, widths).header(header).block(block);
    frame.render_widget(table, area);
}

fn draw_heatmap(frame: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    let block = t.panel("Activity — last 52 weeks");

    if app.submissions.is_empty() {
        let msg = Paragraph::new("  No activity yet.")
            .style(Style::default().fg(t.dim))
            .block(block);
        frame.render_widget(msg, area);
        return;
    }

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let now = chrono::Utc::now();
    let mut day_counts: std::collections::HashMap<i64, u32> = std::collections::HashMap::new();
    for sub in &app.submissions {
        if sub.verdict == Verdict::Accepted {
            let days_ago = (now - sub.submitted_at).num_days();
            if (0..365).contains(&days_ago) {
                *day_counts.entry(days_ago).or_insert(0) += 1;
            }
        }
    }

    let weeks = 52.min(inner.width as i64);
    for week in 0..weeks {
        let x = inner.x + week as u16;
        for day in 0..7u16 {
            let y = inner.y + day;
            if y >= inner.y + inner.height {
                break;
            }
            let days_ago = (weeks - 1 - week) * 7 + day as i64;
            let count = day_counts.get(&days_ago).copied().unwrap_or(0);
            let (symbol, color) = match count {
                0 => ("·", t.border),
                1 => ("▪", t.accent_dim),
                2..=3 => ("◼", t.accent),
                4..=6 => ("◼", t.success),
                _ => ("◼", t.warning),
            };
            if x < inner.x + inner.width {
                if let Some(cell) = frame.buffer_mut().cell_mut(Position::new(x, y)) {
                    cell.set_symbol(symbol).set_fg(color);
                }
            }
        }
    }
}
