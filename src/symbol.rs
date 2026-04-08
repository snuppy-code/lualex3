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