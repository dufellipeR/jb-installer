use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::{fs, io, process};
use std::time::Instant;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use spinoff::{Color, Spinner, spinners};
use tar::Archive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ide {
    name: String,
    comment: String,
    icon: String,
    exec: String,
    version: String,
    short_name: String,
}

impl Ide {
    // Separate into modules https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html
    pub fn new() -> Ide {
        Ide{
            name: String::new(),
            comment: String::new(),
            icon: String::new(),
            exec: String::new(),
            version: String::new(),
            short_name: String::new(),
        }
    }
    pub fn build(&mut self, archive_name: &str, main_dir_path: &Path) -> Result<(), &'static str> {
        let ide_details = Self::detect_ide(archive_name).unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(1)
        });

        self.short_name = ide_details.0;
        self.version = ide_details.1;

        let format_path = format!("src/entries/{}.json", self.short_name);
        let path = Path::new(&format_path);
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("No entries found for this IDE")
        };
        let reader = BufReader::new(file);
        let parsed_ide: Ide = serde_json::from_reader(reader).expect("Failed to parse JSON");

        self.name = parsed_ide.name;
        self.comment = parsed_ide.comment;
        self.icon = format!("{}/{}bin/{}.png", main_dir_path.display(), archive_name, self.short_name);
        self.exec = format!("{}/{}bin/{}.sh", main_dir_path.display(), archive_name, self.short_name);

        Ok(())
    }

    fn detect_ide(archive_name: &str) -> Result<(String, String), &'static str> {
        let parts: Vec<&str> = archive_name.split("-").collect();
        let version: String = parts[parts.len() - 1].replace("/", "");

        let normalized_archive = archive_name.to_lowercase();
        let short_name: &'static str;

        if normalized_archive.contains("go") {
            short_name = "goland"
        } else if normalized_archive.contains("py") {
            short_name = "pycharm"
        } else if normalized_archive.contains("rust") {
            short_name = "rustrover"
        } else if normalized_archive.contains("idea") {
            short_name = "idea"
        } else {
            return Err("> IDE not supported, symbolic link and desktop entry NOT created");
        }

        Ok((String::from(short_name), String::from(version)))
    }

    pub fn create_symlink(&self) -> Result<(), &'static str>{
        match symlink(&self.exec, format!("/usr/local/bin/{}", self.short_name)) {
            Ok(_) => println!("> ✅ successfully created symbolic link"),
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => println!("> skipping symlink creation..."),
                ErrorKind::PermissionDenied => {
                    println!("> no permission to create symlink, try running with \"sudo\"");
                    println!("> skipping symlink creation...");
                }
                _err => return Err("> unknown error when creating symlink")
            }
        }

        Ok(())
    }

    pub fn create_entry(&self) -> Result<(), &'static str>{
        let filename = format!("/usr/share/applications/{}.desktop", self.short_name);
        let mut file = File::create(filename).unwrap_or_else(|err| {
            println!("> unexpected error occurred {err}");
            process::exit(1)
        });

        //creating and formatting entry content
        let entry = format!("[Desktop Entry]\nVersion={}\nType=Application\nName={}\nIcon={}\
    \nExec={}\nComment={}\nCategories=Development;IDE;\nTerminal=false\nStartupWMClass=jetbrains-{}"
                            , self.version, self.name, self.icon, self.exec, self.comment, self.short_name);

        match file.write_all(entry.as_ref()) {
            Ok(_) => println!("> ✅ successfully created desktop entry"),
            Err(_) => return Err("> unexpected error when writing IDE entry")
        };

        Ok(())
    }
}

pub fn create_directory(default_path: &Path) -> Result<(), &'static str>{
    match fs::create_dir(&default_path) {
        Ok(_) => { println!("> ✅ successfully created JetBrains directory") }
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => println!("> skipping directory creation..."),
            ErrorKind::PermissionDenied => return Err("> no permission to create directory, try running with \"sudo\""),
            _err => return Err("> unknown error when creating main directory")
        },
    };

    Ok(())
}

pub fn unpack_tar(file_path: &PathBuf, default_path: &Path) -> Result<String, io::Error>{
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
            break; // Exit loop after getting the first entry
        }
    };
    spinner.stop_with_message(&*format!("> ✅ successfully unpacked IDE, took {:?}", start.elapsed()));

    Ok(ide_dir_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test all functions

    // #[test]
    // fn is_building_ides() {
    //     let archive_name = "RustRover-2023.0.1/";
    //     let path = Path::new("/opt/JetBrains");
    //     let mut ide = Ide::new();
    //
    //     assert!(Ide::build(&mut ide, archive_name, path).is_ok());
    //     assert_eq!(ide.short_name, "rustrover");
    //     assert_eq!(ide.exec, "/opt/JetBrains/RustRover-2023.0.1/bin/rustrover.sh");
    //     assert_eq!(ide.icon, "/opt/JetBrains/RustRover-2023.0.1/bin/rustrover.png");
    //
    // }
    //
    // #[test]
    // fn is_creating_symlink {
    //
    // }


    #[test]
    fn test_successfully_detect_rustrover(){
        assert_eq!((String::from("rustrover"), String::from("1.0")), Ide::detect_ide("RustRover-1.0").unwrap());
    }

    #[test]
    fn test_successfully_detect_idea(){
        assert_eq!((String::from("idea"), String::from("1.0")), Ide::detect_ide("Idea-IU-1.0").unwrap());
    }

    #[test]
    fn test_successfully_detect_goland(){
        assert_eq!((String::from("goland"), String::from("1.0")), Ide::detect_ide("GoLand-1.0").unwrap());
    }

    #[test]
    fn test_successfully_detect_pycharm(){
        assert_eq!((String::from("pycharm"), String::from("1.0")), Ide::detect_ide("PyCharm-1.0").unwrap());
    }

    #[test]
    fn test_unknown_ide(){
        assert!(Ide::detect_ide("unknown-ide").is_err());
    }
}
