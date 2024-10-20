use std::{fs, path::MAIN_SEPARATOR};

use crate::configuration::TemplateFileParametersConf;

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

pub fn generate_from_template<S: ToString>(
    folder: &S,
    input: &S,
    parameters: &Vec<TemplateFileParametersConf>,
) -> Result<String, String> {
    let mut result: String;
    let full_path = parse_folder(folder) + &input.to_string();
    match fs::read_to_string(&full_path) {
        Ok(text) => result = text,
        Err(err) => {
            return Err(format!(
                "Could not load the file {} \nError: {}",
                &full_path,
                err.to_string()
            ))
        }
    }
    for parameter in parameters {
        let TemplateFileParametersConf { name, value } = parameter;
        let template: String = format!("{{{{{}}}}}", &name);
        result = result.replace(&template, value);
    }
    Ok(result)
}

pub fn write_text<S: ToString>(folder: &S, path: &S, text: &S) -> Result<String, String> {
    let folder = folder.to_string();
    if !is_dir(&folder) {
        if let Err(err) = fs::create_dir_all(&folder) {
            return Err(format!(
                "Could not create the folder {} \nError: {}",
                &folder,
                err.to_string()
            ));
        }
    }
    let full_path = parse_folder(&folder) + &path.to_string();
    if let Err(err) = fs::write(&full_path, text.to_string()) {
        Err(format!(
            "Could not write the file {} \nError: {}",
            &full_path,
            err.to_string()
        ))
    } else {
        Ok(full_path)
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
