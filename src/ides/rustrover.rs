use crate::ides::{Entry, IDE};

pub struct RustRover {
    entries: Entry,
}

impl IDE for RustRover {

    fn get_name(&self) -> &String {
        &self.entries.name
    }

    fn get_comment(&self) -> &String {
        &self.entries.comment
    }

    fn get_short_name(&self) -> &String {
        &self.entries.short_name
    }

    fn get_exec(&self) -> &String {
        &self.entries.exec
    }

    fn get_icon(&self) -> &String {
        &self.entries.icon
    }

    fn get_entries(&self) -> &Entry {
        &self.entries
    }

    fn set_icon(&mut self, icon_path: String) {
        self.entries.icon = icon_path;
    }

    fn set_exec(&mut self, exec_path: String) {
        self.entries.exec = exec_path
    }

}

impl RustRover {
    pub fn new() -> Self {
        RustRover{ entries: Entry {
            name: "RustRover".to_string(),
            comment: "Focus on what matters".to_string(),
            icon: "".to_string(),
            exec: "".to_string(),
            short_name: "rustrover".to_string(),
        }}
    }
}