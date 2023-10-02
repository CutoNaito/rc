use std::fs;
use std::convert::TryInto;
use rc::token;

fn main() {
    let source: String = fs::read_to_string("./test").unwrap();

    pub fn substr(input: &String, start_pos: i32, end_pos: i32) -> String {
        let mut result: String = String::from("");

        let mut i: i32 = start_pos;
        while i <= end_pos {
            result.push(input.chars().nth(i.try_into().unwrap()).unwrap());
            i += 1;
        }

        return result;
    }

    pub struct Tokenizer {
        input: String,
        position: i32,
        next_position: i32,
        ch: char,
    }

    impl Tokenizer {
        pub fn checker(&mut self) -> token::Token {
            let reserv_tok: Vec<token::Token> = token::reserved_tokens();
            let mut token: token::Token = token::Token {
                tok_type: None,
                tok_lit: None
            };
                
            if self.ch.is_alphabetic() {
                let ident = self.read_ident();

                for t in reserv_tok {
                    if ident == t.tok_lit.unwrap() {
                        token = token::Token {
                            tok_type: t.tok_type,
                            tok_lit: t.tok_lit
                        };
                        break;
                    }
                }

                if token.tok_type == None && token.tok_lit == None {
                    token = token::create_token("IDENT", ident.as_str());
                }

                return token;
            }

            if self.ch.is_digit(10) {
                let number = self.read_number();
                token = token::create_token("INT", number.as_str());

                return token;
            }

            else { return token::create_token("ILLEGAL", "0") }
        }

        pub fn read_token(&mut self) -> token::Token {
            let sl_tok: Vec<token::Token> = token::single_line_tokens();

            self.skip_whitespace();

            let mut token: token::Token = token::Token {
                tok_type: None,
                tok_lit: None
            };

            for t in sl_tok.iter() {
                if self.ch == t.tok_lit.unwrap().chars().nth(0).unwrap() {
                    token = token::Token {
                        tok_type: t.tok_type,
                        tok_lit: t.tok_lit
                    };
                    break;
                }
            }

            if token.tok_lit == None && token.tok_type == None {
                token = self.checker();
            }

            return token;
        }

        pub fn read_char(&mut self) {
            if self.next_position >= self.input.len().try_into().unwrap() {
                self.ch = '0';
            } else {
                self.ch = self.input.chars().nth(self.next_position.try_into().unwrap()).unwrap();
            }

            self.position = self.next_position;
            self.next_position += 1;
        }

        pub fn skip_whitespace(&mut self) {
            if self.ch.is_whitespace() {
                self.read_char();
            }
        }

        pub fn read_ident(&mut self) -> String {
            let position = self.position;

            while self.ch.is_alphabetic() {
                self.read_char();
            }

            return substr(&self.input, position, self.position)
        }

        pub fn read_number(&mut self) -> String {
            let position = self.position;

            while self.ch.is_digit(10) {
                self.read_char();
            }

            return substr(&self.input, position, self.position)
        }
    }
}
