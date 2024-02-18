use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::Instant;

use std::os::unix::fs::symlink;

use clap::Parser;
use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Parser)]
struct Cli {
    zip_path: std::path::PathBuf
}

fn unpack_tar(file_path: &PathBuf, dest_path: &Path) -> std::io::Result<()>{

    println!("Start unpacking...");

    let start: Instant = Instant::now();

    let file = File::open(file_path)?;

    let tar = GzDecoder::new(file);

    let mut archive = Archive::new(tar);

    println!("This may take a while...");

    archive.unpack(dest_path)?; // i need to pass the new address of the archive default_path + archiveName

    let finish = Instant::now();

    println!("Successful unpacked, took {:?}", finish.duration_since(start));

    Ok(())
}


fn main() -> std::io::Result<()>{

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

    let _ = unpack_tar(&args.zip_path, &default_path);

    // Trying to create a symbolic link
    // symlink(original, link)


    Ok(())
}
