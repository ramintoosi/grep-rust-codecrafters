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
        Self::match_pattern_internal(input, &mut parser)
    }

    fn match_pattern_internal(input: &str, parser: &mut Parser) -> bool {
        println!("input: {}, pattern: {:?}", input, parser.chars);

        if input.is_empty() && !parser.peek().is_none() {
            return false;
        }

        if parser.peek().is_none() {
            return true;
        }

        if let Some(cls) = parser.parse_parentheses() {
            let alternates = Self::split_alternatives(&cls);
            for alternate in alternates {
                let mut new_parser = Parser::new(&alternate);
                if Self::match_pattern_internal(input, &mut new_parser) {
                    return true;
                }
            }
            return false;
        }

        if let Some(_) = parser.parse_dot() {
            if input.is_empty() { return false; }
            return Self::match_pattern_internal(&input[1..], parser);
        }

        let literal = parser.parse_literal();
        if !literal.is_empty() {
            if input.starts_with(&literal) {
                return Self::match_pattern_internal(&input[literal.len()..], parser);
            }
            else {
                return false;
            }
        }


        false

    }
}

