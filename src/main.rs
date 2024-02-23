use core::panic;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::{Instant};

use std::process;

use clap::Parser;
use flate2::read::GzDecoder;
use spinoff::{Color, Spinner, spinners};
use tar::Archive;
use serde::{Deserialize, Serialize};

use sudo::{check, RunningAs};
use jb_installer::Ide;

#[derive(Parser)]
struct Cli {
    zip_path: PathBuf
}


fn main() -> io::Result<()>{
    let args = Cli::parse();

    // checking privileges
    match check() {
        RunningAs::Root => {},
        _ => {
            println!("The application needs SUDO permission to work properly\nTry running with \"sudo\" command");
            process::exit(1);
        },
    }

    let default_path = Path::new("/opt/jetbrains");

    create_directory(&default_path);

    let archive_name = unpack_tar(&args.zip_path, &default_path)?;

    Ide::build(&archive_name, &default_path).unwrap_or_else(|err| {
        println!("Some error occurred {err} when trying to build IDE");
        process::exit(1)
    });

    Ide::create_symlink().unwrap_or_else(|err| {
        println!("Some error occurred {err} when trying to create symlink")
    });

    Ide::create_entry().unwrap_or_else(|err| {
        println!("Some error occurred {err}");
        process::exit(1)
    });

    Ok(())
}

fn create_directory(default_path: &Path) -> Result<(), &'static str>{
    match fs::create_dir(&default_path) {
        Ok(_) => { println!("> ✅ successfully created JetBrains directory") }
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => println!("> skipping directory creation..."),
            ErrorKind::PermissionDenied => panic!("> no permission to create directory at {:?}, try running with \"sudo\"", default_path),
            _err => return Err("> unknown error when creating main directory")
        },
    };

    Ok(())
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_detecting_ides(){
        assert_eq!((String::from("rustrover"), String::from("1.0")), Ide::detect_ide("RustRover-1.0").unwrap());
        assert_eq!((String::from("goland"), String::from("1.0")), Ide::detect_ide("GoLand-1.0").unwrap());
        assert_eq!((String::from("idea"), String::from("1.0")), Ide::detect_ide("idea-IU-1.0").unwrap());
        assert_eq!((String::from("pycharm"), String::from("1.0")), Ide::detect_ide("PyCharm-1.0").unwrap());
        assert!(Ide::detect_ide("unknown-ide").is_err());
    }
}
