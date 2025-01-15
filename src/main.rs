use homedir::my_home;
use std::fs::{exists, read_to_string};
use todo_tui::{draw, handle_events};

mod todo;
mod todo_tui;
use todo::Todos;

use ratatui::widgets::ListState;

fn read_file() -> Option<String> {
    let home_dir = match my_home() {
        Ok(home_dir) => home_dir,
        Err(error) => panic!("Couldn't get the home dir: {error:?}"),
    };
    let binding = home_dir.unwrap();
    let home_dir_str = match binding.to_str() {
        Some(home_dir_str) => home_dir_str,
        None => panic!("Couldn't get the string from the home dir"),
    };
    let file_path = String::from(home_dir_str) + "/.config/todo-tui/todos.json";

    let file_exists = exists(&file_path);
    if file_exists.is_err() || file_exists.is_ok_and(|v| !v) {
        return None;
    }

    Some(read_to_string(file_path).expect("Couldn't read the file"))
}

fn main() {
    let file_contents = match read_file() {
        None => return,
        Some(contents) => contents,
    };

    let mut todos = Todos::new(file_contents);

    // TUI
    let mut todos_state = ListState::default();
    todos_state.select_first();

    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame| draw(frame, &mut todos_state, &mut todos))
            .expect("Failed to draw frame");
        if matches!(handle_events(&mut todos, &mut todos_state), Ok(true)) {
            break;
        }
    }
    ratatui::restore();
}
