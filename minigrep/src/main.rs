extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // unwrap_or_elseの引数はクロージャ
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
