#[derive(Debug, PartialEq)]
pub struct LiteralString(String);
impl LiteralString {
    pub fn from_already_escaped(s: &str) -> Self {
        LiteralString(String::from(s))
    }
    pub fn from_escape_short(s: &str) -> Self {
        let mut res = Vec::with_capacity(s.len());
        let mut bytes = s.bytes().peekable();

        while let Some(b) = bytes.next() {
            if b != b'\\' {
                res.push(b);
                continue;
            }
            let escape_c = bytes.next().expect("Trailing slash should have been caught by string lexing, not by escaping !");
            match escape_c {
                b'a' => res.push(b'\x07'),
                b'b' => res.push(b'\x08'),
                b'f' => res.push(b'\x0C'),
                b'n' => res.push(b'\x0A'), //lua 5.3 manual says newline, but see https://www.lua.org/source/5.3/llex.c.html `read_string`, just \n in c code, which is line feed
                b'r' => res.push(b'\x0D'),
                b't' => res.push(b'\x09'),
                b'v' => res.push(b'\x0B'),
                b'\\'=> res.push(b'\\'),
                b'\"'=> res.push(b'\"'),
                b'\''=> res.push(b'\''),
                b'\n'=> res.push(b'\n'), //ouhh platform dependent? unsure if this will work consistently

                b'z' => {
                    while bytes.peek().is_some() &&
                          bytes.peek().unwrap().is_ascii_whitespace() { //might be a problem, unicode whitespace
                        bytes.next().unwrap();
                    }
                },

                b'x' => {
                    let d1 = bytes.next().expect("Unfinished \\x escape sequence at end of string!");
                    let d2 = bytes.next().expect("Unfinished \\x escape sequence at end of string!");

                    let digits = [d1,d2];
                    let tmp_s = str::from_utf8(&digits).expect("Invalid utf8? in \\x escape sequence");
                    let v = u8::from_str_radix(tmp_s, 16).expect("Invalid hex, maybe too large? in \\x escape sequence!");

                    res.push(v);
                },

                b'u' => {
                    let open_brace = bytes.next();
                    if !(open_brace.is_some() && open_brace.unwrap() == b'{') {
                        panic!("Invalid \\u escape sequence! Missing opening brace")
                    }

                    let mut usv_hex: u32 = 0;
                    let mut found_closing = false;
                    let mut encountered_digit = false;

                    while let Some(c) = bytes.next() {
                        if c == b'}' {
                            found_closing = true;
                            break;
                        }
                        if !c.is_ascii_hexdigit() {
                            panic!("Invalid character in \\u escape sequence!");
                        }
                        let d = (c as char).to_digit(16).unwrap();
                        usv_hex = (usv_hex << 4) | d;
                        encountered_digit = true;
                    }

                    if !found_closing {
                        panic!("Invalid \\u escape sequence! Missing closing brace")
                    }
                    if !encountered_digit {
                        panic!("Invalid \\u escape sequence! No digits")
                    }
                    // I'm reasonably sure the limit is 0x10FFFF, not 0x7FFFFFFF in 5.3. I won't test it in 5.3 💔, it's just what I gather from the lua source code and testing in 5.4
                    // see:   readutf8esc   https://www.lua.org/source/5.3/llex.c.html   https://www.lua.org/source/5.4/llex.c.html/
                    if usv_hex > 0x10FFFF {
                        panic!("Invalid \\u escape sequence! beyond lua 5.3 0x10FFFF limit!");
                    }

                    let usv = char::from_u32(usv_hex)
                        .expect("Invalid \\u escape sequence! Provided hex aint valid USV! this error should be impossible!");
                    let mut tmp_s = [0u8;4];
                    res.extend_from_slice(usv.encode_utf8(&mut tmp_s).as_bytes());
                },

                d @ b'0'..=b'9' => {
                    let mut decimals_b = [d,0,0];
                    let mut len = 1;

                    while len < 3 &&
                          bytes.peek().is_some() &&
                          bytes.peek().unwrap().is_ascii_digit() {
                        
                        decimals_b[len] = bytes.next().unwrap();
                        len += 1;
                    }
                    let s = str::from_utf8(&decimals_b[..len]).unwrap();
                    let v = u8::from_str_radix(s, 10).expect("Invalid decimal escape! out of u8 range");
                    res.push(v);
                },

                _ => panic!("invalid escape :/"),
            };
        }

        LiteralString(String::from_utf8(res).unwrap())
    }
    pub fn from_escape_long(s: &str) -> Self {
        let mut res = Vec::with_capacity(s.len());
        let bytes = s.as_bytes();
        let mut cursor = 0;
        const LF: u8 = 10;
        const CR: u8 = 13;

        // For convenience, when the opening long bracket is immediately followed by a newline, the newline is not included in the string.
        if let Some(&b @ (LF | CR)) = bytes.get(cursor) {
            if let Some(&b2 @ (LF | CR)) = bytes.get(cursor+1) && b2 != b {
                cursor+=2;
            } else {
                cursor+=1;
            }
        }

        // Any kind of end-of-line sequence (carriage return, newline, carriage return followed by newline, or newline followed by carriage return) is converted to a simple newline.
        while let Some(&b) = bytes.get(cursor) {
            cursor+=1;
            let (LF | CR) = b else {
                res.push(b);
                continue;
            };

            if let Some(&b2 @ (LF | CR)) = bytes.get(cursor) && b2 != b {
                cursor+=1;
            }
            res.push(LF);
        }

        LiteralString(String::from_utf8(res).unwrap())
    }
}