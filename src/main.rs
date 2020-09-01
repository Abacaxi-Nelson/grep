use std::env;
use std::process;

mod lib;
use lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap();

    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

