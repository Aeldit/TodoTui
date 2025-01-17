use homedir::my_home;
use std::fs::{exists, read_to_string};

pub mod tui;
use tui::{drawing::draw, events_handling::handle_events};

mod states;
mod todo;
use states::States;
use todo::Todos;

fn get_file_path(states: &mut States) -> String {
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

    states.set_file_path(file_path.clone());

    file_path
}

fn main() {
    let mut states = States::new();

    let file_path = get_file_path(&mut states);

    let file_exists = exists(&file_path);
    if file_exists.is_err() || file_exists.is_ok_and(|v| !v) {
        match std::fs::File::create(&file_path) {
            Ok(_) => {}
            Err(_) => panic!("The file '{}' does not exist", file_path),
        }
    };
    let file_contents = read_to_string(&file_path).unwrap(); //.expect("Couldn't read the file");

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
