use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // &[T] はスライス、ここでは String型のスライス
    // Vec<String>の参照は &[String] に自動変換される
    // 'static は静的ライフタイム。この参照がプログラムの全期間生存できる事を意味し、文字列リテラルは全て'staticライフタイムになる
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// Box<dyn Error>は、関数がErrorトレイトを実装する型を返すことを意味する
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
