use std::{fs, hint::black_box};
use criterion::{criterion_group, criterion_main, Criterion};
use lualex3::lexer::Lexer;

fn criterion_benchmark(c: &mut Criterion) {
    
    let full_code = fs::read_dir("example_lua_code/").unwrap()
        .filter_map(|v| v.map(|e| e.path()).ok())
        .map(|p| std::fs::read_to_string(p).unwrap())
        .fold(String::new(), |a,v| a+" "+&v);

    c.bench_function("full eiss", |b| b.iter(|| {
        let mut l = Lexer::new(black_box(&full_code));
        l.lex_to_end();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);