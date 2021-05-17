use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::Frame;

use super::action::Action;
use crate::app::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let title = draw_title();
    let body = draw_body(
        app.is_initialized(),
        app.is_loading(),
        app.count_sleep(),
        app.count_tick(),
    );
    let help = draw_help(app.actions());
    let logs = draw_messages(app.messages());

    let size = rect.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        // .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(32)].as_ref())
        .split(chunks[1]);

    rect.render_widget(body, body_chunks[0]);
    rect.render_widget(help, body_chunks[1]);

    rect.render_widget(title, chunks[0]);
    rect.render_widget(logs, chunks[2]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Plop with TUI")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_body<'a>(initalized: bool, loading: bool, sleeps: u32, ticks: u64) -> Paragraph<'a> {
    let initalized_text = if initalized {
        "Initialized"
    } else {
        "Not Initialized !"
    };
    let loading_text = if loading { "Loading..." } else { "" };
    let sleep_text = format!("Sleep count: {}", sleeps);
    let tick_text = format!("Tick count: {}", ticks);
    Paragraph::new(vec![
        Spans::from(Span::raw(initalized_text)),
        Spans::from(Span::raw(loading_text)),
        Spans::from(Span::raw(sleep_text)),
        Spans::from(Span::raw(tick_text)),
    ])
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Left)
    .block(
        Block::default()
            // .title("Body")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .border_type(BorderType::Plain),
    )
}

fn draw_messages<'a>(messages: &[String]) -> Paragraph<'a> {
    let spans: Vec<Spans> = messages
        .iter()
        .map(|it| Spans::from(Span::from(it.clone())))
        .collect();

    Paragraph::new(spans)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .title("Logs")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_help(actions: &[Action]) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.help()
            } else {
                ""
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}
