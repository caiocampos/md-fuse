use md_fuse::{CONF_FILE, process};

const DEBUG: bool = true;

fn main() {
    let log = process(&CONF_FILE);
    if DEBUG {
        log.iter().for_each(|msg| println!("{}", msg))
    }
}
