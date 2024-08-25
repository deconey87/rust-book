extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // コマンドライン引数を取って、ベクタに変換する
    let args: Vec<String> = env::args().collect();
    // unwrap_or_elseの引数はクロージャ
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
