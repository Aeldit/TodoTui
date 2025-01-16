use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(\"{}\", \"{}\", \"{}\", {})",
            self.title, self.description, self.due_date, self.done
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
            description: contents,
            due_date,
            done,
        });
    }

    pub fn get_todos(&mut self) -> Vec<String> {
        let mut titles: Vec<String> = Vec::with_capacity(self.todos.len());
        for todo in &self.todos {
            if todo.done {
                titles.push(format!("✔ {}", todo.title.clone()));
            } else {
                titles.push(format!("✘ {}", todo.title.clone()));
            }
        }
        titles
    }

    pub fn get_description(&mut self, idx: usize) -> &String {
        &self.todos.get(idx).unwrap().description
    }

    pub fn get_due_date(&mut self, idx: usize) -> String {
        let date = self.todos.get(idx).unwrap().due_date.clone();
        if date.is_empty() {
            String::from("N/A")
        } else {
            date
        }
    }

    pub fn is_done(&mut self, idx: usize) -> String {
        if self.todos.get(idx).unwrap().done {
            String::from("✅")
        } else {
            String::from("❌")
        }
    }

    pub fn toggle(&mut self, idx: usize) {
        let todo = self.todos.get_mut(idx).unwrap();
        todo.toggle();
    }
}

pub enum Screens {
    Main,
    Create,
}

pub struct States {
    todo_list: ListState,
    screen: Screens,
}

impl States {
    pub fn new() -> Self {
        let mut ret = Self {
            todo_list: ListState::default(),
            screen: Screens::Main,
        };
        ret.todo_list.select_first();
        ret
    }

    pub fn get_todo_list(&mut self) -> &mut ListState {
        &mut self.todo_list
    }

    pub fn get_screen(&mut self) -> &Screens {
        &self.screen
    }

    pub fn set_screen(&mut self, screen: Screens) {
        self.screen = screen;
    }
}
