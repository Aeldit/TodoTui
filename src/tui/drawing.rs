use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{block::Title, Block, BorderType, List, Paragraph, Wrap},
    Frame,
};
use Constraint::{Length, Percentage};

use crate::{
    states::{
        CreateTab, Screens, States, ALL_KEY_EDIT, MAX_DATE_LEN, MAX_DESCRIPTION_LEN, MAX_TITLE_LEN,
    },
    todo::Todos,
};

const BLOCK: Block = Block::bordered().border_type(BorderType::Rounded);
const CENTERED_BLOCK: Block = BLOCK.title_alignment(Alignment::Center);

const BG_COLOR: Color = Color::Magenta;
const ACTIVE_COLOR: Color = Color::Red;
const TEXT_STYLE: Style = Style::new().fg(ACTIVE_COLOR);

fn display_main_ui(frame: &mut Frame, states: &mut States, todos: &mut Todos) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Length(3), Percentage(100), Length(1)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new(format!(" {}", env!("CARGO_PKG_NAME")))
            .left_aligned()
            .block(CENTERED_BLOCK)
            .fg(BG_COLOR),
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

    frame.render_stateful_widget(
        List::new(todos.get_todos_titles())
            .block(BLOCK.title(" TODOs ").fg(BG_COLOR))
            .style(TEXT_STYLE)
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
            .style(TEXT_STYLE)
            .block(CENTERED_BLOCK.title(" Due Date ").fg(BG_COLOR)),
        date_done_layout[0],
    );
    frame.render_widget(
        Paragraph::new(is_done)
            .centered()
            .style(TEXT_STYLE)
            .block(CENTERED_BLOCK.title(" Done ").fg(BG_COLOR)),
        date_done_layout[1],
    );
    frame.render_widget(
        Paragraph::new(description)
            .style(TEXT_STYLE)
            .wrap(Wrap { trim: true })
            .block(BLOCK.title(" Contents ").fg(BG_COLOR)),
        date_done_contents_layout[1],
    );

    frame.render_widget(
        Paragraph::new("q: quit | t: toggle done | e: edit | d: delete")
            .centered()
            .fg(BG_COLOR),
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
        .constraints(vec![Length(4), Percentage(100), Length(1), Length(1)])
        .flex(ratatui::layout::Flex::Start)
        .vertical_margin(5)
        .split(horizontal_layout[0]);

    let title_date_done_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage(50), Percentage(50)])
        .split(vertical_layout[0]);

    frame.render_widget(
        Paragraph::new(String::from(states.get_title()))
            .wrap(Wrap { trim: true })
            .block(
                BLOCK
                    .title(" Title ")
                    .title(Title::from(format!(
                        " {}/{} ",
                        states.get_nb_char_in_tab(CreateTab::Title),
                        MAX_TITLE_LEN,
                    )))
                    .style(TEXT_STYLE)
                    .border_style(
                        Style::default().fg(states.get_fg_color_for_tab(CreateTab::Title)),
                    ),
            ),
        title_date_done_layout[0],
    );
    frame.render_widget(
        Paragraph::new(String::from(states.get_date()))
            .wrap(Wrap { trim: true })
            .block(
                BLOCK
                    .title(" Due Date ")
                    .title(Title::from(format!(
                        " {}/{} ",
                        states.get_nb_char_in_tab(CreateTab::Date),
                        MAX_DATE_LEN,
                    )))
                    .style(TEXT_STYLE)
                    .border_style(
                        Style::default().fg(states.get_fg_color_for_tab(CreateTab::Date)),
                    ),
            ),
        title_date_done_layout[1],
    );

    frame.render_widget(
        Paragraph::new(String::from(states.get_description()))
            .wrap(Wrap { trim: true })
            .block(
                BLOCK
                    .title(" Description ")
                    .title(Title::from(format!(
                        " {}/{} ",
                        states.get_nb_char_in_tab(CreateTab::Description),
                        MAX_DESCRIPTION_LEN,
                    )))
                    .border_style(
                        Style::default().fg(states.get_fg_color_for_tab(CreateTab::Description)),
                    ),
            )
            .style(TEXT_STYLE),
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Percentage(100)])
            .split(vertical_layout[1])[0],
    );

    frame.render_widget(
        Paragraph::new(match states.is_in_writting_mode() {
            true => String::from("Esc: exit writting mode"),
            false => format!(
                "q/Esc: quit | Tab: cycle tab | {}: edit | a: add the TODO",
                ALL_KEY_EDIT
            ),
        })
        .centered()
        .fg(BG_COLOR),
        vertical_layout[2],
    );
    /*frame.render_widget(
        Paragraph::new(match states.is_in_writting_mode() {
            true => String::from("Esc: exit writting mode"),
            false => format!("{}: paste | {}: copy", ALL_KEY_PASTE, ALL_KEY_COPY),
        })
        .centered()
        .fg(BG_COLOR),
        vertical_layout[3],
    );*/
}

pub fn draw(frame: &mut Frame, states: &mut States, todos: &mut Todos) {
    match states.get_screen() {
        Screens::Main => display_main_ui(frame, states, todos),
        Screens::Create => display_create_ui(frame, states),
    }
}
