use std::env;
use std::process;

use dummy_grep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    if let Err(e) = dummy_grep::run(config) {
        eprintln!("Aplication error: {e}");
        process::exit(1);
    }
}

