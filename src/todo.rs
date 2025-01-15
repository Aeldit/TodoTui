use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
struct Todo {
    pub title: String,
    pub contents: String,
    pub due_date: String,
    pub done: bool,
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

pub struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    pub fn new(file_contents: String) -> Todos {
        let t: Vec<Todo> = match serde_json::from_str(&file_contents) {
            Ok(t) => t,
            Err(e) => panic!("Problem opening the file: {e:?}"),
        };
        Self { todos: t }
    }

    pub fn add(&mut self, title: String, contents: String, due_date: String, done: bool) {
        self.todos.push(Todo {
            title,
            contents,
            due_date,
            done,
        });
    }

    pub fn get_todos(&mut self) -> Vec<String> {
        let mut titles: Vec<String> = Vec::with_capacity(self.todos.len());
        for todo in &self.todos {
            titles.push(todo.title.clone());
        }
        titles
    }

    pub fn get_contents(&mut self, idx: usize) -> &String {
        &self.todos.get(idx).unwrap().contents
    }
}
