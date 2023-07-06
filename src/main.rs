use glob::glob;
use std::{fs, io};

fn decorate_path(path: &str) -> String {
    let decorated_path: String;

    if path.starts_with("/") {
        decorated_path = ".".to_string() + path;
    } else if path.starts_with("./") {
        decorated_path = path.to_string();
    } else if !path.is_empty(){
        decorated_path = "./".to_string() + path;
    } else {
        decorated_path = "".to_string();
    }
    decorated_path
}

fn remove_file_or_folder(path: &str)-> io::Result<()> {
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        fs::remove_file(path)?;
    } else if metadata.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        eprintln!("Path is neither a file nor a folder: {}", path);
    }

    Ok(())
}

fn get_paths_from_path(line: &str) {
    let customised_line = decorate_path(line);
    for entry in glob(&customised_line).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Err(e) = remove_file_or_folder(path.to_str().unwrap()) {
                    eprintln!("Failed to remove file or folder: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to retrieve entry: {}", e);
            }
        }
    }
}

fn main() {
    let gitignore_path = ".gitignore";

    match fs::read_to_string(gitignore_path) {
        Ok(contents) => {
            let lines = contents.lines();
            for line in lines {
                get_paths_from_path(line);
            }
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }

}
