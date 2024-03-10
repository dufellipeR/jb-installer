use crate::ides::{Entry, IDE};
use crate::ides::ide::{Buildable, EntryCreator, Readable, SymlinkCreator, Writable};

pub struct Idea {
    entries: Entry,
}

impl Buildable for Idea {}
impl EntryCreator for Idea {}
impl SymlinkCreator for Idea {}

impl Readable for Idea {
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

impl Writable for Idea {
    fn set_icon(&mut self, icon_path: String) {
        self.entries.icon = icon_path;
    }

    fn set_exec(&mut self, exec_path: String) {
        self.entries.exec = exec_path
    }
}

impl IDE for Idea {}

impl Idea {
    pub fn new() -> Self {
        Idea{ entries: Entry {
            name: "IntelliJ IDEA".to_string(),
            comment: "The Leading Java and Kotlin IDE".to_string(),
            icon: "".to_string(),
            exec: "".to_string(),
            short_name: "idea".to_string(),
            hex_color: 0x573dc5,
            }
        }
    }
}