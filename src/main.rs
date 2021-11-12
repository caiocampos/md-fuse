mod lib;

use lib::configuration::Conf;
use lib::file::generate_text;
use lib::file::write_text;

pub const CONF_FILE: &str = "md.conf.json";

fn main() {
    let config = Conf::load(CONF_FILE);
    for file_conf in config.files {
        println!("Processando {}", file_conf.name);
        let text = match generate_text(&config.input_folder, &file_conf.inputs) {
            Ok(res) => {
                println!("Arquivos lidos com sucesso!");
                res
            }
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        match write_text(&config.output_folder, &file_conf.output, &text) {
            Ok(res) => println!("Arquivo {} gravado com sucesso!", res),
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
