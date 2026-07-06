use std::collections::HashMap;

use super::{
    configuration::Conf,
    file::{generate_from_template, generate_text, read_dictionary_inputs, write_text},
};

fn print_push<S: ToString>(msgs: &mut Vec<String>, msg: &S) {
    let text = msg.to_string();
    println!("{}", text);
    msgs.push(text);
}

pub fn process(conf_file_path: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut dict_vars: HashMap<String, String> = HashMap::new();
    let config = match Conf::load(conf_file_path) {
        Ok(res) => {
            print_push(&mut result, &"Configuração lida com sucesso!");
            res
        }
        Err(err) => {
            print_push(&mut result, &err);
            Conf::default()
        }
    };
    for dict_conf in config.dictionary_inputs {
        println!("Lendo {}", dict_conf);

        match read_dictionary_inputs(
            &config.input_folder,
            &config.dictionary_subfolder,
            &dict_conf,
        ) {
            Ok(res) => {
                print_push(&mut result, &"Arquivo lido com sucesso!");

                dict_vars.extend(res);
            }
            Err(err) => {
                print_push(&mut result, &err);
                continue;
            }
        };
    }
    for file_conf in config.template_files {
        println!("Processando {}", file_conf.name);
        let text = match generate_from_template(
            &config.input_folder,
            &file_conf.input,
            &file_conf.parameters,
            &dict_vars,
        ) {
            Ok(res) => {
                print_push(&mut result, &"Arquivo lido com sucesso!");
                res
            }
            Err(err) => {
                print_push(&mut result, &err);
                continue;
            }
        };
        match write_text(&config.input_folder, &file_conf.output, &text) {
            Ok(res) => {
                print_push(
                    &mut result,
                    &format!("Arquivo {} gravado com sucesso!", res),
                );
            }
            Err(err) => {
                print_push(&mut result, &err);
            }
        }
    }
    for file_conf in config.files {
        println!("Processando {}", file_conf.name);
        let text = match generate_text(&config.input_folder, &file_conf.inputs) {
            Ok(res) => {
                print_push(&mut result, &"Arquivos lidos com sucesso!");
                res
            }
            Err(err) => {
                print_push(&mut result, &err);
                continue;
            }
        };
        match write_text(&config.output_folder, &file_conf.output, &text) {
            Ok(res) => {
                print_push(
                    &mut result,
                    &format!("Arquivo {} gravado com sucesso!", res),
                );
            }
            Err(err) => {
                print_push(&mut result, &err);
            }
        }
    }
    result
}
