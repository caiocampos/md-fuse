use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use serde_json::from_reader as read_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileConf {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub output: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateFileParametersConf {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateFileConf {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub parameters: Vec<TemplateFileParametersConf>,
    #[serde(default)]
    pub output: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conf {
    #[serde(default)]
    pub input_folder: String,
    #[serde(default)]
    pub output_folder: String,
    #[serde(default)]
    pub dictionary_subfolder: String,
    #[serde(default)]
    pub dictionary_inputs: Vec<String>,
    #[serde(default)]
    pub template_files: Vec<TemplateFileConf>,
    #[serde(default)]
    pub files: Vec<FileConf>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            input_folder: String::new(),
            output_folder: String::new(),
            dictionary_subfolder: String::new(),
            dictionary_inputs: Vec::new(),
            template_files: Vec::new(),
            files: Vec::new(),
        }
    }
}

impl Conf {
    pub fn load(path: &str) -> Result<Self, String> {
        match File::open(path) {
            Ok(file) => match read_json(BufReader::new(file)) {
                Ok(json) => Ok(Self::from(&json)),
                Err(err) => Err(format!(
                    "Could not read the file {} \nError: {}",
                    &path,
                    err.to_string()
                )),
            },
            Err(err) => Err(format!(
                "Could not load the file {} \nError: {}",
                &path,
                err.to_string()
            )),
        }
    }

    fn from(other: &Self) -> Self {
        let mut res = Self::default();
        if !other.input_folder.is_empty() {
            res.input_folder = other.input_folder.clone();
        }
        if !other.output_folder.is_empty() {
            res.output_folder = other.output_folder.clone();
        }
        if !other.dictionary_subfolder.is_empty() {
            res.dictionary_subfolder = other.dictionary_subfolder.clone();
        }
        if !other.dictionary_inputs.is_empty() {
            res.dictionary_inputs = other.dictionary_inputs.clone();
        }
        if !other.template_files.is_empty() {
            res.template_files = other.template_files.clone();
        }
        if !other.files.is_empty() {
            res.files = other.files.clone();
        }
        res
    }
}
