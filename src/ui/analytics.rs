use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::data::models::Verdict;
use crate::ui::progress;

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(11),
            Constraint::Min(8),
            Constraint::Length(12),
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

    let ratings: Vec<u32> = app
        .rating_history
        .iter()
        .map(|r| r.new_rating)
        .collect();
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let max_r = ratings.iter().copied().max().unwrap_or(1600);
    let min_r = ratings.iter().copied().min().unwrap_or(800);
    let range = (max_r - min_r).max(50);
    let current = *ratings.last().unwrap_or(&0);

    let label = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" {current} "),
            Style::default()
                .fg(app.theme.rating_color(Some(current)))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!("peak {max_r}"), Style::default().fg(t.dim)),
        Span::styled(
            format!("  range {min_r}–{max_r}"),
            Style::default().fg(t.dim),
        ),
    ]));
    frame.render_widget(label, Rect::new(inner.x, inner.y, inner.width, 1));

    let chart = Rect {
        x: inner.x,
        y: inner.y + 1,
        width: inner.width,
        height: inner.height.saturating_sub(2),
    };
    if chart.height < 2 || chart.width < 4 {
        return;
    }

    let baseline_y = chart.y + chart.height - 1;
    for x in chart.x..chart.x + chart.width {
        if let Some(cell) = frame.buffer_mut().cell_mut(Position::new(x, baseline_y)) {
            cell.set_symbol("─").set_fg(t.border);
        }
    }

    let plot_h = (chart.height - 1) as u32;
    let bar_w: u16 = 2;
    let gap: u16 = 1;
    let slot = bar_w + gap;
    let slots = ((chart.width as u16).saturating_sub(bar_w) / slot + 1).max(1) as usize;
    let step = ratings.len().div_ceil(slots).max(1);

    for (i, &rating) in ratings.iter().step_by(step).take(slots).enumerate() {
        let norm = ((rating.saturating_sub(min_r)) as f64 / range as f64).clamp(0.0, 1.0);
        let bar_height = ((norm * plot_h as f64).round() as u16).max(if rating > min_r { 1 } else { 0 });
        let x0 = chart.x + (i as u16 * slot);
        if x0 + bar_w > chart.x + chart.width {
            break;
        }
        let color = app.theme.rating_color(Some(rating));
        for dy in 0..bar_height {
            let y = baseline_y.saturating_sub(dy);
            if y < chart.y {
                break;
            }
            for dx in 0..bar_w {
                if let Some(cell) =
                    frame.buffer_mut().cell_mut(Position::new(x0 + dx, y))
                {
                    cell.set_symbol("█").set_fg(color);
                }
            }
        }
    }

    let axis = Paragraph::new(Line::from(vec![
        Span::styled(format!("{min_r}"), Style::default().fg(t.dim)),
        Span::styled(
            " rating over time ".to_string(),
            Style::default().fg(t.border),
        ),
        Span::styled(format!("{max_r}"), Style::default().fg(t.dim)),
    ]))
    .alignment(Alignment::Center);
    frame.render_widget(
        axis,
        Rect::new(inner.x, inner.y + inner.height - 1, inner.width, 1),
    );
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

    let visible = block.inner(area).height.saturating_sub(1) as usize;

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
            let color = progress::rate_color(t, rate);
            let bar = progress::bar_line(progress::BAR_WIDTH, rate as f64 / 100.0, color);
            let avg = s
                .avg_rating
                .map(|r| format!("{r:.0}"))
                .unwrap_or_else(|| "—".to_string());

            Row::new(vec![
                Cell::from(format!(" {:<18}", truncate(&s.tag, 17)))
                    .style(Style::default().fg(t.fg)),
                Cell::from(format!("{:^8}", format!("{}/{}", s.solved, total)))
                    .style(Style::default().fg(t.dim)),
                Cell::from(format!("{:>5}", format!("{rate}%")))
                    .style(Style::default().fg(color)),
                Cell::from(bar),
                Cell::from(format!("{:>5}", avg)).style(Style::default().fg(t.dim)),
            ])
        })
        .collect();

    let header = Row::new(vec![
        Cell::from(" Topic"),
        Cell::from(" Solved "),
        Cell::from(" Rate "),
        Cell::from(" Progress"),
        Cell::from("   Avg "),
    ])
    .style(t.header_style())
    .bottom_margin(1);

    let widths = [
        Constraint::Length(20),
        Constraint::Length(9),
        Constraint::Length(7),
        Constraint::Length(16),
        Constraint::Length(6),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .block(block);
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

    let legend_h = 1u16;
    let grid_h = inner.height.saturating_sub(legend_h);
    let label_w = 2u16;
    let grid = Rect {
        x: inner.x + label_w,
        y: inner.y,
        width: inner.width.saturating_sub(label_w),
        height: grid_h,
    };

    const DAY_LABELS: [&str; 7] = ["M", " ", "W", " ", "F", " ", "S"];
    for (day, label) in DAY_LABELS.iter().enumerate() {
        if day as u16 >= grid_h {
            break;
        }
        let y = inner.y + day as u16;
        frame.render_widget(
            Paragraph::new(*label).style(Style::default().fg(t.dim)),
            Rect::new(inner.x, y, label_w, 1),
        );
    }

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

    let weeks = 52.min(grid.width as i64);
    for week in 0..weeks {
        let x = grid.x + week as u16;
        for day in 0..7u16 {
            let y = grid.y + day;
            if y >= grid.y + grid.height {
                break;
            }
            let days_ago = (weeks - 1 - week) * 7 + day as i64;
            let count = day_counts.get(&days_ago).copied().unwrap_or(0);
            let color = match count {
                0 => t.border,
                1 => t.accent_dim,
                2..=3 => t.accent,
                4..=6 => t.success,
                _ => t.warning,
            };
            if x < grid.x + grid.width {
                if let Some(cell) = frame.buffer_mut().cell_mut(Position::new(x, y)) {
                    cell.set_symbol(" ").set_bg(if count == 0 { t.bg } else { color });
                }
            }
        }
    }

    let legend_y = inner.y + inner.height - 1;
    let legend = Paragraph::new(Line::from(vec![
        Span::styled(" less ", Style::default().fg(t.dim)),
        Span::styled(" ", Style::default().bg(t.border)),
        Span::raw(" "),
        Span::styled(" ", Style::default().bg(t.accent_dim)),
        Span::raw(" "),
        Span::styled(" ", Style::default().bg(t.accent)),
        Span::raw(" "),
        Span::styled(" ", Style::default().bg(t.success)),
        Span::raw(" "),
        Span::styled(" ", Style::default().bg(t.warning)),
        Span::styled(" more solves", Style::default().fg(t.dim)),
    ]));
    frame.render_widget(legend, Rect::new(inner.x, legend_y, inner.width, 1));
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{truncated}…")
    }
}
