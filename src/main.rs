use std::env;
use std::process;

use rustivity::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "From {} to {}. DPI: {}, Sensitivity: {}",
        config.origin, config.target, config.dpi, config.sensitivity
    );

    if let Err(e) = rustivity::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
