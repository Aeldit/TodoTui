use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, List, Paragraph},
    Frame,
};
use Constraint::{Length, Percentage};

use crate::todo::{Screens, States, Todos};

const BLOCK: Block = Block::bordered().border_type(BorderType::Rounded);
const CENTERED_BLOCK: Block = BLOCK.title_alignment(Alignment::Center);

fn display_main_ui(frame: &mut Frame, states: &mut States, todos: &mut Todos) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100), Length(1)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new(format!(" {}", env!("CARGO_PKG_NAME")))
            .left_aligned()
            .block(CENTERED_BLOCK),
        outer_layout[0],
    );
    frame.render_widget(
        Paragraph::new(format!("v{} ", env!("CARGO_PKG_VERSION")))
            .right_aligned()
            .block(CENTERED_BLOCK),
        outer_layout[0],
    );

    let todos_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(35), Percentage(65)])
        .split(outer_layout[1]);

    frame.render_widget(CENTERED_BLOCK.title("TODOs"), todos_layout[0]);

    let list = List::new(todos.get_todos())
        .block(BLOCK.title(" TODOs "))
        .highlight_style(Style::new().reversed())
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, todos_layout[0], states.get_todo_list());

    let date_done_contents_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100)])
        .split(todos_layout[1]);

    let date_done_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(70), Percentage(30)])
        .split(date_done_contents_layout[0]);

    frame.render_widget(
        Paragraph::new(
            todos
                .get_due_date(states.get_todo_list().selected().unwrap())
                .to_owned(),
        )
        .centered()
        .block(CENTERED_BLOCK.title(" Due Date ")),
        date_done_layout[0],
    );
    frame.render_widget(
        Paragraph::new(
            todos
                .is_done(states.get_todo_list().selected().unwrap())
                .to_owned(),
        )
        .centered()
        .block(CENTERED_BLOCK.title(" Done ")),
        date_done_layout[1],
    );

    frame.render_widget(
        Paragraph::new(
            todos
                .get_description(states.get_todo_list().selected().unwrap())
                .to_owned(),
        )
        .block(BLOCK.title(" Contents ")),
        date_done_contents_layout[1],
    );

    frame.render_widget(
        Paragraph::new("q: quit | t: toggle done | e: edit TODO").centered(),
        outer_layout[2],
    );
}

fn display_create_ui(frame: &mut Frame, states: &mut States) {
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(70)])
        .flex(ratatui::layout::Flex::Center)
        .split(frame.area());
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(4), Percentage(100)])
        .flex(ratatui::layout::Flex::Start)
        .vertical_margin(5)
        .split(horizontal_layout[0]);

    let title_date_done_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(50), Percentage(50)])
        .split(vertical_layout[0]);

    frame.render_widget(
        Paragraph::new(String::from(states.get_title())).block(BLOCK.title(" Title ")),
        title_date_done_layout[0],
    );
    frame.render_widget(
        Paragraph::new(String::from(states.get_date())).block(BLOCK.title(" Due Date ")),
        title_date_done_layout[1],
    );

    frame.render_widget(
        Paragraph::new(String::from(states.get_description())).block(BLOCK.title(" Description ")),
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Percentage(100)])
            .split(vertical_layout[1])[0],
    );
}

pub fn draw(frame: &mut Frame, states: &mut States, todos: &mut Todos) {
    match states.get_screen() {
        Screens::Main => display_main_ui(frame, states, todos),
        Screens::Create => display_create_ui(frame, states),
    }
}

pub fn handle_events(todos: &mut Todos, states: &mut States) -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(false);
        }

        match states.get_screen() {
            Screens::Create => {
                if states.is_in_writting_mode() {
                    if key.code == KeyCode::Esc {
                        states.set_writting_mode(false);
                    } else if key.code == KeyCode::Backspace {
                        states.pop_char();
                    } else if key.code.to_string().eq("Space") {
                        states.add_char(' ');
                    } else if let Some(c) = key.code.to_string().chars().next() {
                        states.add_char(c);
                    }
                } else {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => states.set_screen(Screens::Main),
                        KeyCode::Enter => states.set_writting_mode(true),
                        KeyCode::Tab => states.next_tab(),
                        KeyCode::Char('a') => {
                            todos.add(
                                states.get_title().to_owned(),
                                states.get_description().to_owned(),
                                states.get_date().to_owned(),
                                false,
                            );
                            states.clear_strings();
                            states.set_screen(Screens::Main);
                        }
                        _ => {}
                    }
                }
            }
            Screens::Main => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('a') => states.set_screen(Screens::Create),
                KeyCode::Down => states.get_todo_list().scroll_down_by(1),
                KeyCode::Up => states.get_todo_list().scroll_up_by(1),
                KeyCode::Char('t') => todos.toggle(states.get_todo_list().selected().unwrap()),
                _ => {}
            },
        }
    }
    Ok(false)
}
