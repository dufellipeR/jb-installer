use crate::ides::{Entry, IDE};

pub struct RubyMine {
    entries: Entry,
}

impl IDE for RubyMine {

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

    fn get_color(&self) -> u64 { self.entries.hex_color }

    fn set_icon(&mut self, icon_path: String) {
        self.entries.icon = icon_path;
    }

    fn set_exec(&mut self, exec_path: String) {
        self.entries.exec = exec_path
    }

}

impl RubyMine {
    pub fn new() -> Self {
        RubyMine{ entries: Entry {
            name: "RubyMine".to_string(),
            comment: "Empowering Ruby Developers".to_string(),
            icon: "".to_string(),
            exec: "".to_string(),
            short_name: "rubymine".to_string(),
            hex_color: 0xdb0ee2
            }
        }
    }
}