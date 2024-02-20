use core::panic;
use std::fs::{self, File};
use std::io::{self, ErrorKind};
use std::path::{self, Path, PathBuf, StripPrefixError};
use std::time::{Duration, Instant};

use std::os::unix::fs::symlink;
use std::thread::sleep;

use clap::Parser;
use flate2::read::GzDecoder;
use spinoff::{Color, Spinner, spinners};
use tar::Archive;

#[derive(Parser)]
struct Cli {
    zip_path: std::path::PathBuf
}

fn unpack_tar(file_path: &PathBuf, dest_path: &Path) -> Result<String, io::Error>{

    let start: Instant = Instant::now();
    let mut spinner = Spinner::new(spinners::Binary, "Extracting files...", Color::Green);

    let file = File::open(file_path)?;
    let file_read = File::open(file_path)?;

    let mut archive = Archive::new(GzDecoder::new(file));
    let mut archive_read = Archive::new(GzDecoder::new(file_read));

    let mut ide_dir_name = String::new();

    archive.unpack(dest_path)?;
    for entry in archive_read.entries()? {
        let mut entry = entry?;
        if let Some(name) = entry.path()?.to_str() {
            ide_dir_name = name.to_string();
            break; // Exit loop after getting the first entry
        }
    };
    spinner.stop_with_message(&*format!("✅ IDE successfully unpacked, took {:?}", start.elapsed()));
    Ok(ide_dir_name)
}

// fn create_desktop_entry(){
//     println!("test")
//
//     /*
//         [Desktop Entry]
//         Version=1.0
//         Type=Application
//         Name=IntelliJ IDEA
//         Icon=/opt/idea-<version>/bin/idea.png
//         Exec="/opt/idea-<version>/bin/idea.sh" %f
//         Comment=Develop with pleasure!, The complete IDE crafted for Gophers, The Python IDE for Professional Developers, Focus on what matters
//         Categories=Development;IDE;
//         Terminal=false
//         StartupWMClass=jetbrains-idea
//      */
//
// }
//
fn main() -> std::io::Result<()>{
    
    // identify de IDE by path name ? by archive ? 

    let args = Cli::parse();

    let default_path = Path::new("/home/hal/jetbrains");

    
    println!("Creating JetBrains dir...!");
    
    match fs::create_dir(default_path) {
        Ok(result) => result,
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => println!("Skipping directory creation..."),
            _ => {
                panic!("Try running with sudo: {}", error)
            }
        }
    };

    let archive_name = unpack_tar(&args.zip_path, &default_path)?;

    println!("archive: {}/{}", default_path.display(), archive_name.replace("/", ""));

    // Trying to create a symbolic link
    // symlink(original, link)


    Ok(())
}
