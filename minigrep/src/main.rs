use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    // コマンドライン引数を取って、ベクタに変換する
    let args: Vec<String> = env::args().collect();
    // unwrap_or_elseの引数はクロージャ
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // &[T] はスライス、ここでは String型のスライス
    // Vec<String>の参照は &[String] に自動変換される
    // 'static は静的ライフタイム。この参照がプログラムの全期間生存できる事を意味し、文字列リテラルは全て'staticライフタイムになる
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// Box<dyn Error>は、関数がErrorトレイトを実装する型を返すことを意味する
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
