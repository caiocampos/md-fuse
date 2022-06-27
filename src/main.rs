mod lib;

use lib::process;

pub const CONF_FILE: &str = "md.conf.json";

fn main() {
    process(&CONF_FILE);
}
