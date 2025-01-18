use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::{
    states::{Screens, States, ALL_KEY_EDIT},
    todo::Todos,
};

fn handle_create_ui_events(todos: &mut Todos, states: &mut States, key: KeyEvent, edit: bool) {
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
            KeyCode::Char(ALL_KEY_EDIT) => states.set_writting_mode(true),
            KeyCode::Tab => states.next_tab(),
            KeyCode::Char('a') => {
                if edit {
                    if let Some(idx) = states.get_todo_list().selected() {
                        todos.edit(idx, states);
                    }
                } else {
                    todos.add(
                        states.get_title().to_owned(),
                        states.get_description().to_owned(),
                        states.get_date().to_owned(),
                        false,
                    );
                }
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
        KeyCode::Down => states.scroll_down(todos),
        KeyCode::Up => states.scroll_up(),
        KeyCode::Char('t') => todos.toggle(states.get_todo_list().selected().unwrap()),
        KeyCode::Char('d') => todos.delete(states.get_todo_list().selected().unwrap()),
        KeyCode::Char(ALL_KEY_EDIT) => {
            states.init_edit_mode(todos);
            states.set_screen(Screens::Edit);
        }
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
            Screens::Main => return handle_main_ui_events(todos, states, key),
            Screens::Create => handle_create_ui_events(todos, states, key, false),
            Screens::Edit => handle_create_ui_events(todos, states, key, true),
        }
    }
    Ok(false)
}
