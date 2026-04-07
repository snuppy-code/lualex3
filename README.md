I'm workin 'ere!

Resources:
- https://www.lua.org/manual/5.3/manual.html
- https://www.lua.org/source/5.3/

Current problems/weaknesses:
- I believe short string \z escape sequence will not successfully skip unicode whitespace
- I don't think short string \n escape sequence will be treated consistently across platforms, I think on windows it will be viewed as CR LF, on linux it will be LF, and on mac it will be CR. This might affect semantic analysis later.
- Floats are funky across platforms so I'm not sure if the evaluation of float numeric constants will have problems on other CPU architectures, if this minifier ever runs on any other than x86. Also, I'm doing a middle step before passing the lexed numeric constant strings to `parse::<f64>` or the `hexf-parse` crate to actually parse into rust `f64`s, and I'm not 100% sure the functions I'm using evaluate floats exactly the same way lua 5.3 does. I'm not good enough at c, don't know its standard library well enough, and don't know niche float issues well enough to have an opinion here. I just know in the tests I did, output matches lua 5.3.
