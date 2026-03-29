use crate::lexer_types::{Lexer, Span, TokenKind};

mod lexer_tests;
mod lexer_types;

fn main() {
   // let mut a = "abcb";
   // let mut i = 1;
   // while !a.is_empty() {
   //    println!("{a}");
   //    a = &a[1..];
      
   //    // println!("{i}");
   //    // i+=1;
   //    // if let Some(b) = a.find('b') {
   //    //    println!("`{}`",a);
   //    //    a = &a[b..];
   //    //    println!("`{}`",a);
   //    //    a = a.strip_prefix('b').unwrap();
   //    //    println!("`{}`",a);
   //    // } else {
   //    //    break;
   //    // }
   // }

   let a = "\"\\u{1F445}\\123\\x3A\\z \nb\\\"c\\\'§\\\\\"";
   let mut b = Lexer::new(a);
   b.lex_to_end();
   dbg!(b);

   // let mut a = Lexer::new("[=[ abc\nabc]=]");
   // a.lex_to_end();
   // assert_eq!(a.get_internal_view(), "");
   // assert_eq!(a.get_internal_lexed().len(), 1);
   // assert_eq!(dbg!(a.get_internal_lexed()[0].get_kind()), dbg!(&TokenKind::LiteralString(" abc\nabc")));
   // assert_eq!(a.get_internal_lexed()[0].get_span(), Span("[=[ abc\nabc]=]"));

   // let a = "abc";
   // let b = a.find('b').unwrap();
   // println!("{b}");
   // println!("{}",&a[b..]);
}


/*

*/