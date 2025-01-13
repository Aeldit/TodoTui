use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::size;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};

use crate::todo::{Todo, Todos};

pub fn draw(frame: &mut Frame) {
    use Constraint::Length;

    let term_size = match size() {
        Ok(term_size) => term_size,
        Err(_) => return,
    };

    let vertical = Layout::vertical([Length(term_size.0)]);
    let [title_area] = vertical.areas(frame.area());

    frame.render_widget(
        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("TODO List"),
        title_area,
    );
}

pub fn handle_events(todos: &mut Todos) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            if let KeyCode::Char('q') = key.code {
                return Ok(true);
            } else if let KeyCode::Char('a') = key.code {
                todos.add(Todo {
                    title: String::new(),
                    contents: String::new(),
                    due_date: String::new(),
                    done: false,
                });
                return Ok(true);
            }
        }
        _ => {}
    }
    Ok(false)
}
