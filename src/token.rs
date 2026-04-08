use crate::token_kind::TokenKind;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span<'i>(pub &'i str);

#[derive(Debug, PartialEq)]
pub struct Token<'i> {
    kind: TokenKind,
    span: Span<'i>,
}
impl<'i> Token<'i> {
    pub fn new(kind: TokenKind, span: Span<'i>) -> Self {
        return Token { kind, span };
    }
    pub fn get_kind(&self) -> &TokenKind {
        return &self.kind;
    }
    pub fn take_kind(self) -> TokenKind {
        self.kind
    }
    pub fn get_span(&self) -> Span<'i> {
        return self.span;
    }
}