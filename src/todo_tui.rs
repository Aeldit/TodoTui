use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, List, Paragraph},
    Frame,
};

use crate::todo::{States, Todos};

pub fn draw(frame: &mut Frame, states: &mut States, todos: &mut Todos) {
    use Constraint::{Length, Percentage};

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new(format!(" {}", env!("CARGO_PKG_NAME")))
            .left_aligned()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_alignment(Alignment::Center),
            ),
        outer_layout[0],
    );
    frame.render_widget(
        Paragraph::new(format!("v{} ", env!("CARGO_PKG_VERSION")))
            .right_aligned()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_alignment(Alignment::Center),
            ),
        outer_layout[0],
    );

    let todos_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(35), Percentage(65)])
        .split(outer_layout[1]);

    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("TODOs"),
        todos_layout[0],
    );

    let list = List::new(todos.get_todos())
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" TODOs ")
                .title_alignment(Alignment::Center),
        )
        .highlight_style(Style::new().reversed())
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, todos_layout[0], states.get_todo_list());

    let date_contents_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100)])
        .split(todos_layout[1]);

    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Due Date ")
            .title_alignment(Alignment::Center),
        date_contents_layout[0],
    );

    let p = Paragraph::new(
        todos
            .get_contents(states.get_todo_list().selected().unwrap())
            .to_owned(),
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Contents ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded),
    );
    frame.render_widget(p, date_contents_layout[1]);
}

pub fn handle_events(todos: &mut Todos, states: &mut States) -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('a') => todos.add(String::new(), String::new(), String::new(), false),
                KeyCode::Down => states.get_todo_list().scroll_down_by(1),
                KeyCode::Up => states.get_todo_list().scroll_up_by(1),
                _ => {}
            }
        }
    }
    Ok(false)
}
