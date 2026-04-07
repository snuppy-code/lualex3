use crate::lexer_types::{Lexer, Span, TokenKind};

mod lexer_tests;
mod lexer_types;
mod util;

fn main() {
   let a = "0x345 0xff 0xBEBADA 0xA3Fb.F31fAp-14";
   let mut b = Lexer::new(a);
   b.lex_to_end();
}


/*

*/