use std::iter::Peekable;
use std::str::Chars;

enum Token {
    Literal(String),
    CharClass(String),
    Dot,
    Slash,
    Parentheses(String),
}

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

    pub fn parse_literal(&mut self) -> Option<String> {
        let mut literal = String::new();

        let specials = ".^$+?|()[]\\";

        while let Some(c) = self.peek() {
            if specials.contains(c) {
                break;
            }
            literal.push(c);
            self.next();
        }

        if literal.is_empty() {
            None
        }
        else {
            Some(literal)
        }
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

    pub fn parse_slash(&mut self) -> Option<String> {
        if let Some(c) = self.peek() {
            if c == '\\' {
                self.next();
                if let Some(c) = self.peek() {
                    if c == 'd' {
                        self.next();
                        Some("\\d".to_string())
                    }
                    else {panic!("Invalid escape sequence")}
                }
                else {panic!("Invalid escape sequence")}
            }
            else {None}
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

    pub fn parse_start_anchor(&mut self) -> Option<String> {
        if let Some(c) = self.peek() {
            if c == '^' {
                self.next();
                return Some("^".to_string())
            }
            else {return None}
        }
        None
    }

    pub fn parse_end_anchor(&mut self) -> Option<String> {
        if let Some(c) = self.peek() {
            if c == '$' {
                self.next();
                return Some("$".to_string());
            }
            else {return None};
        }
        None
    }

    pub fn match_pattern(input: &str, pattern: &str) -> bool {
        let mut parser = Parser::new(pattern);
        let (flag, _) = Self::match_pattern_internal(input, &mut parser, false);
        println!("-------> match pattern done flag: {}", flag);
        flag
    }

    fn match_token_atom(input: &str, token: Token, start_anchor: bool) -> (bool, i32) {
        match token {
            Token::CharClass(token) => {
                let mut len_input_checked: usize = 0;
                let mut flag = false;
                for c in input.chars() {
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
                else if start_anchor && len_input_checked > 1 {
                    return (false, 0);
                }
                else {
                    return (flag, len_input_checked as i32);
                }
            }
            Token::Slash => {
                if input.is_empty() { return (false, 0); }
                for (index, c) in input.chars().enumerate() {
                    if c.is_digit(10) {
                        if start_anchor && index > 0 {
                            return (false, 0);
                        }
                        return (true, index as i32 + 1);
                    }
                }
                return (false, 0);
            }
            Token::Dot => {
                if input.is_empty() { return (false, 0); }
                return (true, 1);
            }
            Token::Literal(token) => {
                if input.len() < token.len() { return (false, 0); }
                for index in 0..input.len() - token.len() + 1 {
                    if input[index..].starts_with(&token) {
                        if start_anchor && index > 0 {
                            return (false, 0);
                        }
                        return (true, index as i32 + token.len() as i32);
                    }
                }
                return (false, 0);
            }
            Token::Parentheses(token) => {
                let alternates: Vec<String> = Self::split_alternatives(&token);
                for alternate in alternates {
                    let mut new_parser = Parser::new(&alternate);
                    let (flag, len) = Self::match_pattern_internal(&input, &mut new_parser, start_anchor);
                    if flag {
                        return (true, len);
                    }
                }
                return (false, 0);
            }
        }
        
    }

    fn match_pattern_internal(input: &str, parser: &mut Parser, mut start_anchor: bool) -> (bool, i32) {
        println!("-> input: {}, pattern: {:?}", input, &parser.chars.clone().collect::<String>());

        if let Some(_) = parser.parse_start_anchor() {
            start_anchor = true;
        }

        if let Some(_) = parser.parse_end_anchor() {
            if input.is_empty() {
                return (true, 0);
            }
            else {
                return (false, 0);
            }
        }

        if input.is_empty() && !parser.peek().is_none() {
            return (false, 0);
        }

        if parser.peek().is_none() {
            return (true, 0);
        }

        let mut input_check = input.to_string();


        // if let Some(token) = parser.parse_parentheses() {
            
        // }

        let mut token: Option<Token> = None;

        if let Some(pattern) = parser.parse_char_class() {
            token = Some(Token::CharClass(pattern));
        }
        else if let Some(pattern) = parser.parse_parentheses() {
            token = Some(Token::Parentheses(pattern));
        }
        else if let Some(_) = parser.parse_slash() {
            token = Some(Token::Slash);
        }
        else if let Some(_) = parser.parse_dot() {
            token = Some(Token::Dot);
        }
        else if let Some(literal) = parser.parse_literal() {
            token = Some(Token::Literal(literal));
        }

        if let Some(token) = token {
            let (is_match, atom_len) =  Self::match_token_atom(&input_check, token, start_anchor);
            if is_match {
                input_check = input_check[atom_len as usize..].to_string();
                let (flag, len) = Self::match_pattern_internal(&input_check, parser, false);
                return (flag, atom_len as i32 + len);
            }
            else {
                return (false, 0);
            }
        }
        (false, 0)
    }
}

