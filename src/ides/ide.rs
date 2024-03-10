use std::fs::File;
use std::io::{ErrorKind, Write};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process;
use recolored::Colorize;
use crate::utils::generate_entry;

pub trait Buildable: Writable + Readable {
    fn build(&mut self, archive_name: &String, main_dir_path: &Path) -> Result<(), &'static str> {
        self.set_icon(format!("{}/{}bin/{}.png", main_dir_path.to_string_lossy(), archive_name, self.get_short_name()));
        self.set_exec(format!("{}/{}bin/{}.sh", main_dir_path.to_string_lossy(), archive_name, self.get_short_name()));

        Ok(())
    }
}

pub trait SymlinkCreator: Readable {
    fn create_symlink(&self, default_symlink_path: &Path) -> Result<(), &'static str> {
        let path = default_symlink_path.join(&self.get_short_name());

        match symlink(&self.get_exec(), &path) {
            Ok(_) => println!("> ✅ {} created symbolic link", "successfully".green()),
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => println!("> ✅ skipping symlink creation..."),
                ErrorKind::PermissionDenied => {
                    println!("> no permission to create symlink, try running with \"sudo\"");
                    println!("> ✅ skipping symlink creation...");
                }
                _err => return Err("> unknown error when creating symlink")
            }
        }
        Ok(())
    }
}

pub trait EntryCreator: Readable {
    fn create_entry(&self, default_entry_path: &Path) -> Result<(), &'static str>{
        let filename = format!("{}{}.desktop", default_entry_path.to_string_lossy(), self.get_short_name());
        let mut file = File::create(&filename).unwrap_or_else(|err| {
            println!("> unexpected error occurred {err}");
            process::exit(1)
        });

        //creating and formatting entry content
        let entry = generate_entry(self.get_entries());

        match file.write_all(entry.as_ref()) {
            Ok(_) => println!("> ✅ {} created desktop entry", "successfully".green()),
            Err(_) => return Err("> unexpected error when writing IDE entry")
        };

        Ok(())
    }

}

pub trait Readable {
    fn get_name(&self) -> &String;
    fn get_comment(&self) -> &String;
    fn get_short_name(&self) -> &String;
    fn get_exec(&self) -> &String;
    fn get_icon(&self) -> &String;
    fn get_entries(&self) -> &Entry;

    fn get_color(&self) -> u64;
}

pub trait Writable {
    fn set_icon(&mut self, icon_path: String);
    fn set_exec(&mut self, exec_path: String);
}

pub trait IDE: Buildable + SymlinkCreator + EntryCreator + Readable + Writable {}
pub struct Entry {
    pub name: String,
    pub comment: String,
    pub icon: String,
    pub exec: String,
    pub short_name: String,
    pub hex_color: u64
}