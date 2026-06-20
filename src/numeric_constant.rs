use crate::{lexer_errors::LexerError, token::{Span, Token}, token_kind::TokenKind};

#[derive(Debug, PartialEq)]
pub enum NumericConstant {
    Integer(i64),
    Float(f64),
}
pub fn lex_numeric_constant<'i>(view: &'i str) -> Result<Option<(Token<'i>,&'i str)>,LexerError> {
    let bytes = view.as_bytes();
    
    if bytes.starts_with(b"0x") || bytes.starts_with(b"0X") { 
        return lex_hexadecimal_numeric_constant(&view[2..]);
    }

    match bytes {
        [b'0'..=b'9', ..] | [b'.', b'0'..=b'9', ..] => {
            return lex_decimal_numeric_constant(view);
        },
        _ => return Ok(None),
    };
}

pub fn lex_decimal_numeric_constant<'i>(view: &'i str) -> Result<Option<(Token<'i>,&'i str)>,LexerError> {
    let bytes = view.as_bytes();
    let mut cursor = 0;

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
        return Err(LexerError::DecimalNumericConstantNeedsIntOrFracPart)
    }
    if let [b'e'|b'E', ..] = &bytes[cursor..] {
        cursor+=1;
        if let &[sign @ (b'+'|b'-'), ..] = &bytes[cursor..] {
            cursor+=1;
            exp_part_bs.push(sign);
        }
        let mut added_exp_digit = false;
        while let Some(&b) = bytes.get(cursor) {
            cursor+=1;
            if !b.is_ascii_digit() {
                break;
            }
            added_exp_digit = true;
            exp_part_bs.push(b);
        }
        if !added_exp_digit {
            return Err(LexerError::DecimalNumericConstantMalformedExponent);
        }
    }

    if internally_int {
        let int_part_s = str::from_utf8(&int_part_bs).unwrap();
        let value = i64::from_str_radix(int_part_s, 10).unwrap();
        let kind = TokenKind::NumericConstant(NumericConstant::Integer(value));
        let span = &view[..cursor];
        let new_view = &view[cursor..];
        
        return Ok(Some((Token::new(kind, Span(span)),new_view)));
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
    let span = &view[..cursor];
    let new_view = &view[cursor..];
    
    return Ok(Some((Token::new(kind, Span(span)),new_view)));
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
pub fn lex_hexadecimal_numeric_constant<'i>(view: &'i str) -> Result<Option<(Token<'i>,&'i str)>,LexerError> {
    let bytes = view.as_bytes();
    let mut cursor = 0;

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
        return Err(LexerError::HexNumericConstantNeedsIntOrFracPart)
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
            return Err(LexerError::HexNumericConstantMalformedExponent);
        }
    }

    if internally_int {
        let value = i64::from_str_radix(str::from_utf8(&int_part_b).unwrap(), 16).unwrap();
        let kind = TokenKind::NumericConstant(NumericConstant::Integer(value));
        let span = &view[..cursor];
        let new_view = &view[cursor..];
        
        return Ok(Some((Token::new(kind, Span(span)),new_view)));
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
    let span = &view[..cursor];
    let new_view = &view[cursor..];
    
    return Ok(Some((Token::new(kind, Span(span)),new_view)));
}