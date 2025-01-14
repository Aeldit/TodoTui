use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::todo::Todos;

pub fn draw(frame: &mut Frame) {
    use Constraint::{Length, Percentage};

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100)])
        .split(frame.area());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(40), Percentage(60)])
        .split(outer_layout[1]);

    frame.render_widget(
        Paragraph::new("Tabs").block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("TODO List")
                .title_alignment(ratatui::layout::Alignment::Center),
        ),
        outer_layout[0],
    );
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("TODOs"),
        inner_layout[0],
    );
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Contents"),
        inner_layout[1],
    );
}

pub fn handle_events(todos: &mut Todos) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            if let KeyCode::Char('q') = key.code {
                return Ok(true);
            } else if let KeyCode::Char('a') = key.code {
                todos.add(String::new(), String::new(), String::new(), false);
            }
        }
        _ => {}
    }
    Ok(false)
}
