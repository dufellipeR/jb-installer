use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::time::Instant;
use flate2::read::GzDecoder;
use recolored::Colorize;
use spinoff::{Color, Spinner, spinners};
use tar::Archive;
use crate::ides::{Entries, Goland, IDE, Idea, Pycharm, RustRover};


pub fn generate_entry(entries: &Entries) -> String {
    return format!("[Desktop Entry]\nVersion={}\nType=Application\nName={}\nIcon={}\
    \nExec={}\nComment={}\nCategories=Development;IDE;\nTerminal=false\nStartupWMClass=jetbrains-{}"
                   ,entries.version, entries.name, entries.icon, entries.exec,entries.comment, entries.short_name);
}

pub fn detect_version(archive_name: &str){
    let parts: Vec<&str> = archive_name.split("-").collect();
    let version: String = parts[parts.len() - 1].replace("/", "");
}

pub fn detect_ide(archive_name: &String) -> Result<Box<dyn IDE>, &'static str> {

    let normalized_archive = archive_name.to_lowercase();
    let mut ide: Box<dyn IDE>;

    if normalized_archive.contains("go") {
        ide = Box::new(Goland::new())
    } else if normalized_archive.contains("py") {
        ide = Box::new(Pycharm::new())
    } else if normalized_archive.contains("rust") {
        ide = Box::new(RustRover::new())
    } else if normalized_archive.contains("idea") {
        ide = Box::new(Idea::new())
    } else {
        return Err("> IDE not supported, symbolic link and desktop entry NOT created");
    }

    Ok(ide)
}


pub fn create_directory(default_path: &str) -> Result<(), &'static str>{
    match fs::create_dir(&default_path) {
        Ok(_) => { println!("> ✅ {} created JetBrains directory", "successfully".green()) }
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => println!("> ✅ skipping directory creation..."),
            ErrorKind::PermissionDenied => return Err("> no permission to create directory, try running with \"sudo\""),
            _err => return Err("> unknown error when creating main directory")
        },
    };

    Ok(())
}

pub fn unpack_tar(file_path: &PathBuf, default_path: &str) -> Result<String, Error>{
    let start: Instant = Instant::now();
    print!("> ");
    let mut spinner = Spinner::new(spinners::Binary, "extracting files...", Color::Green);

    let file = File::open(file_path)?;
    let file_read = File::open(file_path)?;

    let mut archive = Archive::new(GzDecoder::new(file));
    let mut archive_read = Archive::new(GzDecoder::new(file_read));

    let mut ide_dir_name = String::new();

    match archive.unpack(default_path) {
        Ok(_) => {},
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => panic!("> no permission, try running with \"sudo\""),
            ErrorKind::AlreadyExists => panic!("> unpacked IDE already exists"),
            _err => panic!("> unknown error {_err}")
        }
    };

    for entry in archive_read.entries()? {
        let entry = entry?;
        if let Some(name) = entry.path()?.to_str() {
            ide_dir_name = name.to_string();
            break; // Exit loop after getting the first entry
        }
    };
    spinner.stop_with_message(&*format!("> ✅ {} unpacked IDE, took {:?}", "successfully".green(), start.elapsed()));

    Ok(ide_dir_name)
}