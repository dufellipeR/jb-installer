use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::{Path};
use std::time::Instant;
use flate2::read::GzDecoder;
use recolored::Colorize;
use spinoff::{Color, Spinner, spinners};
use tar::Archive;
use crate::ides::{Entry, Goland, IDE, Idea, Pycharm, RubyMine, RustRover};

pub fn greeting() {
    println!("> Welcome to {} !", "Painite".true_color(128, 0,0).bold())
}

pub fn generate_entry(entries: &Entry) -> String {
    return format!("[Desktop Entry]\nType=Application\nName={}\nIcon={}\
    \nExec={}\nComment={}\nCategories=Development;IDE;\nTerminal=false\nStartupWMClass=jetbrains-{}"
                   , entries.name, entries.icon, entries.exec,entries.comment, entries.short_name);
}

pub fn detect_ide(archive_name: &String) -> Result<Box<dyn IDE>, &'static str> {

    let normalized_archive = archive_name.to_lowercase();
    let ide: Box<dyn IDE>;

    if normalized_archive.contains("go") {
        ide = Box::new(Goland::new())
    } else if normalized_archive.contains("py") {
        ide = Box::new(Pycharm::new())
    } else if normalized_archive.contains("rust") {
        ide = Box::new(RustRover::new())
    } else if normalized_archive.contains("idea") {
        ide = Box::new(Idea::new())
    } else if normalized_archive.contains("ruby"){
        ide = Box::new(RubyMine::new())
    } else  {
        return Err("> IDE not supported, symbolic link and desktop entry NOT created");
    }

    Ok(ide)
}

pub fn create_directory(default_path: &Path) -> Result<(), &'static str>{
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

pub fn unpack_tar(file_path: &Path, default_path: &Path) -> Result<String, Error>{
    let start: Instant = Instant::now();
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
            break;
        }
    };
    spinner.stop_with_message(&*format!("> ✅ {} unpacked IDE, took {:?}", "successfully".green(), start.elapsed()));

    Ok(ide_dir_name)
}

#[cfg(test)]
mod tests {
    use crate::ides::Entry;
    use crate::utils::{detect_ide, generate_entry};

    #[test]
    fn it_should_successfully_generate_entry(){
        let entry = Entry{
            name: "TesteIDE".to_string(),
            comment: "The most testable IDE".to_string(),
            icon: "/test/pic.png".to_string(),
            exec: "/test/test.sh".to_string(),
            short_name: "test".to_string(),
            hex_color: 0x000,
        };

        assert_eq!(String::from("[Desktop Entry]\nType=Application\nName=TesteIDE\nIcon=/test/pic.png\
    \nExec=/test/test.sh\nComment=The most testable IDE\nCategories=Development;IDE;\nTerminal=false\nStartupWMClass=jetbrains-test"), generate_entry(&entry));
    }

    #[test]
    fn it_should_successfully_detect_idea() {
        let result = detect_ide(&String::from("idea-UI"));
        assert!(result.is_ok());
        let ide = result.unwrap();
        assert_eq!(ide.as_ref().get_short_name(), "idea");
    }

    #[test]
    fn it_should_successfully_detect_goland() {
        let result = detect_ide(&String::from("GoLand-2023.3.4"));
        assert!(result.is_ok());
        let ide = result.unwrap();
        assert_eq!(ide.as_ref().get_short_name(), "goland");
    }

    #[test]
    fn it_should_successfully_detect_pycharm() {
        let result = detect_ide(&String::from("pycharm-2023.3.3"));
        assert!(result.is_ok());
        let ide = result.unwrap();
        assert_eq!(ide.as_ref().get_short_name(), "pycharm");
    }

    #[test]
    fn it_should_successfully_detect_rustrover() {
        let result = detect_ide(&String::from("RustRover-233.14015.147"));
        assert!(result.is_ok());
        let ide = result.unwrap();
        assert_eq!(ide.as_ref().get_short_name(), "rustrover");
    }

    #[test]
    fn it_should_successfully_detect_rubymine() {
        let result = detect_ide(&String::from("RubyMine-2023.3.4"));
        assert!(result.is_ok());
        let ide = result.unwrap();
        assert_eq!(ide.as_ref().get_short_name(), "rubymine");
    }
}