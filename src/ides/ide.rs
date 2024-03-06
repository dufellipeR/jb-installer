use std::fs::File;
use std::io::{ErrorKind, Write};
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process;
use recolored::Colorize;
use crate::utils::{detect_version, generate_entry};


pub trait IDE {

    fn build(&mut self, archive_name: &String, main_dir_path: &Path) -> Result<(), &'static str> {
        let version = detect_version(archive_name);
        self.set_version(version);
        self.set_icon(format!("{}/{}bin/{}.png", main_dir_path.to_string_lossy(), archive_name, self.get_short_name()));
        self.set_exec(format!("{}/{}bin/{}.sh", main_dir_path.to_string_lossy(), archive_name, self.get_short_name()));

        Ok(())
    }

    fn create_symlink(&self, default_symlink_path: &Path) -> Result<(), &'static str> {
        let path = default_symlink_path.join(&self.get_short_name());

        match symlink(&self.get_exec(), &path) {
            Ok(_) => println!("> ✅ {} created symbolic link", "successfully".green()),
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => println!("> ✅ skipping symlink creation..."),
                ErrorKind::PermissionDenied => {
                    println!("> no permission to create symlink, try running with \"sudo\"");
                    println!("> ✅ skipping symlink creation...");
                }
                _err => return Err("> unknown error when creating symlink")
            }
        }

        Ok(())
    }

    fn create_entry(&self, default_entry_path: &Path) -> Result<(), &'static str>{
        let filename = format!("{}{}.desktop", default_entry_path.to_string_lossy(), self.get_short_name());
        let mut file = File::create(&filename).unwrap_or_else(|err| {
            println!("> unexpected error occurred {err}");
            process::exit(1)
        });

        //creating and formatting entry content
        let entry = generate_entry(self.get_entries());

        match file.write_all(entry.as_ref()) {
            Ok(_) => println!("> ✅ {} created desktop entry", "successfully".green()),
            Err(_) => return Err("> unexpected error when writing IDE entry")
        };

        Ok(())
    }

    fn get_name(&self) -> &String;
    fn get_comment(&self) -> &String;
    fn get_version(&self) -> &String;
    fn get_short_name(&self) -> &String;
    fn get_exec(&self) -> &String;
    fn get_icon(&self) -> &String;
    fn get_entries(&self) -> &Entry;

    fn set_version(&mut self, version: String);
    fn set_icon(&mut self, icon_path: String);
    fn set_exec(&mut self, exec_path: String);


}
pub struct Entry {
    pub name: String,
    pub comment: String,
    pub icon: String,
    pub exec: String,
    pub version: String,
    pub short_name: String,
}



// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_successfully_returns_name() {
//         let archive_name = "RustRover-2023.0.1/";
//         let path = "/opt/JetBrains";
//         let mut ide = Ide::new();
//
//         assert!(&ide.build(archive_name, path).is_ok());
//         assert_eq!("Rust Rover", &ide.get_name().to_owned());
//     }
//     #[test]
//     fn it_successfully_build_rustrover() {
//         let archive_name = "RustRover-2023.0.1/";
//         let path = "/opt/JetBrains";
//         let mut ide = Ide::new();
//
//         assert!(&ide.build(archive_name, path).is_ok());
//         assert_eq!(ide.short_name, "rustrover");
//         assert_eq!(ide.exec, "/opt/JetBrains/RustRover-2023.0.1/bin/rustrover.sh");
//         assert_eq!(ide.icon, "/opt/JetBrains/RustRover-2023.0.1/bin/rustrover.png");
//
//     }
//
//     #[test]
//     fn it_successfully_build_idea() {
//         let archive_name = "idea-IU-2023.0.1/";
//         let path = "/opt/JetBrains";
//         let mut ide = Ide::new();
//
//         assert!(&ide.build(archive_name, path).is_ok());
//         assert_eq!(ide.short_name, "idea");
//         assert_eq!(ide.exec, "/opt/JetBrains/idea-IU-2023.0.1/bin/idea.sh");
//         assert_eq!(ide.icon, "/opt/JetBrains/idea-IU-2023.0.1/bin/idea.png");
//
//     }
//
//     #[test]
//     fn it_successfully_build_pycharm() {
//         let archive_name = "PyCharm-2023.0.1/";
//         let path = "/opt/JetBrains";
//         let mut ide = Ide::new();
//
//         assert!(&ide.build(archive_name, path).is_ok());
//         assert_eq!(ide.short_name, "pycharm");
//         assert_eq!(ide.exec, "/opt/JetBrains/PyCharm-2023.0.1/bin/pycharm.sh");
//         assert_eq!(ide.icon, "/opt/JetBrains/PyCharm-2023.0.1/bin/pycharm.png");
//
//     }
//
//     #[test]
//     fn it_successfully_build_goland() {
//         let archive_name = "GoLand-2023.0.1/";
//         let path = "/opt/JetBrains";
//         let mut ide = Ide::new();
//
//         assert!(&ide.build(archive_name, path).is_ok());
//         assert_eq!(ide.short_name, "goland");
//         assert_eq!(ide.exec, "/opt/JetBrains/GoLand-2023.0.1/bin/goland.sh");
//         assert_eq!(ide.icon, "/opt/JetBrains/GoLand-2023.0.1/bin/goland.png");
//
//     }
//
//     #[test]
//     fn it_successfully_detect_rustrover(){
//         assert_eq!((String::from("rustrover"), String::from("1.0")), Ide::detect_ide("RustRover-1.0").unwrap());
//     }
//
//     #[test]
//     fn it_successfully_detect_idea(){
//         assert_eq!((String::from("idea"), String::from("1.0")), Ide::detect_ide("Idea-IU-1.0").unwrap());
//     }
//
//     #[test]
//     fn it_successfully_detect_goland(){
//         assert_eq!((String::from("goland"), String::from("1.0")), Ide::detect_ide("GoLand-1.0").unwrap());
//     }
//
//     #[test]
//     fn it_successfully_detect_pycharm(){
//         assert_eq!((String::from("pycharm"), String::from("1.0")), Ide::detect_ide("PyCharm-1.0").unwrap());
//     }
//
//     #[test]
//     fn it_throws_error_unknown_ide(){
//         assert!(Ide::detect_ide("unknown-ide").is_err());
//     }
// }


