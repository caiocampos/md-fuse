use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use crate::configuration::TemplateFileParametersConf;

macro_rules! merge_path {
    ($( $element:expr ),+ $(,)?) => {
        merge_path(&[ $( $element ),+ ])
    };
}

pub fn read_dictionary_inputs<P: AsRef<Path>>(
    folder: &P,
    subfolder: &P,
    path: &P,
) -> Result<HashMap<String, String>, String> {
    let full_path = merge_path!(folder, subfolder, path);
    match fs::read_to_string(&full_path) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(map) => Ok(map),
            Err(err) => {
                return Err(format!(
                    "Could not process the file {} \nError: {}",
                    full_path.display(),
                    err.to_string()
                ));
            }
        },
        Err(err) => {
            return Err(format!(
                "Could not load the file {} \nError: {}",
                full_path.display(),
                err.to_string()
            ));
        }
    }
}

pub fn generate_text<P: AsRef<Path>>(folder: &P, inputs: &Vec<P>) -> Result<String, String> {
    let mut result = String::new();
    for path in inputs {
        let full_path = merge_path!(folder, path);
        match fs::read_to_string(&full_path) {
            Ok(text) => result += &text,
            Err(err) => {
                return Err(format!(
                    "Could not load the file {} \nError: {}",
                    full_path.display(),
                    err.to_string()
                ));
            }
        }
    }
    Ok(result)
}

pub fn generate_from_template<P: AsRef<Path>>(
    folder: &P,
    path: &P,
    parameters: &Vec<TemplateFileParametersConf>,
    dictionary: &HashMap<String, String>,
) -> Result<String, String> {
    let mut result: String;
    let full_path = merge_path!(folder, path);
    match fs::read_to_string(&full_path) {
        Ok(text) => result = text,
        Err(err) => {
            return Err(format!(
                "Could not load the file {} \nError: {}",
                full_path.display(),
                err.to_string()
            ));
        }
    }
    for parameter in parameters {
        let TemplateFileParametersConf { name, value } = parameter;
        let template: String = format!("{{{{{}}}}}", &name);
        let dict_prefix = "DICT_VAR:";
        let env_prefix = "ENV_VAR:";
        if value.starts_with(dict_prefix) {
            let dict_key = value.strip_prefix(dict_prefix).unwrap_or("");
            let dict_value = match dictionary.get(dict_key) {
                Some(res) => res.to_string(),
                None => "".to_string(),
            };
            result = result.replace(&template, &dict_value);
        } else if value.starts_with(env_prefix) {
            let env_key = value.strip_prefix(env_prefix).unwrap_or("");
            let env_value = match env::var(env_key) {
                Ok(res) => res,
                Err(_) => "".to_string(),
            };
            result = result.replace(&template, &env_value);
        } else {
            result = result.replace(&template, value);
        }
    }
    Ok(result)
}

pub fn write_text<P: AsRef<Path>>(folder: &P, path: &P, text: &String) -> Result<String, String> {
    if !is_dir(&folder) {
        if let Err(err) = fs::create_dir_all(&folder) {
            return Err(format!(
                "Could not create the folder {} \nError: {}",
                folder.as_ref().display(),
                err.to_string()
            ));
        }
    }
    let full_path = merge_path!(folder, path);
    if let Err(err) = fs::write(&full_path, text) {
        Err(format!(
            "Could not write the file {} \nError: {}",
            full_path.display(),
            err.to_string()
        ))
    } else {
        Ok(full_path.to_string_lossy().into())
    }
}

fn is_dir<P: AsRef<Path>>(folder: &P) -> bool {
    if let Ok(metadata) = fs::metadata(folder) {
        metadata.is_dir()
    } else {
        false
    }
}

fn merge_path<P: AsRef<Path>>(parts: &[P]) -> PathBuf {
    parts
        .iter()
        .filter(|part| !part.as_ref().as_os_str().is_empty())
        .collect()
}
