mod parse;
use std::env;
use std::process;
use std::io;
use env_logger;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;
use std::path::Path;
use std::fs;


// Usage: echo <input_text> | your_program.sh -E <pattern>
// for full debug logs, set the RUST_LOG environment variable to "debug": RUST_LOG=codecrafters_grep=debug
struct PathIterator {
    queue: VecDeque<String>,
    is_recursive: bool,
}

impl PathIterator {
    fn new(args: VecDeque<String>, is_recursive: bool) -> Self {
        Self { queue: args, is_recursive }
    }
        
}

impl Iterator for PathIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.queue.pop_front() {
            let path = Path::new(&entry);
            if path.is_file() {
                return Some(entry);
            }

            if path.is_dir() {
                if !self.is_recursive {
                    panic!("Expected file or -r for directory");
                }

                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        self.queue.push_back(entry.path().to_str().unwrap().to_string());
                    }
                }
                continue;
            }

            panic!("Invalid path");
        }

        None
    }
}




fn main() {

    env_logger::init();

    let valid_args: Vec<String> = vec!["-E".to_string(), "-r".to_string()];
    let mut args = env::args().collect::<VecDeque<String>>();
    args.pop_front();

    let first_arg = args.pop_front().expect("Expected at least one argument");
    let is_recursive: bool;
    if !valid_args.contains(&first_arg) {
        eprintln!("Expected first argument to be '-E' or '-r'");
        process::exit(1);
    }
    else {
        is_recursive = first_arg == "-r";
    }

    if is_recursive {
        let second_arg = args.pop_front().expect("Expected -E argument");
        if second_arg != "-E" {
            eprintln!("Expected second argument to be '-E'");
            process::exit(1);
        }
    }

    let pattern = args.pop_front().expect("Expected pattern argument");
    let mut match_flag: bool = false;

    if !args.is_empty() {
        let if_print_filename: bool = is_recursive || args.len() > 1;
        PathIterator::new(args.clone(), is_recursive).for_each(|path| {
            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line_str = line.unwrap();
                let match_flag_single_line = parse::Parser::match_pattern(&line_str, &pattern);
                if match_flag_single_line {
                    match_flag = true;
                    if if_print_filename {
                        println!("{}:{}", &path, line_str);
                    }
                    else {
                        println!("{}", line_str);
                    }
                }
            }
        })
        
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
