use crate::lexer_types::{Lexer, Span, TokenKind};

mod lexer_tests;
mod lexer_types;

fn main() {
   let a = "0xA3Fb.F31fAp-234";
   let mut b = Lexer::new(a);
   b.lex_to_end();
}


/*

*/