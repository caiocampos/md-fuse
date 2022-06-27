use super::configuration::Conf;
use super::file::{generate_text, write_text};

fn print_push(msgs: &mut Vec<String>, msg: String) {
    println!("{}", msg);
    msgs.push(msg);
}

pub fn process(conf_file: &str) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let config = match Conf::load(conf_file) {
        Ok(res) => {
            print_push(&mut result, "Configuração lida com sucesso!".into());
            res
        }
        Err(err) => {
            print_push(&mut result, err);
            Conf::default()
        }
    };
    for file_conf in config.files {
        println!("Processando {}", file_conf.name);
        let text = match generate_text(&config.input_folder, &file_conf.inputs) {
            Ok(res) => {
                print_push(&mut result, "Arquivos lidos com sucesso!".into());
                res
            }
            Err(err) => {
                print_push(&mut result, err);
                continue;
            }
        };
        match write_text(&config.output_folder, &file_conf.output, &text) {
            Ok(res) => {
                print_push(&mut result, format!("Arquivo {} gravado com sucesso!", res));
            }
            Err(err) => {
                print_push(&mut result, err);
            }
        }
    }
    result
}
