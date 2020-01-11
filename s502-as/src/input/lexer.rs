use super::super::ir::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

// wrapper over code,
// second member's last element is the current file, line number, column position
// to copy into tokens
struct Reader<'a>(Peekable<Chars<'a>>, Vec<(String, u32, u32)>);

impl Reader<'_> {
    // get the next character, skipping whitespace and updating the line number
    fn next(&mut self) -> Option<char> {
        while let Some(c) = self.0.next() {
            match c {
                '\n' => {
                    return Some(c);
                }
                _ if c.is_whitespace() => {
                    self.1.last_mut().unwrap().2 += 1;
                }
                _ => {
                    self.1.last_mut().unwrap().2 += 1;
                    return Some(c);
                }
            }
        }
        None
    }

    // Takes a series of characters starting with `first` into a string as long as a condition is met.
    // Returns the string and whether it ended at the end of the line.
    fn take_while<P>(&mut self, first: char, pred: P) -> String
    where
        P: Fn(&char) -> bool,
    {
        let mut word = String::new();
        word.push(first);

        while let Some(c) = self.0.peek() {
            if *c == '\n' {
                return word;
            }
            if pred(c) {
                word.push(*c);
                self.1.last_mut().unwrap().2 += 1;
                self.0.next();
            } else {
                break;
            }
        }
        word
    }

    fn err(&self, msg: &str) -> ! {
        let current_pos = self.1.last().unwrap();
        eprintln!(
            "error at {}:{}:{}: {}",
            current_pos.0, current_pos.1, current_pos.2, msg
        );
        std::process::exit(1);
    }

    fn take_string(&mut self, first: char) -> TokenType {
        let string = self.take_while(first, |c| *c != '"');
        if let Some('"') = self.next() {
            return TokenType::Str(string);
        } else {
            self.err("unclosed string");
        }
    }

    fn take_number(&mut self, first: char, radix: u32) -> TokenType {
        let num = self.take_while(first, |c| c.is_digit(radix));
        let (max_len, byte_len) = match radix {
            16 => (4, 2),
            8 => (6, 3),
            2 => (16, 8),
            _ => (5, 3),
        };
        if num.len() > max_len {
            self.err("number is too large");
        }
        let val = u16::from_str_radix(&num, radix).unwrap_or_else(|_| {
            self.err("failed to parse number");
        });
        if radix != 10 {
            self.1.last_mut().unwrap().2 += 1;
        }
        if val > 255 || num.len() > byte_len {
            return TokenType::Word(val);
        } else {
            return TokenType::Byte(val as u8);
        }
    }
}

pub fn tokenize(source_file: String) -> Peekable<std::vec::IntoIter<Token>> {
    let code =
        std::fs::read_to_string(&source_file).expect(&format!("error reading {}", &source_file));
    let mut reader = Reader(code.chars().peekable(), vec![(source_file, 1, 0)]);
    let mut tokens = Vec::new();

    // to be less repetitive
    macro_rules! token_push {
        ($val:expr) => {{
            let current_pos = reader.1.last().unwrap();
            let pos = (current_pos.0.clone(), current_pos.1, current_pos.2 - 1);
            let val = $val;
            if val == TokenType::Newline {
                reader.1.last_mut().unwrap().1 += 1;
                reader.1.last_mut().unwrap().2 = 0;
            }
            tokens.push(Token { pos: pos, val: val });
        }};
    }

    while let Some(c) = reader.next() {
        match c {
            '\n' => {
                reader.1.last_mut().unwrap().2 += 1;
                token_push!(TokenType::Newline);
            }
            '#' => token_push!(TokenType::Pound),
            '<' => token_push!(TokenType::Langle),
            '>' => token_push!(TokenType::Rangle),
            '(' => token_push!(TokenType::Lparen),
            ')' => token_push!(TokenType::Rparen),
            '.' => token_push!(TokenType::Period),
            ',' => token_push!(TokenType::Comma),
            ':' => token_push!(TokenType::Colon),
            '!' => match reader.0.peek() {
                Some('!') => {
                    token_push!(TokenType::VisGlobal);
                    reader.next();
                }
                _ => token_push!(TokenType::VisFile),
            },
            ';' => {
                reader.take_while(c, |_| true);
                //reader.1.last_mut().unwrap().2 += 1;
            }
            '$' => {
                if let Some(c) = reader.0.next() {
                    token_push!(reader.take_number(c, 16));
                } else {
                    reader.err("expected number");
                }
            }
            '@' => {
                if let Some(c) = reader.0.next() {
                    token_push!(reader.take_number(c, 8));
                } else {
                    reader.err("expected number");
                }
            }
            '%' => {
                if let Some(c) = reader.0.next() {
                    token_push!(reader.take_number(c, 2));
                } else {
                    reader.err("expected number");
                }
            }
            _ if c.is_digit(10) => {
                token_push!(reader.take_number(c, 10));
            }
            '"' => {
                if let Some(c) = reader.next() {
                    token_push!(reader.take_string(c));
                } else {
                    reader.err("expected string");
                }
            }
            _ if c.is_alphabetic() => {
                let word = reader.take_while(c, |c| c.is_alphabetic() || *c == '_');

                match &word.to_lowercase() as &str {
                    "a" => token_push!(TokenType::A),
                    "x" => token_push!(TokenType::X),
                    "y" => token_push!(TokenType::Y),
                    "inl" => {
                        // let include_file = if let Some('"') = reader.next() {
                        //     if let Some(c) = reader.next() {
                        //         if let TokenType::Str(s) = reader.take_string(c) {
                        //             s
                        //         } else {
                        //             reader.err("unreachable code, take_string is guaranteed to return a Str");
                        //         }
                        //     } else {
                        //         reader.err("expected string");
                        //     }
                        // } else {
                        //     reader.err("expected string");
                        // };

                        let include_file = reader
                            .next()
                            .filter(|c| *c == '"')
                            .and_then(|_| reader.next())
                            .and_then(|c| {
                                if let TokenType::Str(s) = reader.take_string(c) {
                                    Some(s)
                                } else {
                                    reader.err("unreachable code");
                                }
                            })
                            .unwrap_or_else(|| reader.err("expected string"));

                        let mut include_path = std::env::current_dir()
                            .expect("failed to get current working directory");
                        include_path.push(include_file);
                        tokens.extend(tokenize(String::from(include_path.to_str().unwrap())))
                    }
                    _ => token_push!(TokenType::Ident(word)),
                }
            }
            _ => {}
        }
    }

    reader.1.last_mut().unwrap().2 += 1;
    token_push!(TokenType::Newline);
    tokens.into_iter().peekable()
}

#[cfg(test)]
speculate! {
    use TokenType::*;

    before {
        let mut code_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("failed to get cargo manifest dir"));
        code_path.push("test-files/test.65a");
        let code_file = String::from(code_path.to_str().unwrap());
        // only care about token types
        let tokens = tokenize(code_file.clone());
        let token_types: Vec<TokenType> = tokenize(code_file).map(|tok| tok.val).collect();
    }

    it "should read tokens" {
        println!("read tokens:");
        for tok in tokens.collect::<Vec<Token>>() {
            println!("{:?}", tok);
        }

        assert_eq!(token_types, [Byte(0x25), Word(0x30), Byte(0x10)]);
    }
}
