extern crate walkdir;
use ratatui::widgets::ListState;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub current_path: String,
    pub state: ListState,
    pub current_item_details: Option<String>,
    pub current_item_content: Option<String>,
    pub list_of_items: Vec<String>,
}

impl App {
    pub fn new(current_path: String) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            should_quit: false,
            current_path,
            state,
            current_item_details: None,
            current_item_content: None,
            list_of_items: vec![],
        }
    }

    pub fn update_current_path(&mut self) {
        let i = self.state.selected().unwrap();

        if i == 0 {
            let path = Path::new(&self.current_path);
            let parent_path = path.parent().unwrap().to_str().unwrap().to_string();
            self.current_path = parent_path;
            self.get_items();
            ()
        }
        let selected_item = self.list_of_items.get(i).unwrap();
        let path = &format!("{}/{}", self.current_path, selected_item);

        match fs::metadata(path) {
            Ok(metadat) => {
                if metadat.is_dir() {
                    self.current_path = path.clone();
                    self.get_items();
                }
            }
            Err(_) => {}
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn next_item(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list_of_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.get_item_content(i);
        self.get_item_details(i);
    }

    pub fn previous_item(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list_of_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.get_item_content(i);
        self.get_item_details(i);
    }

    pub fn get_items(&mut self) {
        self.list_of_items = WalkDir::new(self.current_path.clone())
            .max_depth(1)
            .into_iter()
            .enumerate()
            .map(|(index, entry)| {
                if index == 0 {
                    return format!("{}", ".");
                }
                entry.unwrap().file_name().to_str().unwrap().to_string()
            })
            .collect::<Vec<String>>();
    }

    pub fn get_item_content(&mut self, i: usize) {
        let item = self.list_of_items.get(i).unwrap();
        let path = format!("{}/{}", self.current_path, item);
        match fs::read_to_string(path) {
            Ok(content) => {
                self.current_item_content = Some(content);
            }
            Err(_) => {
                self.current_item_content = None;
            }
        }
    }

    fn get_readeable_time(&mut self, time: SystemTime) -> String {
        let datetime: DateTime<Utc> = time.into();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn get_item_details(&mut self, i: usize) {
        let item = self.list_of_items.get(i).unwrap();
        let path = format!("{}/{}", self.current_path, item);
        match fs::metadata(path) {
            Ok(meta) => {
                let is_dir = meta.file_type().is_dir();
                let time_created = meta.created().unwrap();
                let time_modified = meta.modified().unwrap();
                if is_dir {
                    self.current_item_details = Some(format!(
                        "{}\nCreated: {}\nModified: {}",
                        "Directory",
                        self.get_readeable_time(time_created),
                        self.get_readeable_time(time_modified)
                    ));
                } else {
                    self.current_item_details = Some(format!(
                        "{}\nCreated: {}\nModified: {}",
                        "File",
                        self.get_readeable_time(time_created),
                        self.get_readeable_time(time_modified)
                    ));
                }
            }
            Err(_) => {
                self.current_item_details = None;
            }
        }
    }
}
