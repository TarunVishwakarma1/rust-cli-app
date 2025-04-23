use std::env;
use std::process;
use rust_cli_app::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("searching for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = rust_cli_app::run(config){
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}



