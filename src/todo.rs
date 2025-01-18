use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};

use crate::states::States;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub done: bool,
}

pub struct Todos {
    todos: Vec<Todo>,
    file_path: String,
}

impl Todos {
    pub fn new(file_contents: String, file_path: String) -> Todos {
        Self {
            todos: serde_json::from_str(&file_contents).unwrap_or_default(),
            file_path,
        }
    }

    pub fn write(&mut self) {
        match File::create(&self.file_path) {
            Ok(mut file) => {
                if let Ok(json_str) = serde_json::to_string_pretty(&self.todos) {
                    if file.write_all(json_str.as_bytes()).is_err() {
                        println!("couldn't write to the file '{}'", self.file_path)
                    }
                }
            }
            Err(_) => {
                println!("couldn't open/create to the file '{}'", self.file_path);
            }
        }
    }

    pub fn get_todo(&mut self, idx: usize) -> Option<&Todo> {
        self.todos.get(idx)
    }

    pub fn add(&mut self, title: String, description: String, due_date: String, done: bool) {
        self.todos.push(Todo {
            title,
            description,
            due_date,
            done,
        });
        self.write();
    }

    pub fn edit(&mut self, idx: usize, states: &mut States) {
        if let Some(todo) = self.todos.get_mut(idx) {
            todo.title = String::from(states.get_title());
            todo.description = String::from(states.get_description());
            todo.due_date = String::from(states.get_date());
            self.write();
        }
    }

    pub fn delete(&mut self, idx: usize) {
        self.todos.remove(idx);
        self.write();
    }

    pub fn get_todos_titles(&mut self) -> Vec<String> {
        Vec::from_iter(self.todos.iter().map(|t| match t.done {
            true => format!(
                "✔ {}",
                if t.title.is_empty() {
                    String::from("N/A")
                } else {
                    t.title.clone()
                }
            ),
            false => format!(
                "✘ {}",
                if t.title.is_empty() {
                    String::from("N/A")
                } else {
                    t.title.clone()
                }
            ),
        }))
    }

    pub fn get_description(&mut self, idx: usize) -> String {
        match self.todos.get(idx) {
            Some(todo) => match todo.description.is_empty() {
                true => String::from("N/A"),
                false => self.todos.get(idx).unwrap().description.clone(),
            },
            None => String::from("N/A"),
        }
    }

    pub fn get_due_date(&mut self, idx: usize) -> String {
        match self.todos.get(idx) {
            Some(todo) => match todo.due_date.is_empty() {
                true => String::from("N/A"),
                false => self.todos.get(idx).unwrap().due_date.clone(),
            },
            None => String::from("N/A"),
        }
    }

    pub fn is_done(&mut self, idx: usize) -> String {
        match self.todos.get(idx) {
            Some(todo) => match todo.done {
                true => String::from("✅"),
                false => String::from("❌"),
            },
            None => String::from("N/A"),
        }
    }

    pub fn toggle(&mut self, idx: usize) {
        if let Some(todo) = self.todos.get_mut(idx) {
            todo.done = !todo.done;
            self.write();
        }
    }
}
