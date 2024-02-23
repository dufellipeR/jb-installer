use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process;
use serde::{Deserialize, Serialize};

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
    pub fn build(archive_name: &str, main_dir_path: &Path) -> Result<Ide, &'static str> {
        let ide_details = Self::detect_ide(archive_name).unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(1)
        });

        Self.short_name = ide_details.0;
        Self.version = ide_details.1;

        let format_path = format!("src/entries/{}.json", Self.short_name);
        let path = Path::new(&format_path);
        let file = File::open(path).unwrap_or_else(|err| {
            println!("Error when reading file");
            process::exit(1)
        });
        let reader = BufReader::new(file);
        let parsed_ide: Ide = serde_json::from_reader(reader).expect("Failed to parse JSON");

        Self.name = parsed_ide.name;
        Self.comment = parsed_ide.comment;
        Self.icon = format!("{}/{}bin/{}.png", main_dir_path.display(), archive_name, ide_details.0);
        Self.exec = format!("{}/{}bin/{}.sh", main_dir_path.display(), archive_name, ide_details.0);

        Ok(Self)
    }

    fn detect_ide(archive_name: &str) -> Result<(String, String), &'static str> {
        let parts: Vec<&str> = archive_name.split("-").collect();
        let version = parts[parts.len() - 1].replace("/", "");

        let normalized_archive = archive_name.to_lowercase();
        let short_name: String;

        if normalized_archive.contains("go") {
            short_name = String::from("goland")
        } else if normalized_archive.contains("py") {
            short_name = String::from("pycharm")
        } else if normalized_archive.contains("rust") {
            short_name = String::from("rustrover")
        } else if normalized_archive.contains("idea") {
            short_name = String::from("idea")
        } else {
            return Err("> IDE not supported, symbolic link and desktop entry NOT created");
        }

        Ok((short_name, String::from(version)))
    }

    pub fn create_symlink() -> Result<(), &'static str>{
        match symlink(Self.exec, format!("/usr/local/bin/{}", Self.short_name)) {
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

    pub fn create_entry() -> Result<(), &'static str>{
        let filename = format!("/usr/share/applications/{}.desktop", Self.short_name);
        let mut file = File::create(filename)?;

        //creating and formatting entry content
        let entry = format!("[Desktop Entry]\nVersion={}\nType=Application\nName={}\nIcon={}\
    \nExec={}\nComment={}\nCategories=Development;IDE;\nTerminal=false\nStartupWMClass=jetbrains-{}"
                            , Self.version, Self.name, Self.icon, Self.exec, Self.comment, Self.short_name);

        match file.write_all(entry.as_ref()) {
            Ok(_) => println!("> ✅ successfully created desktop entry"),
            Err(e) => return Err("> unexpected error")
        };

        Ok(())
    }
}
