use std::iter::Peekable;
use std::str::Chars;

pub struct Parser<'a> {
    pub chars: Peekable<Chars<'a>>
}

impl<'a> Parser<'a> {

    pub fn new(pattern: &'a str) -> Self {
        Self {chars: pattern.chars().peekable()}
    }

    pub fn next(&mut self) -> Option<char>{
        self.chars.next()
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn parse_literal(&mut self) -> String {
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

    pub fn parse_dot(&mut self) -> Option<String> {
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

    pub fn parse_char_class(&mut self) -> Option<String> {
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

    pub fn parse_quantifier(&mut self) -> Option<char> {
        if let Some(c) = self.peek() {
            if c == '+' || c == '?' {
                self.next();
                return Some(c)
            }
        }
        None
    }

    pub fn parse_parentheses(&mut self) -> Option<String> {
        if let Some(c) = self.peek() {
            let mut depth = 0;
            if c == '(' {
                let mut result = String::new();
                while let Some(c) = self.next() {
                    result.push(c);
                    if c == ')' {
                        depth -= 1;
                        if depth == 0 {
                            return Some(result);
                        }
                    }
                    else if c == '(' {
                        depth += 1;
                    }
                }
                panic!("Unclosed parentheses");
            }
        }
        None
    }

    pub fn split_alternatives(group: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut alter = String::new();
        let mut depth = 0;
        for c in group[1..group.len() - 1].chars() {
            alter.push(c);
            if c == '|' && depth == 0 {
                alter.pop();
                result.push(alter);
                alter = String::new();
            }
            else if c == '(' {
                depth += 1;
            }
            else if c == ')' {
                depth -= 1;
            }
        }
        if !alter.is_empty() {result.push(alter);}
        result
    }

    pub fn match_pattern(input: &str, pattern: &str) -> bool {
        let mut parser = Parser::new(pattern);
        let (flag, _) = Self::match_pattern_internal(input, &mut parser);
        println!("-------> match pattern done flag: {}", flag);
        flag
    }

    fn match_pattern_internal(input: &str, parser: &mut Parser) -> (bool, i32) {
        println!("-> input: {}, pattern: {:?}", input, &parser.chars.clone().collect::<String>());

        if input.is_empty() && !parser.peek().is_none() {
            return (false, 0);
        }

        if parser.peek().is_none() {
            return (true, 0);
        }

        let mut token: Option<String> = None;
        let mut input_check = input.clone().to_string();
        let mut len_input_checked: usize = 0;


        if let Some(token) = parser.parse_char_class() {
            let mut flag = false;
            for c in input_check.chars() {
                len_input_checked += 1;
                if token.contains(c) && !token.starts_with("[^") {
                    flag = true;
                    break
                }
                else if token.starts_with("[^") && !token.contains(c) {
                    flag = true;
                    break
                }
            }
            if !flag {
                return (false, 0);
            }
            else {
                input_check = input_check[len_input_checked..].to_string();
                let (flag, len) = Self::match_pattern_internal(&input_check, parser);
                return (flag, len);
            }
        }


        if let Some(token) = parser.parse_parentheses() {
            let alternates = Self::split_alternatives(&token);
            for alternate in alternates {
                let mut new_parser = Parser::new(&alternate);
                let (flag, len) = Self::match_pattern_internal(&input_check, &mut new_parser);
                if flag {
                    input_check = input_check[len as usize..].to_string();
                    println!("input_check: {}, pattern: {:?}", input_check, &parser.chars.clone().collect::<String>());
                    let (flag, len_inside) = Self::match_pattern_internal(&input_check, parser);
                    return (flag, len + len_inside);
                }
            }
            return (false, 0);
        }

        if let Some(_) = parser.parse_dot() {
            if input_check.is_empty() { return (false, 0); }
            let (flag, len) = Self::match_pattern_internal(&input_check[1..], parser);
            return (flag, 1 as i32 + len);
        }

        let literal = parser.parse_literal();
        if !literal.is_empty() {
            if input_check.starts_with(&literal) {
                input_check = input_check[literal.len()..].to_string();
                let (flag, len) = Self::match_pattern_internal(&input_check, parser);
                return (flag, literal.len() as i32 + len);
            }
            else {
                return (false, 0);
            }
        }

        (false, 0)

    }
}

