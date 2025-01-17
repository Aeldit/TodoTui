use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    states::{Screens, States},
    todo::Todos,
};

fn handle_create_ui_events(todos: &mut Todos, states: &mut States, key: KeyEvent) {
    if states.is_in_writting_mode() {
        if key.code == KeyCode::Esc {
            states.set_writting_mode(false);
        } else if key.code == KeyCode::Backspace {
            states.pop_char();
        } else {
            match key.code.to_string().as_str() {
                "Space" => states.add_char(' '),
                "Tab" => states.add_char('\t'),
                _ => {
                    if let Some(c) = key.code.to_string().chars().next() {
                        states.add_char(c);
                    }
                }
            }
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

fn handle_main_ui_events(
    todos: &mut Todos,
    states: &mut States,
    key: KeyEvent,
) -> std::io::Result<bool> {
    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char('a') => states.set_screen(Screens::Create),
        KeyCode::Down => states.get_todo_list().scroll_down_by(1),
        KeyCode::Up => states.get_todo_list().scroll_up_by(1),
        KeyCode::Char('t') => todos.toggle(states.get_todo_list().selected().unwrap()),
        KeyCode::Char('d') => {}
        _ => {}
    }
    Ok(false)
}

pub fn handle_events(todos: &mut Todos, states: &mut States) -> std::io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(false);
        }

        match states.get_screen() {
            Screens::Create => handle_create_ui_events(todos, states, key),
            Screens::Main => return handle_main_ui_events(todos, states, key),
        }
    }
    Ok(false)
}
