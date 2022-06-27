use std::{fs, path::MAIN_SEPARATOR};

pub fn generate_text<S: ToString>(folder: &S, inputs: &Vec<S>) -> Result<String, String> {
    let mut result = String::new();
    for path in inputs {
        let full_path = parse_folder(folder) + &path.to_string();
        match fs::read_to_string(&full_path) {
            Ok(text) => result += &text,
            Err(err) => {
                return Err(format!(
                    "Could not load the file {} \nError: {}",
                    &full_path,
                    err.to_string()
                ))
            }
        }
    }
    Ok(result)
}

pub fn write_text<S: ToString>(folder: &S, path: &S, text: &S) -> Result<String, String> {
    let folder = folder.to_string();
    if !is_dir(&folder) {
        match fs::create_dir_all(&folder) {
            Ok(()) => {}
            Err(err) => {
                return Err(format!(
                    "Could not create the folder {} \nError: {}",
                    &folder,
                    err.to_string()
                ))
            }
        }
    }
    let full_path = parse_folder(&folder) + &path.to_string();
    match fs::write(&full_path, text.to_string()) {
        Ok(()) => Ok(full_path),
        Err(err) => Err(format!(
            "Could not write the file {} \nError: {}",
            &full_path,
            err.to_string()
        )),
    }
}

fn is_dir<S: ToString>(folder: &S) -> bool {
    if let Ok(metadata) = fs::metadata(folder.to_string()) {
        metadata.is_dir()
    } else {
        false
    }
}

fn parse_folder<S: ToString>(folder: &S) -> String {
    let mut folder = folder.to_string();
    if !folder.ends_with(MAIN_SEPARATOR) {
        folder.push(MAIN_SEPARATOR);
    }
    folder
}
