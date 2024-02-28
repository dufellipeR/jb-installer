use std::io::{self};
use std::path::{PathBuf};
use std::process;

use clap::Parser;
use recolored::Colorize;
use sudo::{check, RunningAs};

use painite::{create_directory, Ide, unpack_tar};

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
            println!("> The application needs SUDO permission to work properly\n> Try running with \"sudo\" command");
            process::exit(1);
        },
    }

    let default_jetbrains_path_directory = "/opt/JetBrains";
    let default_symlink_path = "/usr/local/bin/";
    let default_entry_path = "/usr/share/applications/";

    create_directory(&default_jetbrains_path_directory).unwrap_or_else(|err| {
        println!("Unexpected error occurred {err} when trying to create directory");
        process::exit(1)
    });

    let archive_name = unpack_tar(&args.gz_path, &default_jetbrains_path_directory)?;

    let mut ide = Ide::new();

    ide.build(&archive_name, &default_jetbrains_path_directory).unwrap_or_else(|err| {
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
    println!("> {} {} installed", &ide.get_name().hex_color(0xc7059c), "successfully".green());

    Ok(())
}

fn greeting() {
    println!("> Welcome to {} !", "Painite".true_color(128, 0,0).bold())
}

