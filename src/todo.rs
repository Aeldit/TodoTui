use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

#[derive(Serialize, Deserialize)]
struct Todo {
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub done: bool,
}

impl Todo {
    fn toggle(&mut self) {
        self.done = !self.done;
    }
}

pub struct Todos {
    todos: Vec<Todo>,
    file_path: String,
}

impl Todos {
    pub fn new(file_contents: String, file_path: String) -> Todos {
        let todos: Vec<Todo> = match serde_json::from_str(&file_contents) {
            Ok(t) => t,
            Err(e) => panic!("Problem opening the file: {e:?}"),
        };
        Self { todos, file_path }
    }

    pub fn write(&mut self) {
        if File::create(&self.file_path)
            .unwrap()
            .write_all(
                serde_json::to_string_pretty(&self.todos)
                    .unwrap()
                    .as_bytes(),
            )
            .is_err()
        {
            println!("Couldn't write to the file '{}'", self.file_path)
        }
    }

    pub fn add(&mut self, title: String, contents: String, due_date: String, done: bool) {
        self.todos.push(Todo {
            title,
            description: contents,
            due_date,
            done,
        });
        self.write();
    }

    pub fn delete(&mut self, idx: usize) {
        self.todos.remove(idx);
        self.write();
    }

    pub fn get_todos_titles(&mut self) -> Vec<String> {
        Vec::from_iter(self.todos.iter().map(|t| match t.done {
            true => format!("✔ {}", t.title.clone()),
            false => format!("✘ {}", t.title.clone()),
        }))
    }

    pub fn get_description(&mut self, idx: usize) -> String {
        match self.todos.get(idx).unwrap().description.is_empty() {
            true => String::from("N/A"),
            false => self.todos.get(idx).unwrap().description.clone(),
        }
    }

    pub fn get_due_date(&mut self, idx: usize) -> String {
        match self.todos.get(idx).unwrap().due_date.is_empty() {
            true => String::from("N/A"),
            false => self.todos.get(idx).unwrap().due_date.clone(),
        }
    }

    pub fn is_done(&mut self, idx: usize) -> String {
        match self.todos.get(idx).unwrap().done {
            true => String::from("✅"),
            false => String::from("❌"),
        }
    }

    pub fn toggle(&mut self, idx: usize) {
        self.todos.get_mut(idx).unwrap().toggle();
        self.write();
    }
}
