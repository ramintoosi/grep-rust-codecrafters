use std::env;
use std::io;
use std::process;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Expr {
    Literal(String),
    Dot,
    AnchorStart,
    AnchorEnd,
    Seq(Vec<Expr>),
    Empty
}

struct Parser<'a> {
    chars: Peekable<Chars<'a>>
}

impl<'a> Parser<'a> {

    fn new(pattern: &'a str) -> Self {
        Self {chars: pattern.chars().peekable()}
    }

    fn next(&mut self) -> Option<char>{
        self.chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn parse_literal(&mut self) -> String {
        let mut literal = String::new();

        let specials = ".^$+?|()[]\\";

        while let Some(c) = self.peek() {
            if specials.contains(c) {
                break;
            }
            literal.push(c);
            self.next();
        }

        literal
    }

    fn parse_dot(&mut self) -> Option<String> {
        if let Some(c) = self.peek() {
            if c == '.' {
                self.next();
                Some(".".to_string())
            }
            else {
                None
            }
        }
        else {None}
    }

    fn parse_char_class(&mut self) -> Option<String> {
        if let Some(c) = self.peek() {
            if c == '[' {
                let mut result = String::new();
                while let Some(c) = self.next() {
                    result.push(c);
                    if c == ']' {
                        return Some(result);
                    }
                }
                panic!("Unclosed character class");
            }
            else {None}
        }
        else {None}
    }

    fn parse_quantifier(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            if c == '+' || c == '?' {
                self.next();
                return Some(c)
            }
        }
        None
    }
    
}



fn main() {}

#[cfg(test)]
mod tests {
    use super::*; // import Parser

    #[test]
    fn test_parse_literal_basic() {
        let mut parser = Parser::new("hello+world");
        let literal = parser.parse_literal();
        assert_eq!(literal, "hello");
        assert_eq!(parser.chars.collect::<String>(), "+world");
    }

    #[test]
    fn test_parse_literal_with_special_chars() {
        let mut parser = Parser::new("abc.def");
        let literal = parser.parse_literal();
        assert_eq!(literal, "abc"); // stops before '.'
    }

    #[test]
    fn test_parse_literal_full_literal() {
        let mut parser = Parser::new("simpletext");
        let literal = parser.parse_literal();
        assert_eq!(literal, "simpletext"); // consumes entire input
    }
    #[test]
    fn test_parse_dot_when_dot_present() {
        let mut parser = Parser::new(".abc");
        let dot = parser.parse_dot();
        assert_eq!(dot, Some(".".to_string()));
        assert_eq!(parser.chars.collect::<String>(), "abc"); // dot consumed
    }
    
    #[test]
    fn test_parse_dot_when_no_dot() {
        let mut parser = Parser::new("abc");
        let dot = parser.parse_dot();
        assert_eq!(dot, None);
        assert_eq!(parser.chars.collect::<String>(), "abc"); // nothing consumed
    }
    #[test]
    fn test_parse_char_class_basic() {
        let mut parser = Parser::new("[abc]+");
        let cls = parser.parse_char_class();
        assert_eq!(cls, Some("[abc]".to_string()));
        assert_eq!(parser.chars.collect::<String>(), "+");
    }

    #[test]
    fn test_parse_char_class_negated() {
        let mut parser = Parser::new("[^xyz]end");
        let cls = parser.parse_char_class();
        assert_eq!(cls, Some("[^xyz]".to_string()));
        assert_eq!(parser.chars.collect::<String>(), "end");
    }

    #[test]
    #[should_panic(expected = "Unclosed character class")]
    fn test_parse_char_class_unclosed() {
        let mut parser = Parser::new("[abc");
        let _ = parser.parse_char_class(); // should panic
    }

    #[test]
    fn test_parse_quantifier_plus() {
        let mut parser = Parser::new("+next");
        let q = parser.parse_quantifier();
        assert_eq!(q, Some('+'));
        assert_eq!(parser.chars.collect::<String>(), "next");
    }

    #[test]
    fn test_parse_quantifier_question() {
        let mut parser = Parser::new("?abc");
        let q = parser.parse_quantifier();
        assert_eq!(q, Some('?'));
        assert_eq!(parser.chars.collect::<String>(), "abc");
    }

    #[test]
    fn test_parse_quantifier_none() {
        let mut parser = Parser::new("abc");
        let q = parser.parse_quantifier();
        assert_eq!(q, None);
        assert_eq!(parser.chars.collect::<String>(), "abc"); // unchanged
    }
}


// fn find_pattern(pattern: &str) -> (bool, String, usize) {

//     let mut current_pattern = String::new();
//     let mut flag_special_char = false;
//     let pattern_size = pattern.len();

//     // let all_special_chars: Vec<char> = r"[($+?\".chars().collect();
//     let opening_special_chars: Vec<char> = r"[(".chars().collect();

//     let mut pattern_output_len: usize = 0;

//     'mainloop: for (index, c) in pattern.char_indices() {

//         if c == '\\' {
//             if index + 1 < pattern_size {
//                 if current_pattern.len() > 0 {
//                     break;
//                 }
//                 current_pattern = pattern[index..index+2].to_string();
//                 flag_special_char = true;
//                 if (index + 2 <= pattern_size - 1) && (pattern.chars().nth(index+2).unwrap() == '+' || pattern.chars().nth(index+2).unwrap() == '?') {
//                     let second_special_char = pattern.chars().nth(index+2).unwrap();
//                     if second_special_char == '+' {
//                         pattern_output_len = 3;
//                         break;
//                     }
//                     else if second_special_char == '?' {
//                         pattern_output_len = 3;
//                         current_pattern = "".to_string();
//                         break;
//                     }
//                 }
//                 break;
//             }
//             else {
//                 panic!("Invalid pattern: {}", pattern);
//             }
//         }
//         else if opening_special_chars.contains(&c) {
//             if current_pattern.len() > 0 {
//                 break;
//             }
//             for (index_2, c_2) in pattern[index..].chars().enumerate() {
//                 flag_special_char = true;
//                 current_pattern.push(c_2);
//                 if c_2 == ')' || c_2 == ']' {
//                     if index_2 + 1 < pattern_size {
//                         if pattern.chars().nth(index + index_2 + 1).unwrap() == '+' {
//                             pattern_output_len = current_pattern.len() + 1;
//                             break 'mainloop;
//                         }
//                         else if pattern.chars().nth(index + index_2 + 1).unwrap() == '?' {
//                             pattern_output_len = current_pattern.len() + 1;
//                             current_pattern = "".to_string();
//                         }
                    
//                     }
//                     break 'mainloop;
//                 }   
                
//             }
//         }
//         else if c == '^' {
//             if current_pattern.len() > 0 {
//                 break;
//             }
//             flag_special_char = true;
//             current_pattern.push(c)
//         }
//         else if c == '$' {
//             flag_special_char = true;
//             current_pattern.push(c);
//             break;
//         }
//         else if c == '+' || c == '?' {
//             if current_pattern.len() > 0 {
//                 pattern_output_len = current_pattern.len() + 1;
//                 // Append the quantifier to the pattern (e.g., "s" becomes "s?")
//                 current_pattern.push(c);
//                 flag_special_char = true;
//                 break;
//             }
//             else {
//                 current_pattern = "".to_string();
//                 pattern_output_len = 1;
//             }
//         }
//         else {
//             current_pattern.push(c);
//         }
//     }
//     if current_pattern.len() > 0 && pattern_output_len == 0 {
//         pattern_output_len = current_pattern.len();
//     }
//     (flag_special_char, current_pattern, pattern_output_len)
// }

// fn match_special_pattern(input_line: &str, pattern: &str) -> (bool, usize){
//     // Handle ? quantifier: match 0 or 1 times (greedy - prefer 1)
//     if pattern.ends_with('?') {
//         let base_pattern = &pattern[..pattern.len()-1];
//         // Try to match once first (greedy)
//         let (matched, consumed) = match_literal_pattern(input_line, base_pattern);
//         if matched && consumed > 0 {
//             return (true, consumed);  // Successfully matched 1 time
//         } else {
//             return (true, 0);  // Match 0 times (it's optional)
//         }
//     }
//     if pattern.starts_with('^') {
//         return (input_line.starts_with(&pattern[1..].to_string()), pattern.len() - 1);
//     }
//     for (index, c) in input_line.chars().enumerate() {
//         if pattern == r"\d" && c.is_digit(10) {return (true, index+1);}
//         else if pattern == r"\w" && (c.is_alphanumeric() || c == '_') {return (true, index+1);}
//         else if pattern.starts_with("[^") {if !pattern[1..pattern.len()-1].contains(c) {return (true, index+1);}}
//         else if pattern.starts_with("[") {if pattern[1..pattern.len()-1].contains(c) {return (true, index+1);}}
//         else if pattern.ends_with("+") && (pattern.contains(c) || pattern == ".+") {return (true, index+1);}
//     }
//     return (false, 0);

// }

// fn match_literal_pattern(input_line: &str, pattern: &str) -> (bool, usize) {
//     // println!("match_literal_pattern: input_line: <{}>, pattern: <{}>", input_line, pattern);
//     if input_line.len() < pattern.len() {return (false, 0)}
//     if pattern == "" {return (true, 0);}
//     for (index, _) in input_line.char_indices() {
//         let mut matched_inside = true;
//         if input_line[index..].len() < pattern.len() {return (false, 0);}
//         for (c_input_2, c_pattern) in input_line[index..].chars().zip(pattern.chars()) {
//             if c_pattern != c_input_2 && c_pattern != '.' {
//                 matched_inside = false;
//                 break;
//             }
//         }
//         if matched_inside {
//             return (true, index + pattern.len());
//         }
//     }
//     return (false, 0);
// }

// fn match_pattern_recursive(input_line: &str, pattern: &str) -> (bool, usize) {
//     if pattern == "$" {return (input_line.len() == 0, 0);}
//     let (is_special_pattern, current_pattern, pattern_output_len) = find_pattern(pattern);
//     let remaining_pattern = pattern[pattern_output_len..].to_string();
//     let mut match_flag: bool;
//     let mut index: usize;
//     println!("input_line: <{}>, pattern: <{}>, is_special_pattern: <{}>, current_pattern: <{}>, remaining_pattern: <{}>", input_line, pattern, is_special_pattern, current_pattern, remaining_pattern);
//     if is_special_pattern {
//         if current_pattern.starts_with('(') {
//             let pattern_split = current_pattern[1..current_pattern.len()-1].split('|');
//             match_flag = false;
//             index = 0;
//             for pattern in pattern_split {
//                 (match_flag, index) = match_pattern_recursive(input_line, pattern);
//                 if match_flag {
//                     break;
//                 }
//             }
//         }
//         else {
//             (match_flag, index) = match_special_pattern(input_line, &current_pattern);
//         }
//     }
//     else {
//         (match_flag, index) = match_literal_pattern(input_line, &current_pattern);
//     }
//     // println!("match_flag: {}, index: {}", match_flag, index);
//     if match_flag {
//         if input_line.len() == 0 && current_pattern.len() > 0 {
//             if current_pattern.ends_with('?') {return (true, index);}
//             else {return (false, 0);}
//         }
//         if remaining_pattern.len() == 0 {return (true, index);}
//         return match_pattern_recursive(&input_line[index..], &remaining_pattern);
//     }
//     else {return (false, 0);}
    
// }


// // Usage: echo <input_text> | your_program.sh -E <pattern>
// fn main() {

//     if env::args().nth(1).unwrap() != "-E" {
//         println!("Expected first argument to be '-E'");
//         process::exit(1);
//     }

//     let pattern = env::args().nth(2).unwrap();
//     let mut input_line = String::new();

//     io::stdin().read_line(&mut input_line).unwrap();

//     // Uncomment this block to pass the first stage
//     let (match_flag, _) = match_pattern_recursive(&input_line, &pattern);
//     if match_flag {
//         process::exit(0)
//     } else {
//         process::exit(1)
//     }
// }
