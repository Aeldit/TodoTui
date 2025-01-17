use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, BorderType, List, Paragraph},
    Frame,
};
use Constraint::{Length, Percentage};

use crate::{
    states::{CreateTab, Screens, States},
    todo::Todos,
};

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

    frame.render_stateful_widget(
        List::new(todos.get_todos_titles())
            .block(BLOCK.title(" TODOs "))
            .highlight_style(Style::new().reversed())
            .repeat_highlight_symbol(true),
        todos_layout[0],
        states.get_todo_list(),
    );

    let date_done_contents_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100)])
        .split(todos_layout[1]);

    let date_done_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(70), Percentage(30)])
        .split(date_done_contents_layout[0]);

    let mut description = String::new();
    let mut due_date = String::new();
    let mut is_done = String::new();
    if let Some(idx) = states.get_todo_list().selected() {
        description.push_str(&todos.get_description(idx));
        due_date.push_str(&todos.get_due_date(idx));
        is_done.push_str(&todos.is_done(idx));
    }
    frame.render_widget(
        Paragraph::new(due_date)
            .centered()
            .block(CENTERED_BLOCK.title(" Due Date ")),
        date_done_layout[0],
    );
    frame.render_widget(
        Paragraph::new(is_done)
            .centered()
            .block(CENTERED_BLOCK.title(" Done ")),
        date_done_layout[1],
    );
    frame.render_widget(
        Paragraph::new(description).block(BLOCK.title(" Contents ")),
        date_done_contents_layout[1],
    );

    frame.render_widget(
        Paragraph::new("q: quit | t: toggle done | e: edit | d: delete").centered(),
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
        Paragraph::new(String::from(states.get_title())).block(
            BLOCK
                .title(" Title ")
                .border_style(Style::default().fg(states.get_fg_color_for_tab(CreateTab::Title))),
        ),
        title_date_done_layout[0],
    );
    frame.render_widget(
        Paragraph::new(String::from(states.get_date())).block(
            BLOCK
                .title(" Due Date ")
                .border_style(Style::default().fg(states.get_fg_color_for_tab(CreateTab::Date))),
        ),
        title_date_done_layout[1],
    );

    frame.render_widget(
        Paragraph::new(String::from(states.get_description())).block(
            BLOCK.title(" Description ").border_style(
                Style::default().fg(states.get_fg_color_for_tab(CreateTab::Description)),
            ),
        ),
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
