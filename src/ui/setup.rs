use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::{language_display, App, SetupStep, LANGUAGES};

pub fn draw(frame: &mut Frame, app: &App) {
    let t = &app.theme;
    let area = centered_rect(70, 60, frame.area());
    frame.render_widget(Clear, area);

    let block = t.panel_accent("Welcome to CPOS · Quick Setup");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // step indicator
            Constraint::Min(6),    // body
            Constraint::Length(2), // hints
        ])
        .split(inner);

    frame.render_widget(step_indicator(app), rows[0]);
    draw_body(frame, app, rows[1]);
    frame.render_widget(
        Paragraph::new(hint_line(app)).wrap(Wrap { trim: true }),
        rows[2],
    );
}

fn step_indicator(app: &App) -> Paragraph<'static> {
    let t = &app.theme;
    let steps = [
        (SetupStep::Handle, "1 Handle"),
        (SetupStep::Language, "2 Language"),
        (SetupStep::Template, "3 Template"),
        (SetupStep::Cses, "4 CSES"),
    ];
    let mut spans = vec![Span::raw(" ")];
    for (i, (step, label)) in steps.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  →  ", Style::default().fg(t.border)));
        }
        let style = if *step == app.setup_step {
            Style::default().fg(t.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(t.dim)
        };
        spans.push(Span::styled(label.to_string(), style));
    }
    Paragraph::new(Line::from(spans))
}

fn draw_body(frame: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    match app.setup_step {
        SetupStep::Handle => {
            let lines = vec![
                Line::from(Span::styled(
                    "Enter your Codeforces handle so CPOS can sync your",
                    Style::default().fg(t.fg),
                )),
                Line::from(Span::styled(
                    "solves, rating, and recommendations.",
                    Style::default().fg(t.fg),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  handle  ", Style::default().fg(t.dim)),
                    Span::styled(
                        format!("{}_", app.setup_handle),
                        Style::default().fg(t.accent).add_modifier(Modifier::BOLD),
                    ),
                ]),
            ];
            frame.render_widget(Paragraph::new(lines), area);
        }
        SetupStep::Language => {
            let mut lines = vec![
                Line::from(Span::styled(
                    "Pick your default language for new solution files.",
                    Style::default().fg(t.fg),
                )),
                Line::from(Span::styled(
                    "Use ←/→ to browse — all of these compile and run locally.",
                    Style::default().fg(t.dim),
                )),
                Line::from(""),
            ];
            // Lay the languages out in rows of four, highlighting the choice.
            for chunk in LANGUAGES.chunks(4) {
                let mut spans = vec![Span::raw("  ")];
                for lang in chunk {
                    let label = language_display(lang);
                    if *lang == app.setup_lang {
                        spans.push(Span::styled(
                            format!(" ▸ {label} "),
                            Style::default().fg(t.bg).bg(t.accent).add_modifier(Modifier::BOLD),
                        ));
                    } else {
                        spans.push(Span::styled(format!("   {label} "), Style::default().fg(t.dim)));
                    }
                    spans.push(Span::raw("  "));
                }
                lines.push(Line::from(spans));
            }
            frame.render_widget(Paragraph::new(lines), area);
        }
        SetupStep::Template => {
            let lines = app.setup_template.lines().count();
            let chars = app.setup_template.trim().len();
            let preview: Vec<&str> = app.setup_template.lines().take(3).collect();

            let mut body = vec![
                Line::from(Span::styled(
                    "Paste your solution template now (⌘V / Ctrl+V).",
                    Style::default().fg(t.fg),
                )),
                Line::from(Span::styled(
                    "Leave blank to use the built-in template — you can",
                    Style::default().fg(t.dim),
                )),
                Line::from(Span::styled(
                    "change it anytime in the Config tab.",
                    Style::default().fg(t.dim),
                )),
                Line::from(""),
            ];
            if chars == 0 {
                body.push(Line::from(Span::styled(
                    "  (nothing pasted yet)",
                    Style::default().fg(t.dim),
                )));
            } else {
                body.push(Line::from(Span::styled(
                    format!("  captured {lines} lines, {chars} chars:"),
                    Style::default().fg(t.success),
                )));
                for l in preview {
                    body.push(Line::from(Span::styled(
                        format!("  │ {l}"),
                        Style::default().fg(t.accent_dim),
                    )));
                }
            }
            frame.render_widget(Paragraph::new(body), area);
        }
        SetupStep::Cses => {
            let connected = !app.setup_cses.trim().is_empty();
            let mut body = vec![
                Line::from(Span::styled(
                    "Connect CSES (optional) to sync your solved problems.",
                    Style::default().fg(t.fg),
                )),
                Line::from(Span::styled(
                    "Press o to open the CSES login page, sign in, then copy",
                    Style::default().fg(t.dim),
                )),
                Line::from(Span::styled(
                    "the PHPSESSID cookie value and paste it here (⌘V).",
                    Style::default().fg(t.dim),
                )),
                Line::from(""),
            ];
            if connected {
                body.push(Line::from(vec![
                    Span::styled("  PHPSESSID  ", Style::default().fg(t.dim)),
                    Span::styled(
                        masked(&app.setup_cses),
                        Style::default().fg(t.success).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("  ✓ ready", Style::default().fg(t.success)),
                ]));
            } else {
                body.push(Line::from(Span::styled(
                    "  (not connected — you can also do this later in Config)",
                    Style::default().fg(t.dim),
                )));
            }
            frame.render_widget(Paragraph::new(body), area);
        }
    }
}

fn masked(s: &str) -> String {
    let s = s.trim();
    if s.len() <= 6 {
        "•".repeat(s.len())
    } else {
        format!("{}…{}", &s[..3], &s[s.len() - 3..])
    }
}

fn hint_line(app: &App) -> Line<'static> {
    let t = &app.theme;
    let key = |k: &'static str| Span::styled(k, Style::default().fg(t.accent).add_modifier(Modifier::BOLD));
    let lbl = |l: &'static str| Span::styled(l, Style::default().fg(t.dim));
    match app.setup_step {
        SetupStep::Handle => Line::from(vec![
            Span::raw(" "),
            key("Enter"),
            lbl(" continue   "),
            key("Esc"),
            lbl(" skip setup"),
        ]),
        SetupStep::Language => Line::from(vec![
            Span::raw(" "),
            key("←/→"),
            lbl(" switch   "),
            key("Enter"),
            lbl(" continue   "),
            key("Esc"),
            lbl(" skip"),
        ]),
        SetupStep::Template => Line::from(vec![
            Span::raw(" "),
            key("Enter"),
            lbl(" continue   "),
            key("Backspace"),
            lbl(" clear   "),
            key("Esc"),
            lbl(" skip"),
        ]),
        SetupStep::Cses => Line::from(vec![
            Span::raw(" "),
            key("o"),
            lbl(" open login   "),
            key("Enter"),
            lbl(" finish   "),
            key("Backspace"),
            lbl(" clear"),
        ]),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
