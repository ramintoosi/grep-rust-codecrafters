use std::env;
use std::io;
use std::process;


fn find_pattern(pattern: &str) -> (bool, String) {

    let special_chars: Vec<char> = r"\[$".chars().collect();
    let special_chars_match: Vec<char> = r"+?".chars().collect();
    let mut current_pattern = String::new();
    let mut flag_special_char = false;
    for c in pattern.chars() {
        if special_chars.contains(&c) {
            if current_pattern.len() > 0{
                // it means we already have a pattern so we should not open a new special pattern
                if c != '+' {break;}
            }
            else {
                current_pattern.push(c);
                flag_special_char = true;
            }
        }
        else if c == '^' {
            current_pattern.push(c);
            if !flag_special_char {
                flag_special_char = true;
            }
        }
        else if special_chars_match.contains(&c) {
            if current_pattern.len() == 1 {
                current_pattern.push(c);
                flag_special_char = true;
                break;
            }
            else if current_pattern.len() > 1 {
                current_pattern.pop();
                break;
            }
            else {
                // raise errot we do not know what to do
                panic!("Invalid pattern: {}", pattern);
            }
        }
        else {
            current_pattern.push(c);
            if flag_special_char {
                if current_pattern.starts_with(r"\") {
                    break;
                }
                else {
                    if c == ']' {break}
                }
            }
        }
        
    };
    (flag_special_char, current_pattern)
}

fn match_special_pattern(input_line: &str, pattern: &str) -> (bool, usize){
    if pattern.ends_with('?') {return (true, 0);}
    if pattern.starts_with('^') {
        return (input_line.starts_with(&pattern[1..].to_string()), pattern.len() - 1);
    }
    for (index, c) in input_line.chars().enumerate() {
        if pattern == r"\d" && c.is_digit(10) {return (true, index+1);}
        else if pattern == r"\w" && (c.is_alphanumeric() || c == '_') {return (true, index+1);}
        else if pattern.starts_with("[^") {if !pattern[1..pattern.len()-1].contains(c) {return (true, index+1);}}
        else if pattern.starts_with("[") {if pattern[1..pattern.len()-1].contains(c) {return (true, index+1);}}
        else if pattern.ends_with("+") && (pattern.contains(c) || pattern == ".+") {return (true, index+1);}
    }
    return (false, 0);

}

fn match_literal_pattern(input_line: &str, pattern: &str) -> (bool, usize) {
    // if let Some(start_index) = input_line.find(pattern) {
    //     return (true, start_index + pattern.len());
    // }
    // else {
    //     return (false, 0);
    // };
    for (index, _) in input_line.char_indices() {
        let mut matched_inside = true;
        for (c_input_2, c_pattern) in input_line[index..].chars().zip(pattern.chars()) {
            // println!("c_pattern: {}, c_input: {}", c_pattern, c_input_2);
            if c_pattern != c_input_2 && c_pattern != '.' {
                matched_inside = false;
                break;
            }
        }
        if matched_inside {
            return (true, index + pattern.len());
        }
    }
    return (false, 0);
}

fn match_pattern_recursive(input_line: &str, pattern: &str) -> bool {
    // println!("input_line: {}, pattern: {}", input_line.trim(), pattern.trim());
    if pattern.len() == 0 {return true;}
    if pattern == "$" {return input_line.len() == 0;}

    let (is_special_pattern, current_pattern) = find_pattern(pattern);
    let remaining_pattern = pattern[current_pattern.len()..].to_string();
    let match_flag: bool;
    let index: usize;
    // println!("current_pattern: {}, remaining_pattern: {}", current_pattern.trim(), remaining_pattern.trim());
    if is_special_pattern {
        (match_flag, index) = match_special_pattern(input_line, &current_pattern);
    }
    else {
        (match_flag, index) = match_literal_pattern(input_line, &current_pattern);
    }
    // println!("match_flag: {}, index: {}", match_flag, index);
    if match_flag {
        if input_line.len() == 0 && current_pattern.len() > 0 {
            if current_pattern.ends_with('?') {return true;}
            else {return false;}
        }
        return match_pattern_recursive(&input_line[index..], &remaining_pattern);
    }
    else {return false}
}


// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    let match_flag = match_pattern_recursive(&input_line, &pattern);
    if match_flag {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
