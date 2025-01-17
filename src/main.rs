use homedir::my_home;
use std::fs::{exists, read_to_string};

pub mod tui;
use tui::{drawing::draw, events_handling::handle_events};

mod states;
mod todo;
use states::States;
use todo::Todos;

fn get_file_path(states: &mut States) -> Option<String> {
    let home_dir = match my_home() {
        Ok(home_dir) => home_dir,
        Err(error) => panic!("Couldn't get the home dir: {error:?}"),
    };

    let home_dir_str = match home_dir {
        Some(home_dir_str) => home_dir_str,
        None => panic!("Couldn't get the string from the home dir"),
    };
    match home_dir_str.to_str() {
        Some(s) => {
            let file_path = String::from(s) + "/.config/todo-tui/todos.json";
            states.set_file_path(file_path.clone());
            Some(file_path)
        }
        None => None,
    }
}

fn main() {
    let mut states = States::new();

    let file_path = match get_file_path(&mut states) {
        Some(file_path) => file_path,
        None => return,
    };

    let file_exists = exists(&file_path);
    if (file_exists.is_err() || file_exists.is_ok_and(|v| !v))
        && std::fs::File::create(&file_path).is_err()
    {
        panic!("The file todos file does not exist and could not be create")
    };
    let file_contents = match read_to_string(&file_path) {
        Ok(file_contents) => file_contents,
        Err(_) => panic!("Counld read the file '{}'", file_path),
    };

    let mut todos = Todos::new(file_contents, file_path);

    // TUI
    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame| draw(frame, &mut states, &mut todos))
            .expect("Failed to draw frame");
        if matches!(handle_events(&mut todos, &mut states), Ok(true)) {
            break;
        }
    }
    ratatui::restore();
}
