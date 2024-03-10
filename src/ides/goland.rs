use crate::ides::{IDE, Entry};
use crate::ides::ide::{Buildable, EntryCreator, Readable, SymlinkCreator, Writable};

pub struct Goland {
    entries: Entry,
}

impl Buildable for Goland {}
impl EntryCreator for Goland {}
impl SymlinkCreator for Goland {}

impl Readable for Goland {
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
}

impl Writable for Goland {
    fn set_icon(&mut self, icon_path: String) {
        self.entries.icon = icon_path;
    }

    fn set_exec(&mut self, exec_path: String) {
        self.entries.exec = exec_path
    }
}

impl IDE for Goland {}

impl Goland {
    pub fn new() -> Self {
        Goland{ entries: Entry {
            name: "GoLand".to_string(),
            comment: "The complete IDE crafted for Gophers".to_string(),
            icon: "".to_string(),
            exec: "".to_string(),
            short_name: "goland".to_string(),
            hex_color: 0x7147f8,
            }
        }
    }
}