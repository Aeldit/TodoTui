use homedir::my_home;
use std::fs::{exists, read_to_string};
use todo_tui::{draw, handle_events};

mod todo;
mod todo_tui;
use todo::Todos;

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
        println!("a");
        return None;
    }

    Some(read_to_string(file_path).expect("Couldn't read the file"))
}

fn main() {
    // Reading
    let file_contents = match read_file() {
        None => return,
        Some(contents) => contents,
    };
    println!("{}", file_contents);

    let mut todos = Todos::new(file_contents);

    // TUI
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("Failed to draw frame");
        if matches!(handle_events(&mut todos), Ok(true)) {
            break;
        }
    }
    ratatui::restore();
}
