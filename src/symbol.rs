use crate::{
    lexer_errors::LexerErrorKind,
    token::{Span, Token},
    token_kind::TokenKind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    //  +               -           *               /                   %           ^               #
    //  &               ~           |               <<                  >>          //
    //  ==              ~=          <=              >=                  <           >               =
    //  (               )           {               }                   [           ]               ::
    //  ;               :           ,               .                   ..          ...
    Plus,
    Dash,
    Star,
    Slash,
    Percent,
    Caret,
    Hashtag,
    Ampersand,
    Tilde,
    Pipe,
    LShift,
    RShift,
    SlashSlash,
    EqualsEquals,
    NotEquals,
    LessOrEquals,
    GreaterOrEquals,
    LessThan,
    GreaterThan,
    Equals,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBracket,
    RBracket,
    ColonColon,
    Semicolon,
    Colon,
    Comma,
    Dot,
    DotDot,
    DotDotDot,
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

pub fn lex_symbol<'i>(view: &'i str) -> Result<Option<(Token<'i>, &'i str)>, LexerErrorKind> {
    let (sym, sym_len) = match view.as_bytes() {
        [b'.', b'.', b'.', ..] => (Symbol::DotDotDot, 3),

        [b'<', b'<', ..] => (Symbol::LShift, 2),
        [b'>', b'>', ..] => (Symbol::RShift, 2),
        [b'/', b'/', ..] => (Symbol::SlashSlash, 2),
        [b'=', b'=', ..] => (Symbol::EqualsEquals, 2),
        [b'~', b'=', ..] => (Symbol::NotEquals, 2),
        [b'<', b'=', ..] => (Symbol::LessOrEquals, 2),
        [b'>', b'=', ..] => (Symbol::GreaterOrEquals, 2),
        [b':', b':', ..] => (Symbol::ColonColon, 2),
        [b'.', b'.', ..] => (Symbol::DotDot, 2),

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

        _ => return Ok(None),
    };

    let sym_str = &view[..sym_len];
    let new_view = &view[sym_len..];

    Ok(Some((
        Token::new(TokenKind::Symbol(sym), Span(sym_str)),
        new_view,
    )))
}
