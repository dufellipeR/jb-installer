use std::io::{self};
use std::path::{Path, PathBuf};

use std::process;

use clap::Parser;

use sudo::{check, RunningAs};
use painite::{create_directory, Ide, unpack_tar};

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
            println!("> The application needs SUDO permission to work properly\n> Try running with \"sudo\" command");
            process::exit(1);
        },
    }

    let default_path = Path::new("/opt/JetBrains");

    create_directory(&default_path).unwrap_or_else(|err| {
        println!("Unexpected error occurred {err} when trying to create directory");
        process::exit(1)
    });

    let archive_name = unpack_tar(&args.zip_path, &default_path)?;

    let mut ide = Ide::new();

    Ide::build(&mut ide, &archive_name, &default_path).unwrap_or_else(|err| {
        println!("Unexpected error occurred {err} when trying to build IDE");
        process::exit(1)
    });

    Ide::create_symlink(&ide).unwrap_or_else(|err| {
        println!("Some error occurred {err} when trying to create symlink")
    });

    Ide::create_entry(&ide).unwrap_or_else(|err| {
        println!("Some error occurred {err}");
        process::exit(1)
    });

    Ok(())
}

