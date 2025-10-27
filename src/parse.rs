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
}

