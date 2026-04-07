use crate::util::*;

#[derive(Debug, PartialEq)]
pub enum TokenKind<'i> {
    Keyword(Keyword),
    Symbol(Symbol),
    LiteralString(LiteralString<'i>),
    NumericConstant(NumericConstant),
    Identifier,
}
impl<'i> TokenKind<'i> {
    pub fn get_keyword(&self) -> Option<&Keyword> {
        match self {
            TokenKind::Keyword(k) => Some(k),
            _ => None
        }
    }
    pub fn get_symbol(&self) -> Option<&Symbol> {
        match self {
            TokenKind::Symbol(s) => Some(s),
            _ => None
        }
    }
    pub fn get_literal_string(&self) -> Option<&LiteralString> {
        match self {
            TokenKind::LiteralString(ls) => Some(ls),
            _ => None
        }
    }
    pub fn get_numeric_constant(&self) -> Option<&NumericConstant> {
        match self {
            TokenKind::NumericConstant(nc) => Some(nc),
            _ => None
        }
    }
    pub fn unwrap_keyword(&self) -> &Keyword {
        match self {
            TokenKind::Keyword(k) => k,
            _ => panic!("Incorrect tokenkind! Actual value: {:?}",self)
        }
    }
    pub fn unwrap_symbol(&self) -> &Symbol {
        match self {
            TokenKind::Symbol(s) => s,
            _ => panic!("Incorrect tokenkind! Actual value: {:?}",self)
        }
    }
    pub fn unwrap_literal_string(&self) -> &LiteralString {
        match self {
            TokenKind::LiteralString(ls) => ls,
            _ => panic!("Incorrect tokenkind! Actual value: {:?}",self)
        }
    }
    pub fn unwrap_numeric_constant(&self) -> &NumericConstant {
        match self {
            TokenKind::NumericConstant(nc) => nc,
            _ => panic!("Incorrect tokenkind! Actual value: {:?}",self)
        }
    }
    pub fn is_identifier(&self) -> bool {
        match self {
            TokenKind::Identifier => false,
            _ => true
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
//  and       break     do        else      elseif    end
//  false     for       function  goto      if        in
//  local     nil       not       or        repeat    return
//  then      true      until     while
    And,       Break,     Do,        Else,      Elseif,    End,
    False,     For,       Function,  Goto,      If,        In,
    Local,     Nil,       Not,       Or,        Repeat,    Return,
    Then,      True,      Until,     While,
}
impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "and" => Some(Keyword::And),
            "break" => Some(Keyword::Break),
            "do" => Some(Keyword::Do),
            "elseif" => Some(Keyword::Elseif),
            "else" => Some(Keyword::Else),
            "end" => Some(Keyword::End),
            "false" => Some(Keyword::False),
            "for" => Some(Keyword::For),
            "function" => Some(Keyword::Function),
            "goto" => Some(Keyword::Goto),
            "if" => Some(Keyword::If),
            "in" => Some(Keyword::In),
            "local" => Some(Keyword::Local),
            "nil" => Some(Keyword::Nil),
            "not" => Some(Keyword::Not),
            "or" => Some(Keyword::Or),
            "repeat" => Some(Keyword::Repeat),
            "return" => Some(Keyword::Return),
            "then" => Some(Keyword::Then),
            "true" => Some(Keyword::True),
            "until" => Some(Keyword::Until),
            "while" => Some(Keyword::While),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
//  +               -           *               /                   %           ^               #
//  &               ~           |               <<                  >>          //
//  ==              ~=          <=              >=                  <           >               =
//  (               )           {               }                   [           ]               ::
//  ;               :           ,               .                   ..          ...
    Plus,           Dash,       Star,           Slash,              Percent,    Caret,          Hashtag,
    Ampersand,      Tilde,      Pipe,           LShift,             RShift,     SlashSlash,
    EqualsEquals,   NotEquals,  LessOrEquals,   GreaterOrEquals,    LessThan,   GreaterThan,    Equals,
    LParen,         RParen,     LCurly,         RCurly,             LBracket,   RBracket,       ColonColon,
    Semicolon,      Colon,      Comma,          Dot,                DotDot,     DotDotDot,
}
impl Symbol {
    pub fn to_str(sym: Symbol) -> &'static str {
        match sym {
            Symbol::Plus => "+",
            Symbol::Dash => "-",
            Symbol::Star => "*",
            Symbol::Slash => "/",
            Symbol::Percent => "%",
            Symbol::Caret => "^",
            Symbol::Hashtag => "#",
            Symbol::Ampersand => "&",
            Symbol::Tilde => "~",
            Symbol::Pipe => "|",
            Symbol::LShift => "<<",
            Symbol::RShift => ">>",
            Symbol::SlashSlash => "//",
            Symbol::EqualsEquals => "==",
            Symbol::NotEquals => "~=",
            Symbol::LessOrEquals => "<=",
            Symbol::GreaterOrEquals => ">=",
            Symbol::LessThan => "<",
            Symbol::GreaterThan => ">",
            Symbol::Equals => "=",
            Symbol::LParen => "(",
            Symbol::RParen => ")",
            Symbol::LCurly => "{",
            Symbol::RCurly => "}",
            Symbol::LBracket => "[",
            Symbol::ColonColon => "::",
            Symbol::RBracket => "]",
            Symbol::Semicolon => ";",
            Symbol::Colon => ":",
            Symbol::Comma => ",",
            Symbol::Dot => ".",
            Symbol::DotDot => "..",
            Symbol::DotDotDot => "...",
        }
    }
    pub fn from_str(s: &str) -> Option<Symbol> {
        match s {
            "+" => Some(Symbol::Plus),
            "-" => Some(Symbol::Dash),
            "*" => Some(Symbol::Star),
            "/" => Some(Symbol::Slash),
            "%" => Some(Symbol::Percent),
            "^" => Some(Symbol::Caret),
            "#" => Some(Symbol::Hashtag),
            "&" => Some(Symbol::Ampersand),
            "~" => Some(Symbol::Tilde),
            "|" => Some(Symbol::Pipe),
            "<<" => Some(Symbol::LShift),
            ">>" => Some(Symbol::RShift),
            "//" => Some(Symbol::SlashSlash),
            "==" => Some(Symbol::EqualsEquals),
            "~=" => Some(Symbol::NotEquals),
            "<=" => Some(Symbol::LessOrEquals),
            ">=" => Some(Symbol::GreaterOrEquals),
            "<" => Some(Symbol::LessThan),
            ">" => Some(Symbol::GreaterThan),
            "=" => Some(Symbol::Equals),
            "(" => Some(Symbol::LParen),
            ")" => Some(Symbol::RParen),
            "{" => Some(Symbol::LCurly),
            "}" => Some(Symbol::RCurly),
            "[" => Some(Symbol::LBracket),
            "::" => Some(Symbol::ColonColon),
            "]" => Some(Symbol::RBracket),
            ";" => Some(Symbol::Semicolon),
            ":" => Some(Symbol::Colon),
            "," => Some(Symbol::Comma),
            "." => Some(Symbol::Dot),
            ".." => Some(Symbol::DotDot),
            "..." => Some(Symbol::DotDotDot),
            _ => None,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum NumericConstant {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum LiteralString<'i> {
    Escaped(String),
    UnescapedLong(&'i str),
    UnescapedShort(&'i str),
}
impl<'i> LiteralString<'i> {
    pub fn is_escaped(&self) -> bool {
        match self {
            Self::Escaped(_) => true,
            _ => false,
        }
    }
    pub fn escape(&self) -> Self {
        match self {
            Self::UnescapedShort(s) => LiteralString::escape_short(s),
            Self::UnescapedLong(s) => LiteralString::escape_long(s),
            Self::Escaped(_) => panic!("This LiteralString is already escaped!"),
        }
    }
    fn escape_short(s: &'i str) -> Self {
        let mut res = Vec::with_capacity(s.len());
        let mut bytes = s.bytes().peekable();

        while let Some(b) = bytes.next() {
            if b != b'\\' {
                res.push(b);
                continue;
            }
            let escape_c = bytes.next().expect("Trailing slash should have been caught by string lexing, not by escaping !");
            match escape_c {
                b'a' => res.push(b'\x07'),
                b'b' => res.push(b'\x08'),
                b'f' => res.push(b'\x0C'),
                b'n' => res.push(b'\x0A'), //lua 5.3 manual says newline, but see https://www.lua.org/source/5.3/llex.c.html `read_string`, just \n in c code, which is line feed
                b'r' => res.push(b'\x0D'),
                b't' => res.push(b'\x09'),
                b'v' => res.push(b'\x0B'),
                b'\\'=> res.push(b'\\'),
                b'\"'=> res.push(b'\"'),
                b'\''=> res.push(b'\''),
                b'\n'=> res.push(b'\n'), //ouhh platform dependent? unsure if this will work consistently

                b'z' => {
                    while bytes.peek().is_some() &&
                          bytes.peek().unwrap().is_ascii_whitespace() { //might be a problem, unicode whitespace
                        bytes.next().unwrap();
                    }
                },

                b'x' => {
                    let d1 = bytes.next().expect("Unfinished \\x escape sequence at end of string!");
                    let d2 = bytes.next().expect("Unfinished \\x escape sequence at end of string!");

                    let digits = [d1,d2];
                    let tmp_s = str::from_utf8(&digits).expect("Invalid utf8? in \\x escape sequence");
                    let v = u8::from_str_radix(tmp_s, 16).expect("Invalid hex, maybe too large? in \\x escape sequence!");

                    res.push(v);
                },

                b'u' => {
                    let open_brace = bytes.next();
                    if !(open_brace.is_some() && open_brace.unwrap() == b'{') {
                        panic!("Invalid \\u escape sequence! Missing opening brace")
                    }

                    let mut usv_hex: u32 = 0;
                    let mut found_closing = false;
                    let mut encountered_digit = false;

                    while let Some(c) = bytes.next() {
                        if c == b'}' {
                            found_closing = true;
                            break;
                        }
                        if !c.is_ascii_hexdigit() {
                            panic!("Invalid character in \\u escape sequence!");
                        }
                        let d = (c as char).to_digit(16).unwrap();
                        usv_hex = (usv_hex << 4) | d;
                        encountered_digit = true;
                    }

                    if !found_closing {
                        panic!("Invalid \\u escape sequence! Missing closing brace")
                    }
                    if !encountered_digit {
                        panic!("Invalid \\u escape sequence! No digits")
                    }
                    // I'm reasonably sure the limit is 0x10FFFF, not 0x7FFFFFFF in 5.3. I won't test it in 5.3 💔, it's just what I gather from the lua source code and testing in 5.4
                    // see:   readutf8esc   https://www.lua.org/source/5.3/llex.c.html   https://www.lua.org/source/5.4/llex.c.html/
                    if usv_hex > 0x10FFFF {
                        panic!("Invalid \\u escape sequence! beyond lua 5.3 0x10FFFF limit!");
                    }

                    let usv = char::from_u32(usv_hex)
                        .expect("Invalid \\u escape sequence! Provided hex aint valid USV! this error should be impossible!");
                    let mut tmp_s = [0u8;4];
                    res.extend_from_slice(usv.encode_utf8(&mut tmp_s).as_bytes());
                },

                d @ b'0'..=b'9' => {
                    let mut decimals_b = [d,0,0];
                    let mut len = 1;

                    while len < 3 &&
                          bytes.peek().is_some() &&
                          bytes.peek().unwrap().is_ascii_digit() {
                        
                        decimals_b[len] = bytes.next().unwrap();
                        len += 1;
                    }
                    let s = str::from_utf8(&decimals_b[..len]).unwrap();
                    let v = u8::from_str_radix(s, 10).expect("Invalid decimal escape! out of u8 range");
                    res.push(v);
                },

                _ => panic!("invalid escape :/"),
            };
        }

        LiteralString::Escaped(String::from_utf8(res).unwrap())
    }
    fn escape_long(s: &'i str) -> Self {
        let mut res = Vec::with_capacity(s.len());
        let mut bytes = s.bytes().peekable();

        // For convenience, when the opening long bracket is immediately followed by a newline, the newline is not included in the string.
        if let Some(10_u8) = bytes.peek() {
            bytes.next();
        }

        // Any kind of end-of-line sequence (carriage return, newline, carriage return followed by newline, or newline followed by carriage return) is converted to a simple newline.
        while let Some(b) = bytes.next() {
            match b {
                10_u8 => {
                    if let Some(&13_u8) = bytes.peek() {
                        bytes.next();
                    }
                    res.push(10_u8);
                },
                13_u8 => {
                    if let Some(&10_u8) = bytes.peek() {
                        bytes.next();
                    }
                    res.push(10_u8);
                },
                _ => res.push(b),
            }
        }

        LiteralString::Escaped(String::from_utf8(res).unwrap())
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span<'i>(pub &'i str);

#[derive(Debug, PartialEq)]
pub struct Token<'i> {
    kind: TokenKind<'i>,
    span: Span<'i>,
}
impl<'i> Token<'i> {
    pub fn new(kind: TokenKind<'i>, span: Span<'i>) -> Self {
        return Token { kind, span };
    }
    pub fn get_kind(&self) -> &TokenKind {
        return &self.kind;
    }
    pub fn get_span(&self) -> Span<'i> {
        return self.span;
    }
}

#[derive(Debug)]
pub struct Lexer<'i> {
    input: &'i str,
    view: &'i str, // substring of input used in parsing
    tokens: Vec<Token<'i>>,
}

impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Lexer<'i> {
        Lexer {
            input: input,
            view: input,
            tokens: Vec::new(),
        }
    }

    pub fn get_view(&self) -> &'i str {
        &self.view
    }
    pub fn iter_tokens(&self) -> core::slice::Iter<Token<'i>> {
        self.tokens.iter()
    }
    pub fn view_len(&self) -> usize {
        self.view.len()
    }
    pub fn tokens_len(&self) -> usize {
        self.tokens.len()
    }
    pub fn lex_to_end(&mut self) {
        
        loop {
            if self.lex_one() == false {
                break;
            }
        }
        if self.view.len() > 0 {
            panic!("failed lexing all, remains:\n-----\n{}\n----",self.view);
        }
    }
    fn lex_one(&mut self) -> bool {
        self.skip_whitespace();
        self.skip_comments(); // not 100% confident this is correct

        let res = self
            .lex_identifier_or_keyword()
            .or_else(|| self.lex_numeric_constant())
            .or_else(|| self.lex_short_literal_string())
            .or_else(|| self.lex_long_literal_string())
            .or_else(|| self.lex_symbol())
            ;

        if let Some(token) = res {
            self.tokens.push(token);
            true
        } else {
            false
        }
    }
    pub fn skip_whitespace(&mut self) {
        self.view = self.view.trim_start();
    }
    pub fn lex_identifier_or_keyword(&mut self) -> Option<Token<'i>> {
        let mut found = None;

        let chars = self.view.char_indices();
        for (c_start_byte,c) in chars {
            if c_start_byte == 0 && !c.is_alphabetic() {
                break;
            }
            if ! (c.is_alphanumeric() || c == '_') {
                break;
            }
            found = Some(
                &self.view[..self.view.ceil_char_boundary(c_start_byte+c.len_utf8())]//wouldn't +1 work just as well here?
            )
        };

        found.and_then(|s| {
            self.view = &self.view[s.len()..];

            if let Some(keyword) = Keyword::from_str(s) {
                Some(Token::new(TokenKind::Keyword(keyword), Span(s)))
            } else {
                Some(Token::new(TokenKind::Identifier, Span(s)))
            }
        })
    }
    pub fn lex_numeric_constant(&mut self) -> Option<Token<'i>> {
        let bytes = self.view.as_bytes();
        let mut cursor = 0;
        
        if bytes.starts_with(b"0x") || bytes.starts_with(b"0X") { 
            // is hex constant
            cursor+=2;
            return self.lex_hexadecimal_numeric_constant(bytes,cursor);
        }
        // normal int or float constant
        match bytes.get(cursor..) {
            Some([b'0'..=b'9', ..]) | Some([b'.', b'0'..=b'9', ..]) => {
                return self.lex_decimal_numeric_constant(bytes, cursor);
            },
            _ => return None,
        };
    }
    pub fn lex_short_literal_string(&mut self) -> Option<Token<'i>> {
        let remains = self.view.as_bytes();
        let mut quote_kind = None;
        let Some(mut remains) = remains.strip_prefix(b"\"")
            .inspect(|_| quote_kind = Some(b'"'))
            .or_else(|| {
                remains.strip_prefix(b"\'").inspect(|_| quote_kind = Some(b'\''))
            })
        else {
            return None;
        };
        let quote_kind = quote_kind.expect("if code above is correct, this should never be None");
        let contents_without_opening = remains;
        let mut raw_contents = None;

        while !remains.is_empty() {
            let Some(maybe_end) = remains.iter().position(|&v| v==quote_kind) else {
                // while loop still running, so we haven't found end, and now there's no more potential ends left
                // println!("Failed to find the end!");
                return None;
            };

            // ..z\\" is not the end of a string, but ..g\\\" is. check for even number of continuous backslashes
            let mut backtrack_check = maybe_end -1;
            let mut backslashes = 0;
            while (remains[backtrack_check] == b'\\') && (backtrack_check > 0) {
                backslashes+=1;
                backtrack_check -=1;
            }
            if backslashes%2 == 0 || backslashes == 0 {
                // println!("found the end - no or even backslashes !");
                raw_contents = Some(&contents_without_opening[..contents_without_opening.len()-1]);
                break;
            } else {
                // THIS IS NOT THE END !! KEEP LOOKING FORWARDS!!
                // println!("Trying again - odd backslashes");
                remains = &remains[maybe_end+1..];
                continue;
            }
        }
        
        let Some(raw_inner_contents) = raw_contents else {
            return None;
        };
        let contents = str::from_utf8(raw_inner_contents).expect("something dealing with bytes shat itself above");

        let span = &self.view[..contents.len()+2];
        let contents_tokenkind = TokenKind::LiteralString(LiteralString::UnescapedShort(contents));

        self.view = &self.view[span.len()..];
        Some(Token::new(contents_tokenkind, Span(span)))
    }

    /// For convenience, when the opening long bracket is immediately followed by a newline, the newline is not included in the string. (same does not apply for short string)
    pub fn lex_long_literal_string(&mut self) -> Option<Token<'i>> {
        let mut remains = self.view;
        remains = remains.strip_prefix('[')?;

        let mut opening_equals = 0;
        while let Some(next_remains) = remains.strip_prefix('=') {
            opening_equals += 1;
            remains = next_remains;
        }

        remains = remains.strip_prefix('[')?;
        let span_without_opening = remains;

        let mut found_closing = false;
        while !remains.is_empty() {
            let Some(potential_closing_start) = remains.find(']') else {
                // println!("we BREAKING");
                break // when no more potential endings, end while loop
            };
            remains = &remains[potential_closing_start..];
            let mut check_close = remains.strip_prefix(']').expect("`remains.find` check above ensures this will not panic");

            let mut closing_equals = 0;
            while let Some(next_check_close) = check_close.strip_prefix('=') {
                closing_equals += 1;
                check_close = next_check_close;
            }

            if closing_equals != opening_equals {
                remains = &remains[1..];
                continue;
            }
            if !check_close.starts_with(']') {
                remains = &remains[1..];
                continue;
            }
            remains = &remains[(2+opening_equals)..];
            found_closing = true;
            break;
        }

        if !found_closing {
            return None;
        }

        let span_len = self.view.len() - remains.len();
        let span = &self.view[..span_len];
        let end_len = 2+opening_equals;
        let inner_content = &span_without_opening[..span_without_opening.len()-end_len];

        self.view = &self.view[span.len()..];
        Some(Token::new(TokenKind::LiteralString(LiteralString::UnescapedLong(inner_content)), Span(span)))
    }
    // pub fn lex_long_literal_string(&mut self) -> Option<Token<'i>> {
    //     // looking at bytes is ok here because we look for '[', ']', '=', which are all ascii and-
    //     // cannot be confused for any part of anything that utf8 encodes anything else to
    //     let bytes = self.view.as_bytes();
        
    //     if bytes.first() != Some(&b'[') {
    //         return None;
    //     }
    //     let mut opening_equals = 0;
    //     let mut opened = false;
    //     for &b in bytes[1..].iter() {
    //         match b {
    //             b'=' => opening_equals += 1,
    //             b'[' => {
    //                 opened = true;
    //                 break
    //             },
    //             _ => return None, // e.g. [==a or [a
    //         }
    //     }
    //     if !opened {
    //         return None; // didn't complete the opening to the string
    //     }

    //     let start_length = opening_equals+2;
    //     let mut search_place = start_length;
    //     while let Some(pos) = (&self.view[search_place..]).find(']') {
    //         let pos = search_place+pos;
    //         if start_length > (&self.view[pos..]).len() {
    //             break; // the end sequence (same length as start sequence) cannot fit in what remains
    //         }
    //         let potential_closing = &bytes[pos..=pos+start_length];
    //         println!("Checking {}",str::from_utf8(potential_closing).unwrap());

    //         let found_closing = potential_closing[pos+start_length] == b']' &&
    //             potential_closing[(pos+1)..pos+start_length].iter().all(|&b| b == b'=');
            
    //         if !found_closing {
    //             search_place = pos + 1; // false match, keep looking
    //             continue;
    //         }

    //         let inner_content = &self.view[start_length..pos+start_length];
    //         let span = &self.view[..pos+start_length];
    //         println!("Succeeded, span: {}",span);
    //     }

    //     None
    // }
    pub fn lex_symbol(&mut self) -> Option<Token<'i>> {
        let symbol_patterns = [
            (Symbol::Plus, "+", 1),
            (Symbol::Dash, "-", 1),
            (Symbol::Star, "*", 1),
            (Symbol::Slash, "/", 1),
            (Symbol::Percent, "%", 1),
            (Symbol::Caret, "^", 1),
            (Symbol::Hashtag, "#", 1),
            (Symbol::Ampersand, "&", 1),
            (Symbol::Tilde, "~", 1),
            (Symbol::Pipe, "|", 1),
            (Symbol::LShift, "<<", 2),
            (Symbol::RShift, ">>", 2),
            (Symbol::SlashSlash, "//", 2),
            (Symbol::EqualsEquals, "==", 2),
            (Symbol::NotEquals, "~=", 2),
            (Symbol::LessOrEquals, "<=", 2),
            (Symbol::GreaterOrEquals, ">=", 2),
            (Symbol::LessThan, "<", 1),
            (Symbol::GreaterThan, ">", 1),
            (Symbol::Equals, "=", 1),
            (Symbol::LParen, "(", 1),
            (Symbol::RParen, ")", 1),
            (Symbol::LCurly, "{", 1),
            (Symbol::RCurly, "}", 1),
            (Symbol::LBracket, "[", 1),
            (Symbol::RBracket, "]", 1),
            (Symbol::Semicolon, ";", 1),
            (Symbol::Colon, ":", 1),
            (Symbol::ColonColon, "::", 2),
            (Symbol::Comma, ",", 1),
            (Symbol::Dot, ".", 1),
            (Symbol::DotDot, "..", 2),
            (Symbol::DotDotDot, "...", 3),
        ];

        let mut found: Option<(Symbol, u8, &'i str)> = None;
        for (kind, str_pattern, preference) in symbol_patterns {
                        
            if str_pattern.len() > self.view.len() {
                continue;
            }

            let this_token_view = &self.view[0..=str_pattern.len()-1];
            let matches_this_pattern = this_token_view == str_pattern;

            if !matches_this_pattern {
                continue;
            }
            if found.is_none() {
                found = Some((kind, preference, this_token_view));
                continue;
            }

            let (found_kind,found_preference, _) = found.unwrap();

            if kind == found_kind && preference > found_preference {
                found = Some((kind, preference, this_token_view));
                continue;
            }
        };
        
        if let Some((kind, _, token_view)) = found {
            // not -1 after token_view.len(), because while token_view.len() is one past token_view, that is perfect as we want the index 1 after token_view
            // more of a note to self
            self.view = &self.view[token_view.len()..=(self.view.len()-1)];
            Some( Token::new(TokenKind::Symbol(kind), Span(token_view)) )
        } else {
            None
        }
    }
    fn skip_comments(&mut self) {
        let bytes = self.view.as_bytes();
        
        if !bytes.starts_with(b"--") {
            return;
        }

        let mut cursor = 2;
        let mut is_long = false;
        let mut opening_eq = 0;

        if bytes.get(cursor) == Some(&b'[') {
            cursor+=1;
            while bytes.get(cursor) == Some(&b'=') {
                cursor+=1;
                opening_eq+=1;
            }
            if bytes.get(cursor) == Some(&b'[') {
                cursor+=1;
                is_long = true;
            }
        }

        // skip short comment
        if !is_long {
            if let Some(newline_pos) = (&bytes[cursor..]).iter().position(|&b| b==b'\n') {
                self.view = &self.view[cursor+newline_pos+1 ..];
            } else {
                self.view = &self.view[self.view.len()..]; // we done,,, end of file
            }
            return;
        }
        
        // skip long comment
        let mut current = cursor;
        while let Some(bracket_pos) = bytes[current..].iter().position(|&b| b==b']') {
            current+=bracket_pos+1;
            
            let mut closing_eq = 0;
            while bytes.get(current) == Some(&b'=') {
                closing_eq += 1;
                current += 1;
            }

            if closing_eq == opening_eq && bytes.get(current) == Some(&b']') {
                self.view = &self.view[current+1..];
                return;
            }
        }

        panic!("Unclosed long comment !");
    }
    
    fn lex_decimal_numeric_constant(&mut self, bytes: &'i [u8], cursor: usize) -> Option<Token<'i>> {
        let mut cursor = cursor;
        let mut internally_int = true;
        let mut int_part_bs = Vec::new();
        let mut frac_part_bs = Vec::new();
        let mut exp_part_bs = Vec::new();

        while let Some(&b) = bytes.get(cursor) {
            if !b.is_ascii_digit() {
                break;
            }
            cursor+=1;
            int_part_bs.push(b);
        }

        if bytes.get(cursor) == Some(&b'.') {
            internally_int = false;
            cursor+=1;
            while let Some(&b) = bytes.get(cursor) {
                if !b.is_ascii_digit() {
                    break;
                }
                cursor+=1;
                frac_part_bs.push(b);
            }
        }

        if int_part_bs.len() == 0 && frac_part_bs.len() == 0 {
            panic!("decimal numeric constant needs at least an integer or fractional part !");
        }

        if bytes.get(cursor) == Some(&(b'e'|b'E')) {
            cursor+=1;
            while let Some(&b) = bytes.get(cursor) {
                if !b.is_ascii_digit() {
                    break;
                }
                cursor+=1;
                exp_part_bs.push(b);
            }
        }

        if internally_int {
            let int_part_s = str::from_utf8(&int_part_bs).unwrap();
            let value = i64::from_str_radix(int_part_s, 10).unwrap();
            let kind = TokenKind::NumericConstant(NumericConstant::Integer(value));
            let span = &self.view[..cursor];
            self.view = &self.view[cursor..];
            
            return Some(Token::new(kind, Span(span)));
        }

        let s = {
            let mut s = String::new();
            if int_part_bs.len() > 0 {
                let int_part_s = str::from_utf8(&int_part_bs).unwrap();
                s.push_str(&int_part_s);
            } else {
                s.push('0');
            }
            if frac_part_bs.len() > 0 {
                let frac_part_s = str::from_utf8(&frac_part_bs).unwrap();
                s.push('.');
                s.push_str(&frac_part_s);
            } else {
                s.push('0');
            }
            if exp_part_bs.len() > 0 {
                let exp_part_s = str::from_utf8(&exp_part_bs).unwrap();
                s.push('e');
                s.push_str(&exp_part_s);
            }

            s
        };
        
        let value = s.parse::<f64>().unwrap();
        let kind = TokenKind::NumericConstant(NumericConstant::Float(value));
        let span = &self.view[..cursor];
        self.view = &self.view[cursor..];
        
        return Some(Token::new(kind, Span(span)));
    }
    /*  Hexadecimal constants:
            MUST have an integer or fractional part.
            If it has an exponent marker 'p'|'P' then it must have an exponent part.
            Integer and fractional parts are hexadecimal, exponent part is decimal.
        --Valid--
        0x0.0p0
        0x0.p0
        0x.0p0
        0x0.0
        0x.0
        0x0.
        0x.0p0
        --Invalid--
        0xp0
        0x
        0x.
        0x.p0
        0x.0p
        0x0.0p*/
    fn lex_hexadecimal_numeric_constant(&mut self, bytes: &'i [u8], cursor: usize) -> Option<Token<'i>> {
        let mut cursor = cursor;
        let mut internally_int = true;
        let mut int_part_b = Vec::new();
        let mut frac_part_b = Vec::new();
        let mut exp_part_b = Vec::new();

        while let Some(&b) = bytes.get(cursor) {
            if !b.is_ascii_hexdigit() {
                break;
            }
            cursor+=1;
            int_part_b.push(b);
        }

        if bytes.get(cursor) == Some(&b'.') {
            internally_int = false;
            cursor+=1;

            while let Some(&b) = bytes.get(cursor) {
                if !b.is_ascii_hexdigit() {
                    break;
                }
                cursor+=1;
                frac_part_b.push(b);
            }
        }

        if int_part_b.len() == 0 && frac_part_b.len() == 0 {
            panic!("hex constant needs integer part or fractional part!");
        }

        if let Some(&(b'p'|b'P')) = bytes.get(cursor) {
            internally_int = false;
            cursor+=1;

            if let Some(&s @ (b'-' | b'+')) = bytes.get(cursor) {
                cursor+=1;
                exp_part_b.push(s);
            }

            let mut added_exp_digit = false;
            while let Some(&b) = bytes.get(cursor) {
                if !b.is_ascii_digit() {
                    break;
                }
                cursor+=1;
                added_exp_digit = true;
                exp_part_b.push(b);
            }
            if !added_exp_digit {
                panic!("Malformed exponent");
            }
        }

        if internally_int {
            let value = i64::from_str_radix(str::from_utf8(&int_part_b).unwrap(), 16).unwrap();
            let kind = TokenKind::NumericConstant(NumericConstant::Integer(value));
            let span = &self.view[..cursor];
            self.view = &self.view[cursor..];
            
            return Some(Token::new(kind, Span(span)));
        }

        let s = {
            let mut s= String::from("0x");

            if int_part_b.len() > 0 {
                let int_part_s = str::from_utf8(&int_part_b).expect("Invalid utf8 in integer part");
                s.push_str(int_part_s);
            } else {
                s.push('0');
            }
            s.push('.');
            if frac_part_b.len() > 0 {
                let frac_part_s = str::from_utf8(&frac_part_b).expect("Invalid utf8 in fractional part");
                s.push_str(frac_part_s);
            } else {
                s.push('0');
            }
            s.push('p');
            if exp_part_b.len() > 0 {
                let exp_part_s = str::from_utf8(&exp_part_b).expect("Invalid utf8 in exponent part");
                s.push_str(&exp_part_s);
            } else {
                s.push_str("0");
            }

            s
        };
        let value = hexf_parse::parse_hexf64(&s, false).unwrap();
        let kind = TokenKind::NumericConstant(NumericConstant::Float(value));
        let span = &self.view[..cursor];
        self.view = &self.view[cursor..];
        
        return Some(Token::new(kind, Span(span)));
    }
}
