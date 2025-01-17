use ratatui::{style::Color, widgets::ListState};
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
        let mut file = File::create(&self.file_path).unwrap();
        if file
            .write_all(
                serde_json::to_string_pretty(&self.todos)
                    .unwrap()
                    .as_bytes(),
            )
            .is_err()
        {
            panic!("Couldn't write to the file '{}'", self.file_path)
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

    pub fn get_description(&mut self, idx: usize) -> String {
        let date = self.todos.get(idx).unwrap().description.clone();
        if date.is_empty() {
            String::from("N/A")
        } else {
            date
        }
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
        self.write();
    }
}

pub enum Screens {
    Main,
    Create,
}

#[derive(PartialEq)]
pub enum CreateTab {
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
    selected_tab: CreateTab,
    file_path: String,
}

impl States {
    pub fn new() -> Self {
        let mut ret = Self {
            todo_list: ListState::default(),
            screen: Screens::Main,
            is_in_writting_mode: false,
            title_string: String::new(),
            date_string: String::new(),
            description_string: String::new(),
            selected_tab: CreateTab::Title,
            file_path: String::new(),
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

    pub fn set_file_path(&mut self, file_path: String) {
        self.file_path = file_path
    }

    pub fn add_char(&mut self, c: char) {
        match self.selected_tab {
            CreateTab::Title => self.title_string.push(c),
            CreateTab::Date => self.date_string.push(c),
            CreateTab::Description => self.description_string.push(c),
        }
    }

    pub fn pop_char(&mut self) {
        match self.selected_tab {
            CreateTab::Title => self.title_string.pop(),
            CreateTab::Date => self.date_string.pop(),
            CreateTab::Description => self.description_string.pop(),
        };
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = match self.selected_tab {
            CreateTab::Title => CreateTab::Date,
            CreateTab::Date => CreateTab::Description,
            CreateTab::Description => CreateTab::Title,
        }
    }

    pub fn clear_strings(&mut self) {
        self.title_string.clear();
        self.date_string.clear();
        self.description_string.clear();
    }

    pub fn get_fg_color_for_tab(&mut self, tab: CreateTab) -> Color {
        if tab == self.selected_tab {
            return Color::Red;
        }
        Color::Magenta
    }
}
