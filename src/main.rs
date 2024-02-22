use core::panic;
use std::fs::{self, File};
use std::io::{self, BufReader, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::{Instant};

use std::os::unix::fs::symlink;

use clap::Parser;
use flate2::read::GzDecoder;
use spinoff::{Color, Spinner, spinners};
use tar::Archive;
use serde::{Deserialize, Serialize};

use sudo::{check, RunningAs};

#[derive(Parser)]
struct Cli {
    zip_path: PathBuf
}

#[derive(Debug, Serialize, Deserialize)]
struct Ide {
    name: String,
    comment: String,
    ttype: String,
    categories: String,
    terminal: bool,
    startup: String,
    icon: String,
    exec: String,
    version: String,
    short_name: String,
}


fn main() -> io::Result<()>{
    let args = Cli::parse();

    match check() {
        RunningAs::Root => {},
        _ => panic!("The application needs SUDO permission to work properly\nTry running with \"sudo\" command"),
    }

    let default_path = Path::new("/opt/jetbrains");

    create_directory(&default_path);

    let archive_name = unpack_tar(&args.zip_path, &default_path)?;

    let ide = build_ide(&archive_name, &default_path)?;

    create_symlink(&ide)?;

    create_desktop_entry(&ide)?;

    Ok(())
}

fn create_directory(default_path: &Path){
    match fs::create_dir(&default_path) {
        Ok(_) => { println!("> ✅ successfully created JetBrains directory") }
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => println!("> skipping directory creation..."),
            ErrorKind::PermissionDenied => panic!("> no permission to create directory at {:?}, try running with \"sudo\"", default_path),
            _err => panic!("> unknown error: {_err}")
        },
    };
}

fn unpack_tar(file_path: &PathBuf, dest_path: &Path) -> Result<String, io::Error>{
    let start: Instant = Instant::now();
    let mut spinner = Spinner::new(spinners::Binary, "extracting files...", Color::Green);

    let file = File::open(file_path)?;
    let file_read = File::open(file_path)?;

    let mut archive = Archive::new(GzDecoder::new(file));
    let mut archive_read = Archive::new(GzDecoder::new(file_read));

    let mut ide_dir_name = String::new();

    match archive.unpack(dest_path) {
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

fn detect_ide(archive_name: &str) -> Result<(String, String), io::Error> {
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
        panic!("> IDE not supported, symbolic link and desktop entry NOT created")
    }
    return Ok((short_name, String::from(version)))
}

fn build_ide(archive_name: &str, main_dir_path: &Path) -> Result<Ide, io::Error > {
    let ide_details = detect_ide(archive_name)?;

    let format_path = format!("src/entries/{}.json", ide_details.0);
    let path = Path::new(&format_path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut parsed_ide: Ide = serde_json::from_reader(reader).expect("Failed to parse JSON");

    parsed_ide.icon = format!("{}/{}bin/{}.png", main_dir_path.display(), archive_name, ide_details.0);
    parsed_ide.exec = format!("{}/{}bin/{}.sh", main_dir_path.display(), archive_name, ide_details.0);
    parsed_ide.version = ide_details.1;

    return Ok(parsed_ide);
}

fn create_symlink(ide: &Ide) -> Result<(), io::Error>{
    match symlink(&ide.exec, format!("/usr/local/bin/{}", &ide.short_name)) {
        Ok(_) => { println!("> ✅ successfully created symbolic link") }
        Err(err) => match err.kind() {
            ErrorKind::AlreadyExists => println!("> skipping symlink creation..."),
            ErrorKind::PermissionDenied => {
                println!("> no permission to create symlink, try running with \"sudo\"");
                println!("> skipping symlink creation...");
            }
            error => panic!("> unknown error: {error}")
        }
    }

    Ok(())
}

fn create_desktop_entry(ide: &Ide) -> Result<(), io::Error>{
    let filename = format!("/usr/share/applications/{}.desktop", ide.short_name);
    let mut file = File::create(filename)?;

    //creating and formatting entry content
    let entry = format!("[Desktop Entry]\nVersion={}\nType=Application\nName={}\nIcon={}\
    \nExec={}\nComment={}\nCategories=Development;IDE;\nTerminal=false\nStartupWMClass=jetbrains-{}"
                        , ide.version, ide.name, ide.icon, ide.exec, ide.comment, ide.short_name);

    match file.write_all(entry.as_ref()) {
        Ok(_) => println!("> ✅ successfully created desktop entry"),
        Err(e) => panic!("> unexpected error {e}")
    };

    Ok(())
}

// fn create_resume_operations(ide: &Ide){
//     /*
//        {IDE NAME}
//
//        Main folder step:
//             - created JetBrains folder
//             - skipped JetBrains folder
//             - failed to create JetBrains folder
//
//        Unpack IDE step:
//             - unpacked into JetBrains folder
//             - failed to unpack
//
//        Symlink step:
//             - symlink created
//             - failed to create symlink
//
//        Entry step:
//             - Entry successfully created
//
//     */
// }
