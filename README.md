I'm workin 'ere!


Resources:
- https://www.lua.org/manual/5.3/manual.html
- https://www.lua.org/source/5.3/.


Current problems/weaknesses:
- I believe short string \z escape sequence will not successfully skip unicode whitespace
- I don't think short string \n escape sequence will be treated consistently across platforms, I think on windows it will be viewed as CR LF, on linux it will be LF, and on mac it will be CR. This should only affect eventual "string interning" during minifying.
- I'm unsure if floating point numeric constants will be viewed consistently across platforms by the minifier. Unsure if this will even matter for anything