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

pub enum CurrentString {
    Title,
    Date,
    Description,
}

pub struct States {
    todo_list: ListState,
    screen: Screens,
    is_in_writting_mode: bool,
    title_string: String,
    date_string: String,
    description_string: String,
    current_string: CurrentString,
}

impl States {
    pub fn new() -> Self {
        let mut ret = Self {
            todo_list: ListState::default(),
            screen: Screens::Create,
            is_in_writting_mode: false,
            title_string: String::new(),
            date_string: String::new(),
            description_string: String::new(),
            current_string: CurrentString::Title,
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

    pub fn is_in_writting_mode(&mut self) -> bool {
        self.is_in_writting_mode
    }

    pub fn get_title(&mut self) -> &String {
        &self.title_string
    }

    pub fn get_date(&mut self) -> &String {
        &self.date_string
    }

    pub fn get_description(&mut self) -> &String {
        &self.description_string
    }

    pub fn set_screen(&mut self, screen: Screens) {
        self.screen = screen;
    }

    pub fn set_writting_mode(&mut self, value: bool) {
        self.is_in_writting_mode = value;
    }

    pub fn add_char(&mut self, c: char) {
        match self.current_string {
            CurrentString::Title => self.title_string.push(c),
            CurrentString::Date => self.date_string.push(c),
            CurrentString::Description => self.description_string.push(c),
        }
    }

    pub fn pop_char(&mut self) {
        match self.current_string {
            CurrentString::Title => self.title_string.pop(),
            CurrentString::Date => self.date_string.pop(),
            CurrentString::Description => self.description_string.pop(),
        };
    }

    pub fn next_tab(&mut self) {
        self.current_string = match self.current_string {
            CurrentString::Title => CurrentString::Date,
            CurrentString::Date => CurrentString::Description,
            CurrentString::Description => CurrentString::Title,
        }
    }

    pub fn clear_strings(&mut self) {
        self.title_string.clear();
        self.date_string.clear();
        self.description_string.clear();
    }
}
