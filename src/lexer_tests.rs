#[cfg(test)]
mod tests {
    use crate::lexer_types::{Keyword, Lexer, LiteralString, NumericConstant, Span, Symbol, TokenKind};

    #[test]
    fn test_create() {
        let s = "while true abc! J(@ jg2gh";
        let l = Lexer::new(s);

        assert_eq!(l.get_view(), s);
        assert_eq!(l.tokens_len(), 0);
    }

    #[test]
    fn test_simple_whitespace() {
        let s = "   ";
        let mut l = Lexer::new(s);

        l.lex_to_end();
        assert_eq!(l.tokens_len(), 0);
    }

    #[test]
    fn test_simple_keyword() {
        let mut l = Lexer::new("and");
        
        l.lex_to_end();
        let mut i = l.iter_tokens();
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::And));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_simple_symbol() {
        let mut l = Lexer::new("*");
        
        l.lex_to_end();
        let mut i = l.iter_tokens();
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Star));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_simple_identifier() {
        let mut l = Lexer::new("x y");
        
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("x"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("y"));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_long_identifier() {
        let mut l = Lexer::new("awoifjaw anderthal");
        
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("awoifjaw"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("anderthal"));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_simple_short_string() {
        let s = r#""ab \" c""#;
        let s1 = r#"ab \" c"#;
        let mut l = Lexer::new(s);
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::LiteralString(LiteralString::UnescapedShort(s1)));
        assert_eq!(i.next().unwrap().get_span(), Span(s));
    }

    #[test]
    fn test_z_short_string_escape() {
        let s = r#""a\z   
b""#;
        let s_unescaped = r#"a\z   
b"#;
        let s_escaped = r#"ab"#;
        let mut l = Lexer::new(s);
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::LiteralString(LiteralString::UnescapedShort(s_unescaped)));
        assert_eq!(i.peek().unwrap().get_span(), Span(s));
        let lit_str = i.next().unwrap().get_kind().unwrap_literal_string();
        assert_eq!(lit_str.escape(), LiteralString::Escaped(String::from(s_escaped)));
    }

    #[test]
    fn test_complex_short_string_escape() {
        let s = r#""a\z   
\
\n
\u{000000078} \65 \118 \a \" \\\\""#;
        let s_unescaped = r#"a\z   
\
\n
\u{000000078} \65 \118 \a \" \\\\"#;
        let s_escaped = "a\n\n\nx A v \u{7} \" \\\\";
        let mut l = Lexer::new(s);
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::LiteralString(LiteralString::UnescapedShort(s_unescaped)));
        assert_eq!(i.peek().unwrap().get_span(), Span(s));
        let lit_str = i.next().unwrap().get_kind().unwrap_literal_string();
        assert_eq!(lit_str.escape(), LiteralString::Escaped(String::from(s_escaped)));
    }

    #[test]
    fn test_simple_long_string() {
        let s = "[[ abc\nabc]]";
        let s1 = " abc\nabc";
        let mut l = Lexer::new(s);
        
        l.lex_to_end();
        let mut i = l.iter_tokens();
        let a = i.next().unwrap();
        assert_eq!(a.get_kind(),
            &TokenKind::LiteralString(LiteralString::UnescapedLong(s1)));
        assert_eq!(a.get_span(), Span(s));
        assert_eq!(i.next(), None);
    }
    
    #[test]
    fn test_complicated_long_string() {
        let s = "[==[ [abc\nabc] ]==]";
        let s1 = " [abc\nabc] ";
        let mut l = Lexer::new(s);

        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(),
            &TokenKind::LiteralString(LiteralString::UnescapedLong(s1)));
        assert_eq!(i.peek().unwrap().get_span(), Span(s));
        assert_eq!(i.next().unwrap().get_kind().get_literal_string().unwrap().escape(),
            LiteralString::Escaped(String::from(s1)));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_numeric_constants() {
        let s = "0 15 153803 1.3 0. .0 0.0005 0x345 0xff 0xBEBADA 0xA3Fb.F31fAp-14";
        let mut l = Lexer::new(s);
        
        l.lex_to_end();
        dbg!(&l);
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Integer(0)));
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Integer(15)));
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Integer(153803)));
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Float(1.3)));
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Float(0.0)));
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Float(0.0)));
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Float(0.0005)));
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Integer(837))); //0x345
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Integer(255))); //0xff
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Integer(12499674))); //0xBEBADA
        // in my testing, lua 5.4 print() only outputs 2.5622527893865, in rust we get an added 48 on there
        // I just assume it is printing differences, this might be incorrect
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Float(2.562252789386548))); //0xA3Fb.F31fAp-14
        assert_eq!(i.next(), None);

    }
}