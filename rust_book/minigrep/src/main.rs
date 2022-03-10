use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Ok(config) -> return the config param;
    // Err(err) -> call the clausre given err.
    // Thus: unwrap_or_else.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Ok(()) -> no action.
    // Err(e) -> print error, exit with 1.
    // Thus if let.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
