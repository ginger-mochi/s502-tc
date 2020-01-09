use super::super::ir::token::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

// wrapper over code,
// second member's last element is the current file + line number position
// to copy into tokens
struct Reader<'a>(Peekable<Chars<'a>>, Vec<(String, u32)>);

impl Reader<'_> {
    // get the next character, skipping whitespace and updating the line number
    fn next(&mut self) -> Option<char> {
        while let Some(c) = self.0.next() {
            match c {
                // next line
                '\n' => self.1.last_mut().unwrap().1 += 1,
                _ if c.is_whitespace() => {}
                _ => return Some(c),
            }
        }
        None
    }

    fn take_while<P>(&mut self, first: char, pred: P) -> String
    where
        P: Fn(&char) -> bool,
    {
        let mut word = String::new();
        word.push(first);

        while let Some(c) = self.0.peek() {
            if pred(c) {
                word.push(*c);
            } else {
                return word;
            }
            self.0.next();
        }
        word
    }
}

pub fn tokenize(source_file: String) -> std::iter::Peekable<std::vec::IntoIter<Token>> {
    let code =
        std::fs::read_to_string(&source_file).expect(&format!("error reading {}", &source_file));
    let mut reader = Reader(code.chars().peekable(), vec![(source_file, 0)]);
    let mut tokens = Vec::new();

    // to be less repetitive
    macro_rules! token_push {
        ($val:expr) => {{
            let current_pos = reader.1.last().unwrap();
            tokens.push(Token {
                pos: (current_pos.0.clone(), current_pos.1),
                val: $val,
            });
        }};
    }

    while let Some(c) = reader.next() {
        match c {
            _ if c.is_alphabetic() => {
                let word = reader.take_while(c, |c| c.is_alphabetic());

                match &word as &str {
                    "a" | "A" => token_push!(TokenType::A),
                    "x" | "X" => token_push!(TokenType::X),
                    "y" | "Y" => token_push!(TokenType::Y),
                    _ => token_push!(TokenType::Ident(word)),
                }
            }
            _ => {}
        }
    }
    tokens.into_iter().peekable()
}

#[cfg(test)]
speculate! {
    use TokenType::*;

    before {
        let mut code_path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR"));
        code_path.push("test-files/test.65a");
        let code_file = String::from(code_path.to_str().unwrap());
        // only care about token types
        let tokens: Vec<TokenType> = tokenize(code_file).map(|tok| tok.val).collect();
    }

    it "should read A" {
        assert_eq!(tokens, [A]);
    }
}
