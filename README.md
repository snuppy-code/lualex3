I'm workin 'ere!

Resources:
- https://www.lua.org/manual/5.3/manual.html
- https://www.lua.org/source/5.3/

Current problems/weaknesses:
- I believe short string \z escape sequence will not successfully skip unicode whitespace
- I don't think short string \n escape sequence will be treated consistently across platforms, I think on windows it will be viewed as CR LF, on linux it will be LF, and on mac it will be CR. This might affect semantic analysis later.
- Floats are funky across platforms so I'm not sure if the evaluation of float numeric constants will have problems on other CPU architectures, if this minifier ever runs on any other than x86. Also, I'm not parsing floats myself, I just do a middle step before passing the lexed numeric constant strings to `parse::<f64>` or the `hexf-parse` crate to actually parse into rust `f64`s. I'm not 100% sure the functions I'm using evaluate floats exactly the same way lua 5.3 does. I'm not good enough at c, don't know its standard library well enough, and don't know niche float issues well enough to have an opinion here. I just know in the tests I did, output matches lua 5.3.

Speed: (criterion)
```
     Running benches/basic_perf_test.rs (target/release/deps/basic_perf_test-3874dc6ed587f255)
Gnuplot not found, using plotters backend
full eiss               time:   [173.24 µs 173.37 µs 173.50 µs]
                        change: [−0.5646% −0.4643% −0.3621%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```
1835 lines of of my old but real lua code lexed in 1.7337e-4 or 0.00017337 seconds. I'm sure that's awfully slow for a lexer in a systems language but I'm happy with it !

Notes to self:
- cargo modules structure --lib
- cargo public-api -sss
- cargo flamegraph --example scratchpad
- cargo bench
- cargo test
- bacon
- echo '-1' | sudo tee /proc/sys/kernel/perf_event_paranoid
- samply record ./target/release/examples/scratchpad
