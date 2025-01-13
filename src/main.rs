use homedir::my_home;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{exists, read_to_string};

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    contents: String,
    due_date: String,
    done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(\"{}\", \"{}\", \"{}\", {})",
            self.title, self.contents, self.due_date, self.done
        )
    }
}

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
    let contents = match read_file() {
        None => return,
        Some(contents) => contents,
    };
    println!("{}", contents);

    let t: Vec<Todo> = match serde_json::from_str(&contents) {
        Ok(t) => t,
        Err(e) => panic!("Problem opening the file: {e:?}"),
    };
    for todo in t {
        println!("{}", todo);
    }
}
