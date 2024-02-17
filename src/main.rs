use std::fs;
use std::io::ErrorKind;


fn main() -> std::io::Result<()>{
    
    println!("Creating JetBrains dir...!");

    let result = fs::create_dir("/opt/jetbrains"); 
    
    match result {
        Ok(result) => result,
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => println!("Skipping directory creation..."),
            _ => { 
                panic!("Try running with sudo: {}", error)
            }
        }
    };

    println!("Dir created!");
    
    Ok(())
}
