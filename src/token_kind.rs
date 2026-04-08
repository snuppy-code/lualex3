use crate::{keyword::Keyword, literalstring::LiteralString, numeric_constant::NumericConstant, symbol::Symbol};


#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    LiteralString(LiteralString),
    NumericConstant(NumericConstant),
    Identifier,
}
impl TokenKind {
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