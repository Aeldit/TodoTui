use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, BorderType, Borders, List, Paragraph},
    Frame,
};
use std::rc::Rc;
use Constraint::{Length, Percentage};

use crate::todo::{Screens, States, Todos};

fn display_bar_get_outer_layout(frame: &mut Frame) -> Rc<[Rect]> {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100), Length(1)])
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
    outer_layout
}

fn display_bar_get_todo_layout(
    frame: &mut Frame,
    outer_layout: Rc<[Rect]>,
    todos: &mut Todos,
    states: &mut States,
) -> Rc<[Rect]> {
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

    todos_layout
}

fn display_todo_contents(
    frame: &mut Frame,
    todos_layout: Rc<[Rect]>,
    states: &mut States,
    todos: &mut Todos,
) {
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
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" Due Date ")
                .title_alignment(Alignment::Center),
        ),
        date_done_layout[0],
    );
    frame.render_widget(
        Paragraph::new(
            todos
                .is_done(states.get_todo_list().selected().unwrap())
                .to_owned(),
        )
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .title(" Done ")
                .title_alignment(Alignment::Center),
        ),
        date_done_layout[1],
    );

    frame.render_widget(
        Paragraph::new(
            todos
                .get_description(states.get_todo_list().selected().unwrap())
                .to_owned(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Contents ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        ),
        date_done_contents_layout[1],
    );
}

fn display_footer(frame: &mut Frame, outer_layout: Rc<[Rect]>) {
    frame.render_widget(
        Paragraph::new("t - toggle done | d - edit due date | c - edit description").centered(),
        outer_layout[2],
    );
}

fn display_create_ui(frame: &mut Frame) {
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(50)])
        .flex(ratatui::layout::Flex::Center)
        .split(frame.area());
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Percentage(25)])
        .flex(ratatui::layout::Flex::Start)
        .vertical_margin(6)
        .split(horizontal_layout[0]);

    frame.render_widget(
        Paragraph::new("Add TODO").block(Block::bordered().border_type(BorderType::Rounded)),
        vertical_layout[0],
    );
}

pub fn draw(frame: &mut Frame, states: &mut States, todos: &mut Todos) {
    match states.get_screen() {
        Screens::Create => display_create_ui(frame),
        Screens::Main => {
            let outer_layout = display_bar_get_outer_layout(frame);
            let todos_layout =
                display_bar_get_todo_layout(frame, outer_layout.clone(), todos, states);
            display_todo_contents(frame, todos_layout, states, todos);
            display_footer(frame, outer_layout);
        }
    }
}

pub fn handle_events(todos: &mut Todos, states: &mut States) -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('q') => match states.get_screen() {
                    Screens::Main => return Ok(true),
                    Screens::Create => states.set_screen(Screens::Main),
                },
                KeyCode::Char('a') => states.set_screen(Screens::Create),
                KeyCode::Down => states.get_todo_list().scroll_down_by(1),
                KeyCode::Up => states.get_todo_list().scroll_up_by(1),
                KeyCode::Char('t') => todos.toggle(states.get_todo_list().selected().unwrap()),
                KeyCode::Esc => states.set_screen(Screens::Main),
                _ => {}
            }
        }
    }
    Ok(false)
}
