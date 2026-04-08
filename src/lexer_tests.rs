#[cfg(test)]
mod tests {
    use crate::{keyword::Keyword, lexer::Lexer, literalstring::LiteralString, numeric_constant::NumericConstant, symbol::Symbol, token::Span, token_kind::TokenKind};

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
    fn test_all_keywords() {
        let s = "and break do elseif else end false for function goto if in local nil not or repeat return then true until while";
        let mut l = Lexer::new(s);

        l.lex_to_end();
        let mut i = l.iter_tokens();

        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::And));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Break));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Do));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Elseif));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Else));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::End));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::False));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::For));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Function));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Goto));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::If));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::In));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Local));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Nil));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Not));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Or));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Repeat));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Return));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Then));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::True));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::Until));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Keyword(Keyword::While));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_all_symbols() {
        let mut l = Lexer::new("+ - * / % ^ # & ~ | << >> // == ~= <= >= < > = ( ) { } [ :: ] ; : , . .. ...");
        
        l.lex_to_end();
        let mut i = l.iter_tokens();
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Plus));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Dash));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Star));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Slash));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Percent));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Caret));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Hashtag));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Ampersand));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Tilde));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Pipe));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::LShift));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::RShift));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::SlashSlash));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::EqualsEquals));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::NotEquals));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::LessOrEquals));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::GreaterOrEquals));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::LessThan));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::GreaterThan));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Equals));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::LParen));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::RParen));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::LCurly));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::RCurly));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::LBracket));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::ColonColon));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::RBracket));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Semicolon));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Colon));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Comma));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::Dot));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::DotDot));
        assert_eq!(i.next().unwrap().get_kind(), &TokenKind::Symbol(Symbol::DotDotDot));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_identifiers() {
        let mut l = Lexer::new("x variable anderthal functional _ _a _3 a3");
        
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("x"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("variable"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("anderthal"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("functional"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("_"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("_a"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("_3"));
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::Identifier);
        assert_eq!(i.next().unwrap().get_span(), Span("a3"));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_simple_short_string() {
        let s = r#""ab \" c""#;
        let s_escaped = r#"ab " c"#;
        let mut l = Lexer::new(s);
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::LiteralString(LiteralString::from_already_escaped(s_escaped)));
        assert_eq!(i.next().unwrap().get_span(), Span(s));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_z_short_string_escape() {
        let s = r#""a\z   
b""#;
        let s_escaped = r#"ab"#;
        let mut l = Lexer::new(s);
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::LiteralString(LiteralString::from_already_escaped(s_escaped)));
        assert_eq!(i.next().unwrap().get_span(), Span(s));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_complex_short_string_escape() {
        let s = r#""a\z   
\
\n
\u{000000078} \65 \118 \a \" \\\\""#;
        let s_escaped = "a\n\n\nx A v \u{7} \" \\\\";
        let mut l = Lexer::new(s);
        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(), &TokenKind::LiteralString(LiteralString::from_already_escaped(s_escaped)));
        assert_eq!(i.next().unwrap().get_span(), Span(s));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_simple_long_string() {
        let s = "[[ abc\nabc]]";
        let s_escaped = " abc\nabc";
        let mut l = Lexer::new(s);
        
        l.lex_to_end();
        let mut i = l.iter_tokens();
        let a = i.next().unwrap();
        assert_eq!(a.get_kind(),
            &TokenKind::LiteralString(LiteralString::from_already_escaped(s_escaped)));
        assert_eq!(a.get_span(), Span(s));
        assert_eq!(i.next(), None);
    }
    
    #[test]
    fn test_complicated_long_string() {
        let s = "[==[\x0A\x0D [--abc\x0A --[\x0D \x0D\x0A]=] \x0A\x0Dabc] ]==]";
        let s_escaped = " [--abc\x0A --[\x0A \x0A]=] \x0Aabc] ";
        let mut l = Lexer::new(s);

        l.lex_to_end();
        let mut i = l.iter_tokens().peekable();
        assert_eq!(i.peek().unwrap().get_kind(),
            &TokenKind::LiteralString(LiteralString::from_already_escaped(s_escaped)));
        assert_eq!(i.next().unwrap().get_span(), Span(s));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_numeric_constants() {
        let s = "0 15 153803 1.3 1.5e-3 1.3e4 0. .0 0.0005 0x345 0xff 0xBEBADA 0xA3Fb.F31fAp-14";
        let mut l = Lexer::new(s);
        
        l.lex_to_end();
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
            &TokenKind::NumericConstant(NumericConstant::Float(0.0015)));
        assert_eq!(i.next().unwrap().get_kind(),
            &TokenKind::NumericConstant(NumericConstant::Float(13000.0)));
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

    /// the example lua code in the string, and the "expected" values, were generated by an llm
    /// I have looked over this, but I have not fine combed through the string and expected values
    #[test]
    fn test_extensive_lua_example() { 
        let s = r#"
-- Single-line comment: Testing the bounds of Lua's lexer
--[[ 
  Multi-line comment block.
  We will cover all 22 keywords, various number formats, 
  string boundaries, and Lua 5.3+ operators.
]]

local function extreme_lexer_test(param1, ...)
    ::start_label:: -- A label for the rarely used goto statement

    -- 1. Numbers (Integers, Floats, Hex, Hex-Floats, Scientific)
    local int_val    = 42
    local float_val  = 3.14159265
    local sci_val    = 1.2e-5
    local hex_val    = 0xDEADBEEF
    local hex_float  = 0x1.921fb54442d18p+1 -- e.g., pi in hex-float

    -- 2. Strings (Short, Long, All Escapes)
    -- Covers: \a, \b, \f, \n, \r, \t, \v, \\, and \"
    local str_double = "Basic escapes: bell\a, backspace\b, formfeed\f, newline\n, return\r, tab\t, vtab\v, slash\\\\, d-quote\""
    
    -- Covers: \', \ddd (decimal), \xXX (hex), \u{XXX} (UTF-8), and \z (skip whitespace)
    local str_single = 'Advanced escapes: s-quote\', dec\065, hex\x42, unicode\u{1F445}, and gap\z
                        skipping across newlines.'

    local long_str   = [==[
        Long strings ignore escape sequences like \n.
        The [==[ syntax allows nested [[ ]] blocks inside safely.
    ]==]

    -- 3. Booleans and Nil
    local is_true  = true
    local is_false = false
    local nothing  = nil

    -- 4. Tables, Punctuation, and Variadic arguments
    local complex_table = {
        ["key_string"] = str_double .. str_single, -- Concatenation
                   = { ... },                  -- Variadic expansion
        nested         = int_val + float_val * 2 / 1
    }

    -- 5. Operators (Length, Arithmetic, Bitwise, Relational, Logical)
    local tbl_len    = #complex_table
    local power_mod  = (5 ^ 2) % 4
    local floor_div  = 10 // 3                     -- Lua 5.3+ Floor division
    local bitwise    = (hex_val & 0xFF) | (1 << 4) ~ (int_val >> 1) -- Bitwise ops

    -- 6. Control Structures (If, Elseif, Else, Goto)
    if not is_false and (int_val ~= float_val) then
        ; -- Empty statement (semicolon)
    elseif int_val == 42 or nothing == nil then
        -- Relational (==, ~=, <, >, <=, >=) all follow similar lexing rules
        local dummy = float_val >= sci_val
    else
        goto start_label
    end

    -- 7. Loops (Numeric For, Generic For, While, Repeat-Until)
    for i = 1, 10, 2 do
        if i > 5 then break end
    end

    for k, v in pairs(complex_table) do
        -- Assuming 'pairs' is in the global environment
    end

    local counter = 0
    while counter <= 5 do
        counter = counter + 1
    end

    repeat
        counter = counter - 1
    until counter < 0

    -- 8. Object-oriented colon syntax
    complex_table:dummy_method()

    -- 9. Anonymous function return
    return function() return tbl_len end
end
        "#;
        
        use crate::token_kind::TokenKind::{Keyword as TKw, Symbol as TSym, Identifier as TId, NumericConstant as TNum, LiteralString as TStr};
        use crate::keyword::Keyword::*;
        use crate::symbol::Symbol::*;
        use crate::numeric_constant::NumericConstant::*;

        let expected = vec![
            TKw(Local), TKw(Function), TId, TSym(LParen), TId, TSym(Comma), TSym(DotDotDot), TSym(RParen),
            TSym(ColonColon), TId, TSym(ColonColon),

            TKw(Local), TId, TSym(Equals), TNum(Integer(42)),
            TKw(Local), TId, TSym(Equals), TNum(Float(3.14159265)),
            TKw(Local), TId, TSym(Equals), TNum(Float(1.2e-05)),
            TKw(Local), TId, TSym(Equals), TNum(Integer(3735928559)),
            TKw(Local), TId, TSym(Equals), TNum(Float(3.141592653589793)),

            // escapes: \a (0x07), \b (0x08), \f (0x0C), \v (0x0B). \\ is just \, \" is just "
            TKw(Local), TId, TSym(Equals), TStr(LiteralString::from_already_escaped("Basic escapes: bell\x07, backspace\x08, formfeed\x0C, newline\n, return\r, tab\t, vtab\x0B, slash\\\\, d-quote\"")),
            
            // \065 is 'A', \x42 is 'B', \z removes whitespace
            TKw(Local), TId, TSym(Equals), TStr(LiteralString::from_already_escaped("Advanced escapes: s-quote', decA, hexB, unicode👅, and gapskipping across newlines.")),
            
            // newline immediately after [==[ is stripped
            // escape sequences aren't parsed in long strings, so \n should be unchanged
            TKw(Local), TId, TSym(Equals), TStr(LiteralString::from_already_escaped("        Long strings ignore escape sequences like \\n.\n        The [==[ syntax allows nested [[ ]] blocks inside safely.\n    ")),

            TKw(Local), TId, TSym(Equals), TKw(True),
            TKw(Local), TId, TSym(Equals), TKw(False),
            TKw(Local), TId, TSym(Equals), TKw(Nil),

            TKw(Local), TId, TSym(Equals), TSym(LCurly),
            TSym(LBracket), TStr(LiteralString::from_escape_short("key_string")), TSym(RBracket), TSym(Equals), TId, TSym(DotDot), TId, TSym(Comma),
            TSym(Equals), TSym(LCurly), TSym(DotDotDot), TSym(RCurly), TSym(Comma),
            TId, TSym(Equals), TId, TSym(Plus), TId, TSym(Star), TNum(Integer(2)), TSym(Slash), TNum(Integer(1)),
            TSym(RCurly),

            TKw(Local), TId, TSym(Equals), TSym(Hashtag), TId,
            TKw(Local), TId, TSym(Equals), TSym(LParen), TNum(Integer(5)), TSym(Caret), TNum(Integer(2)), TSym(RParen), TSym(Percent), TNum(Integer(4)),
            TKw(Local), TId, TSym(Equals), TNum(Integer(10)), TSym(SlashSlash), TNum(Integer(3)),
            TKw(Local), TId, TSym(Equals), TSym(LParen), TId, TSym(Ampersand), TNum(Integer(0xFF)), TSym(RParen), TSym(Pipe), TSym(LParen), TNum(Integer(1)), TSym(LShift), TNum(Integer(4)), TSym(RParen), TSym(Tilde), TSym(LParen), TId, TSym(RShift), TNum(Integer(1)), TSym(RParen),

            TKw(If), TKw(Not), TId, TKw(And), TSym(LParen), TId, TSym(NotEquals), TId, TSym(RParen), TKw(Then),
            TSym(Semicolon),
            TKw(Elseif), TId, TSym(EqualsEquals), TNum(Integer(42)), TKw(Or), TId, TSym(EqualsEquals), TKw(Nil), TKw(Then),
            TKw(Local), TId, TSym(Equals), TId, TSym(GreaterOrEquals), TId,
            TKw(Else),
            TKw(Goto), TId,
            TKw(End),

            TKw(For), TId, TSym(Equals), TNum(Integer(1)), TSym(Comma), TNum(Integer(10)), TSym(Comma), TNum(Integer(2)), TKw(Do),
            TKw(If), TId, TSym(GreaterThan), TNum(Integer(5)), TKw(Then), TKw(Break), TKw(End),
            TKw(End),

            TKw(For), TId, TSym(Comma), TId, TKw(In), TId, TSym(LParen), TId, TSym(RParen), TKw(Do),
            TKw(End),

            TKw(Local), TId, TSym(Equals), TNum(Integer(0)),
            TKw(While), TId, TSym(LessOrEquals), TNum(Integer(5)), TKw(Do),
            TId, TSym(Equals), TId, TSym(Plus), TNum(Integer(1)),
            TKw(End),

            TKw(Repeat),
            TId, TSym(Equals), TId, TSym(Dash), TNum(Integer(1)),
            TKw(Until), TId, TSym(LessThan), TNum(Integer(0)),

            TId, TSym(Colon), TId, TSym(LParen), TSym(RParen),

            TKw(Return), TKw(Function), TSym(LParen), TSym(RParen), TKw(Return), TId, TKw(End),
            TKw(End),
        ];

        let mut l = Lexer::new(s);
        l.lex_to_end();

        let actual = l.iter_tokens()
            .map(|t| t.get_kind())
            .collect::<Vec<&TokenKind>>();

        assert_eq!(actual.len(), expected.len());

        for (actual_token, expected_token) in actual.iter().zip(expected.iter()) {
            assert_eq!(actual_token, &expected_token);
        }
    }
}