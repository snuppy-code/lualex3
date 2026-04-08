use std::{fs, hint::black_box};
use criterion::{criterion_group, criterion_main, Criterion};
use luide3::lexer::Lexer;

fn criterion_benchmark(c: &mut Criterion) {
    let names = [
        "./example_code/Electronic Vision Assistance A.lua",
        "./example_code/Emission Tracking System A.lua",
        "./example_code/KAI 10A Guidance.lua",
        "./example_code/Tactical Situation Display A.lua",
        "./example_code/Target Viewing Camera.lua",
        "./example_code/Track While Scan A.lua",
        "./example_code/vamperion fcs 2.lua",
        "./example_code/Vamperion Hardpoint Controller A.lua",
    ];
    let full_code = names.iter()
        .map(|&name| fs::read_to_string(name).unwrap())
        .fold(String::new(), |a,v| a+" "+&v);

    c.bench_function("full eiss", |b| b.iter(|| {
        let mut l = Lexer::new(black_box(&full_code));
        l.lex_to_end();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);