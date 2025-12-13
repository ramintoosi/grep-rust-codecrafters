mod parse;
use std::env;
use std::process;
use std::io;
use env_logger;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


// Usage: echo <input_text> | your_program.sh -E <pattern>
// for full debug logs, set the RUST_LOG environment variable to "debug": RUST_LOG=codecrafters_grep=debug

fn main() {

    env_logger::init();

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();

    let mut match_flag: bool;

    if env::args().nth(3).is_some() {
        // get all args after the second one
        let file_paths = env::args().skip(2).collect::<Vec<String>>();
        match_flag = false;
        for file_path in file_paths {
            let file = File::open(&file_path).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line_str = line.unwrap();
                let match_flag_single_line = parse::Parser::match_pattern(&line_str, &pattern);
                if match_flag_single_line {
                    match_flag = true;
                        println!("{}:{}", &file_path, line_str);
                }
            }
        }
    }
    else {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        match_flag = parse::Parser::match_pattern(&input_line, &pattern);
    }
    
    if match_flag {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
