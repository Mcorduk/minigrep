use std::{env::args, fs};
use std::process;
use Config;
fn main() {

    let args: Vec<String> = args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read file");
}


