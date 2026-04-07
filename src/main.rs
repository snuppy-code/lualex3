use crate::lexer_types::{Lexer, Span, TokenKind};

mod lexer_tests;
mod lexer_types;
mod util;

fn main() {
   let a = "0xA3Fb.F31fAp-14";
   let a = "0xF.Fp-3 and";
   // let a = "0x0.  ";
   let mut b = Lexer::new(a);
   b.lex_to_end();
}


/*

*/