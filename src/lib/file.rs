use std::{collections::HashMap, env, fs, path::MAIN_SEPARATOR};

use crate::configuration::TemplateFileParametersConf;

pub fn read_dictionary_inputs<S: ToString>(
    folder: &S,
    path: &S,
) -> Result<HashMap<String, String>, String> {
    let full_path = parse_folder(folder) + &path.to_string();
    match fs::read_to_string(&full_path) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(map) => Ok(map),
            Err(err) => {
                return Err(format!(
                    "Could not process the file {} \nError: {}",
                    &full_path,
                    err.to_string()
                ));
            }
        },
        Err(err) => {
            return Err(format!(
                "Could not load the file {} \nError: {}",
                &full_path,
                err.to_string()
            ));
        }
    }
}

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
                ));
            }
        }
    }
    Ok(result)
}

pub fn generate_from_template<S: ToString>(
    folder: &S,
    path: &S,
    parameters: &Vec<TemplateFileParametersConf>,
    dictionary: &HashMap<String, String>,
) -> Result<String, String> {
    let mut result: String;
    let full_path = parse_folder(folder) + &path.to_string();
    match fs::read_to_string(&full_path) {
        Ok(text) => result = text,
        Err(err) => {
            return Err(format!(
                "Could not load the file {} \nError: {}",
                &full_path,
                err.to_string()
            ));
        }
    }
    for parameter in parameters {
        let TemplateFileParametersConf { name, value } = parameter;
        let template: String = format!("{{{{{}}}}}", &name);
        let env_prefix = "ENV_VAR:";
        let dict_prefix = "DICT_VAR:";
        if value.starts_with(env_prefix) {
            let env_key = value.strip_prefix(env_prefix).unwrap_or("");
            let env_value = match env::var(env_key) {
                Ok(res) => res,
                Err(err) => {
                    println!(
                        "Could not find the environment variable \"{}\" \nError: {}",
                        env_key,
                        err.to_string()
                    );
                    "".to_string()
                }
            };
            result = result.replace(&template, &env_value);
        } else if value.starts_with(dict_prefix) {
            let dict_key = value.strip_prefix(dict_prefix).unwrap_or("");
            let dict_value = match dictionary.get(dict_key) {
                Some(res) => res.to_string(),
                None => {
                    println!(
                        "Could not find the variable \"{}\"",
                        dict_key
                    );
                    "".to_string()
                }
            };
            result = result.replace(&template, &dict_value);
        } else {
            result = result.replace(&template, value);
        }
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
