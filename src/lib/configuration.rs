use std::fs::File;
use std::io::BufReader;

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
pub struct Conf {
    #[serde(default)]
    pub input_folder: String,
    #[serde(default)]
    pub output_folder: String,
    #[serde(default)]
    pub files: Vec<FileConf>,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            input_folder: "".into(),
            output_folder: "".into(),
            files: [].into(),
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
        if !other.files.is_empty() {
            res.files = other.files.clone();
        }
        res
    }
}
