use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph},
    Frame,
};

use crate::todo::Todos;

pub fn draw(frame: &mut Frame, todos_state: &mut ListState, todos: &mut Todos) {
    use Constraint::{Length, Percentage};

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100)])
        .split(frame.area());

    let titles_contents_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(35), Percentage(65)])
        .split(outer_layout[1]);

    frame.render_widget(
        Paragraph::new("Tabs").block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("TODO List")
                .title_alignment(Alignment::Center),
        ),
        outer_layout[0],
    );
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("TODOs"),
        titles_contents_layout[0],
    );
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Contents"),
        titles_contents_layout[1],
    );

    let list = List::new(todos.get_todos())
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title("TODOs"),
        )
        .highlight_style(Style::new().reversed())
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, titles_contents_layout[0], todos_state);

    let p = Paragraph::new(
        todos
            .get_contents(todos_state.selected().unwrap())
            .to_owned(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Title")
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(p, titles_contents_layout[1]);
}

pub fn handle_events(todos: &mut Todos, todos_state: &mut ListState) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            if let KeyCode::Char('q') = key.code {
                return Ok(true);
            } else if let KeyCode::Char('a') = key.code {
                todos.add(String::new(), String::new(), String::new(), false);
            } else if let KeyCode::Down = key.code {
                todos_state.scroll_down_by(1);
            } else if let KeyCode::Up = key.code {
                todos_state.scroll_up_by(1);
            }
        }
        _ => {}
    }
    Ok(false)
}
