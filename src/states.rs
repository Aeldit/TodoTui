use ratatui::{style::Color, widgets::ListState};

pub const MAX_TITLE_LEN: usize = 32;
pub const MAX_DATE_LEN: usize = 32;
pub const MAX_DESCRIPTION_LEN: usize = 4096;

pub const ALL_KEY_EDIT: char = 'i';
// TODO: Implement copy/paste
// pub const ALL_KEY_COPY: char = 'c';
// pub const ALL_KEY_PASTE: char = 'p';

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
            CreateTab::Title => {
                if self.title_string.len() < MAX_TITLE_LEN {
                    self.title_string.push(c);
                }
            }
            CreateTab::Date => {
                if self.date_string.len() < MAX_DATE_LEN {
                    self.date_string.push(c);
                }
            }
            CreateTab::Description => {
                if self.description_string.len() < MAX_DESCRIPTION_LEN {
                    self.description_string.push(c);
                }
            }
        }
    }

    pub fn add_str(&mut self, s: &str) {
        println!("{}", s);
        match self.selected_tab {
            CreateTab::Title => {
                if self.title_string.len() + s.len() < MAX_TITLE_LEN {
                    self.title_string.push_str(s);
                }
            }
            CreateTab::Date => {
                if self.date_string.len() + s.len() < MAX_DATE_LEN {
                    self.date_string.push_str(s);
                }
            }
            CreateTab::Description => {
                if self.description_string.len() + s.len() < MAX_DESCRIPTION_LEN {
                    self.description_string.push_str(s);
                }
            }
        }
    }

    pub fn pop_char(&mut self) {
        match self.selected_tab {
            CreateTab::Title => self.title_string.pop(),
            CreateTab::Date => self.date_string.pop(),
            CreateTab::Description => self.description_string.pop(),
        };
    }

    pub fn get_nb_char_in_tab(&mut self, tab: CreateTab) -> usize {
        match tab {
            CreateTab::Title => self.title_string.len(),
            CreateTab::Date => self.date_string.len(),
            CreateTab::Description => self.description_string.len(),
        }
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

impl Default for States {
    fn default() -> Self {
        Self::new()
    }
}
