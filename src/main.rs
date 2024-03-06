mod utils;
mod ides;

use std::io::{self};
use std::path::{Path, PathBuf};
use std::process;
use clap::Parser;

use recolored::Colorize;
use sudo::{check, RunningAs};
use crate::ides::IDE;
use crate::utils::{create_directory, detect_ide, greeting, unpack_tar};


#[derive(Parser)]
struct Cli {
    gz_path: PathBuf
}

fn main() -> io::Result<()>{
    let args = Cli::parse();

    // welcome message
    greeting();

    // checking privileges
    match check() {
        RunningAs::Root => {},
        _ => {
            println!("> the application needs SUDO permission to work properly\n> Try running with \"sudo\" command");
            process::exit(1);
        },
    }

    let default_jetbrains_path_directory = Path::new("/opt/JetBrains");
    let default_symlink_path = Path::new("/usr/bin/");
    let default_entry_path = Path::new("/usr/share/applications/");

    create_directory(&default_jetbrains_path_directory).unwrap_or_else(|err| {
        println!("Unexpected error occurred {err} when trying to create directory");
        process::exit(1)
    });

    let archive_name = unpack_tar(&args.gz_path, &default_jetbrains_path_directory)?;

    let mut ide: Box<dyn IDE> = detect_ide(&archive_name).unwrap_or_else(|err| {
        println!("> unexpected error occurred {err} when trying to detect ide");
        process::exit(1);
    });

    ide.build(&archive_name, default_jetbrains_path_directory).unwrap_or_else(|err| {
        println!("> Unexpected error occurred {err} when trying to build IDE");
        process::exit(1)
    });

    ide.create_symlink(default_symlink_path).unwrap_or_else(|err| {
        println!("> Unexpected error occurred {err} when trying to create symlink");
    });

    ide.create_entry(default_entry_path).unwrap_or_else(|err| {
        println!("> Unexpected error occurred {err} when trying to create entry");
        process::exit(1)
    });

    println!("-----------------------------------------------");
    println!("> {} {} installed", &ide.get_name().hex_color(ide.get_color()).bold(), "successfully".green());

    Ok(())
}


