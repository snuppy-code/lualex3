use crate::{keyword::Keyword, literalstring::LiteralString, numeric_constant::NumericConstant, symbol::Symbol, token::{Span, Token}, token_kind::TokenKind};

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
    pub fn iter_tokens(&'_ self) -> core::slice::Iter<'_,Token<'i>> {
        self.tokens.iter()
    }
    pub fn iter_mut_tokens(&mut self) -> std::slice::IterMut<'_, Token<'i>> {
        self.tokens.iter_mut()
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
        while self.skip_whitespace() || self.skip_comment() {};

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
    pub fn skip_whitespace(&mut self) -> bool {
        let l1 = self.view.len();
        self.view = self.view.trim_start();
        return l1 != self.view.len()
    }
    pub fn lex_identifier_or_keyword(&mut self) -> Option<Token<'i>> {
        let bytes = self.view.as_bytes();

        let &b = bytes.first()?;
        if !(b.is_ascii_alphabetic() || b == b'_') {
            return None;
        }

        let n = bytes.iter()
            .take_while(|&&b| b.is_ascii_alphanumeric() || b == b'_')
            .count();
        
        let something_str = &self.view[..n];
        self.view = &self.view[n..];

        let kind = match something_str {
            "and" => TokenKind::Keyword(Keyword::And),
            "break" => TokenKind::Keyword(Keyword::Break),
            "do" => TokenKind::Keyword(Keyword::Do),
            "elseif" => TokenKind::Keyword(Keyword::Elseif),
            "else" => TokenKind::Keyword(Keyword::Else),
            "end" => TokenKind::Keyword(Keyword::End),
            "false" => TokenKind::Keyword(Keyword::False),
            "for" => TokenKind::Keyword(Keyword::For),
            "function" => TokenKind::Keyword(Keyword::Function),
            "goto" => TokenKind::Keyword(Keyword::Goto),
            "if" => TokenKind::Keyword(Keyword::If),
            "in" => TokenKind::Keyword(Keyword::In),
            "local" => TokenKind::Keyword(Keyword::Local),
            "nil" => TokenKind::Keyword(Keyword::Nil),
            "not" => TokenKind::Keyword(Keyword::Not),
            "or" => TokenKind::Keyword(Keyword::Or),
            "repeat" => TokenKind::Keyword(Keyword::Repeat),
            "return" => TokenKind::Keyword(Keyword::Return),
            "then" => TokenKind::Keyword(Keyword::Then),
            "true" => TokenKind::Keyword(Keyword::True),
            "until" => TokenKind::Keyword(Keyword::Until),
            "while" => TokenKind::Keyword(Keyword::While),
            _ => TokenKind::Identifier,
        };

        Some(Token::new(kind, Span(something_str)))
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
        let bytes = self.view.as_bytes();
        
        let Some(&quote @ (b'\"' | b'\'')) = bytes.get(0) else {
            return None;
        };

        let mut last_q_pos = 1;
        let mut found_end = false;
        while let Some(new_q_pos) = bytes.iter().enumerate().position(|(i,&b)| b==quote && i>last_q_pos) {
            
            // imprecise lower bound, which is okay because we look backwards through the iterator.
            let bslashes = (&bytes[last_q_pos..new_q_pos]).iter()
                .rev()
                .take_while(|&&v| v == b'\\')
                .count();

            last_q_pos = new_q_pos;

            if bslashes%2 == 0 {
                found_end = true;
                break;
            }
        }
        if !found_end {
            panic!("Unclosed short string!");
        }
        
        let contents = str::from_utf8(&bytes[1..last_q_pos]).unwrap();
        let span = &self.view[..contents.len()+2];
        let contents_tokenkind = TokenKind::LiteralString(LiteralString::from_escape_short(contents));

        self.view = &self.view[span.len()..];
        Some(Token::new(contents_tokenkind, Span(span)))
    }
    pub fn lex_long_literal_string(&mut self) -> Option<Token<'i>> {
        let bytes = self.view.as_bytes();
        let mut cursor = 0;
        
        let Some(b'[') = bytes.get(cursor) else {
            return None;
        };
        cursor+=1;

        let opening_eq = (&bytes[cursor..]).iter()
            .take_while(|&&v| v == b'=')
            .count();
        cursor+=opening_eq;

        let Some(b'[') = bytes.get(cursor) else {
            return None;
        };
        cursor+=1;


        while let Some(&b) = bytes.get(cursor) {
            cursor+=1;
            if b != b']' {
                continue;
            }
            
            let closing_eq = (&bytes[cursor..]).iter()
                .take_while(|&&v| v == b'=')
                .count();
            cursor+=closing_eq;
            
            if opening_eq != closing_eq {
                continue;
            }

            if let Some(b']') = bytes.get(cursor) {
                cursor+=1;
                let ends_len = opening_eq+2;
                
                let span = &self.view[..cursor];
                let inner_content = &self.view[ends_len..cursor-ends_len];

                self.view = &self.view[cursor..];
                return Some(Token::new(TokenKind::LiteralString(LiteralString::from_escape_long(inner_content)), Span(span)))
            };
        }
        return None;
    }
    pub fn lex_symbol(&mut self) -> Option<Token<'i>> {
        let (sym,sym_len) = match self.view.as_bytes() {
            [b'.', b'.', b'.', ..] => (Symbol::DotDotDot,3),

            [b'<', b'<', ..] => (Symbol::LShift,2),
            [b'>', b'>', ..] => (Symbol::RShift,2),
            [b'/', b'/', ..] => (Symbol::SlashSlash,2),
            [b'=', b'=', ..] => (Symbol::EqualsEquals,2),
            [b'~', b'=', ..] => (Symbol::NotEquals,2),
            [b'<', b'=', ..] => (Symbol::LessOrEquals,2),
            [b'>', b'=', ..] => (Symbol::GreaterOrEquals,2),
            [b':', b':', ..] => (Symbol::ColonColon,2),
            [b'.', b'.', ..] => (Symbol::DotDot,2),

            [b'+', ..] => (Symbol::Plus, 1),
            [b'-', ..] => (Symbol::Dash, 1),
            [b'*', ..] => (Symbol::Star, 1),
            [b'/', ..] => (Symbol::Slash, 1),
            [b'%', ..] => (Symbol::Percent, 1),
            [b'^', ..] => (Symbol::Caret, 1),
            [b'#', ..] => (Symbol::Hashtag, 1),
            [b'&', ..] => (Symbol::Ampersand, 1),
            [b'~', ..] => (Symbol::Tilde, 1),
            [b'|', ..] => (Symbol::Pipe, 1),
            [b'<', ..] => (Symbol::LessThan, 1),
            [b'>', ..] => (Symbol::GreaterThan, 1),
            [b'=', ..] => (Symbol::Equals, 1),
            [b'(', ..] => (Symbol::LParen, 1),
            [b')', ..] => (Symbol::RParen, 1),
            [b'{', ..] => (Symbol::LCurly, 1),
            [b'}', ..] => (Symbol::RCurly, 1),
            [b'[', ..] => (Symbol::LBracket, 1),
            [b']', ..] => (Symbol::RBracket, 1),
            [b';', ..] => (Symbol::Semicolon, 1),
            [b':', ..] => (Symbol::Colon, 1),
            [b',', ..] => (Symbol::Comma, 1),
            [b'.', ..] => (Symbol::Dot, 1),

            _ => return None,
        };

        let sym_str = &self.view[..sym_len];
        self.view = &self.view[sym_len..];

        Some(Token::new(TokenKind::Symbol(sym), Span(sym_str)))
    }
    fn skip_comment(&mut self) -> bool {
        let bytes = self.view.as_bytes();
        
        if !bytes.starts_with(b"--") {
            return false;
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
            return true;
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
                return true;
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
        if let [b'e'|b'E', ..] = &bytes[cursor..] {
            cursor+=1;
            if let &[sign @ (b'+'|b'-'), ..] = &bytes[cursor..] {
                cursor+=1;
                exp_part_bs.push(sign);
            }
            while let Some(&b) = bytes.get(cursor) {
                cursor+=1;
                if !b.is_ascii_digit() {
                    break;
                }
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
