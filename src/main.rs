mod parse;
use std::env;
use std::process;
use std::io;
use env_logger;


// Usage: echo <input_text> | your_program.sh -E <pattern>
// for full debug logs, set the RUST_LOG environment variable to "debug": RUST_LOG=codecrafters_grep=debug

fn main() {

    env_logger::Builder::from_default_env().init();

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    let match_flag = parse::Parser::match_pattern(&input_line, &pattern);
    if match_flag {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
