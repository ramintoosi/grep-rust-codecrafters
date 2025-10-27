use codecrafters_grep::parse::Parser;

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

    #[test]
    fn test_parse_parentheses_basic() {
        let mut parser = Parser::new("(cat|dog)+");
        let group = parser.parse_parentheses();
        assert_eq!(group, Some("(cat|dog)".to_string()));
        assert_eq!(parser.peek(), Some('+'));
    }

    #[test]
    fn test_parse_parentheses_nested() {
        let mut parser = Parser::new("(a|(b|c))end");
        let group = parser.parse_parentheses();
        assert_eq!(group, Some("(a|(b|c))".to_string()));
        assert_eq!(parser.peek(), Some('e'));
    }

    #[test]
    #[should_panic(expected = "Unclosed parentheses")]
    fn test_parse_parentheses_unclosed() {
        let mut parser = Parser::new("(abc");
        let _ = parser.parse_parentheses();
    }
}
