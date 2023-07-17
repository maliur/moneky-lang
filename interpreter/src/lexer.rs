pub mod token;

pub struct Lexer<'a> {
    position: usize,
    next_position: usize,
    ch: u8,
    pub input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            next_position: 0,
            ch: 0,
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.next_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.next_position];
        }
        self.position = self.next_position;
        self.next_position += 1;
    }

    fn read_identifier(&mut self) -> token::Token {
        let position = self.position;
        loop {
            match self.ch {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
        let literal = &self.input[position..self.position];

        match literal {
            "fn" => token::Token::Func,
            "let" => token::Token::Let,
            "true" => token::Token::Bool(true),
            "false" => token::Token::Bool(false),
            "if" => token::Token::If,
            "else" => token::Token::Else,
            "return" => token::Token::Return,
            _ => token::Token::Ident(String::from(literal)),
        }
    }

    fn read_number(&mut self) -> token::Token {
        let position = self.position;
        loop {
            match self.ch {
                b'0'..=b'9' => self.read_char(),
                _ => break,
            }
        }

        let literal = &self.input[position..self.position];
        match literal.parse::<i64>() {
            Ok(number) => token::Token::Int(number),
            _ => token::Token::Illegal,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn peek(&mut self) -> &str {
        if self.next_position + 1 >= self.input.len() {
            ""
        } else {
            &self.input[self.next_position..self.next_position + 1]
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek() == "=" {
                    self.read_char();
                    token::Token::Equal
                } else {
                    token::Token::Assign
                }
            }
            b',' => token::Token::Comma,
            b'{' => token::Token::Lbrace,
            b'(' => token::Token::Lparen,
            b'+' => token::Token::Plus,
            b'}' => token::Token::Rbrace,
            b')' => token::Token::Rparen,
            b'-' => token::Token::Minus,
            b'!' => {
                if self.peek() == "=" {
                    self.read_char();
                    token::Token::NotEqual
                } else {
                    token::Token::Bang
                }
            }
            b'/' => token::Token::Slash,
            b'*' => token::Token::Asterisk,
            b'>' => token::Token::GreaterThan,
            b'<' => token::Token::LessThan,
            b';' => token::Token::Semicolon,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.read_identifier();
            }
            b'0'..=b'9' => {
                return self.read_number();
            }
            0 => token::Token::Eof,
            _ => token::Token::Illegal,
        };

        self.read_char();
        tok
    }
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10
        10 != 9
"#;
        let test_table = vec![
            // let five = 5;
            token::Token::Let,
            token::Token::Ident(String::from("five")),
            token::Token::Assign,
            token::Token::Int(5),
            token::Token::Semicolon,
            // let ten = 10;
            token::Token::Let,
            token::Token::Ident(String::from("ten")),
            token::Token::Assign,
            token::Token::Int(10),
            token::Token::Semicolon,
            // let add = fn(x, y) {
            //     x + y;
            // };
            token::Token::Let,
            token::Token::Ident(String::from("add")),
            token::Token::Assign,
            token::Token::Func,
            token::Token::Lparen,
            token::Token::Ident(String::from("x")),
            token::Token::Comma,
            token::Token::Ident(String::from("y")),
            token::Token::Rparen,
            token::Token::Lbrace,
            token::Token::Ident(String::from("x")),
            token::Token::Plus,
            token::Token::Ident(String::from("y")),
            token::Token::Semicolon,
            token::Token::Rbrace,
            token::Token::Semicolon,
            // let result = add(five, ten);
            token::Token::Let,
            token::Token::Ident(String::from("result")),
            token::Token::Assign,
            token::Token::Ident(String::from("add")),
            token::Token::Lparen,
            token::Token::Ident(String::from("five")),
            token::Token::Comma,
            token::Token::Ident(String::from("ten")),
            token::Token::Rparen,
            token::Token::Semicolon,
            // !-/*5;
            token::Token::Bang,
            token::Token::Minus,
            token::Token::Slash,
            token::Token::Asterisk,
            token::Token::Int(5),
            token::Token::Semicolon,
            // 5 < 10 > 5;
            token::Token::Int(5),
            token::Token::LessThan,
            token::Token::Int(10),
            token::Token::GreaterThan,
            token::Token::Int(5),
            token::Token::Semicolon,
            // if (5 < 10) {
            //     return true;
            // } else {
            //     return false;
            // }
            token::Token::If,
            token::Token::Lparen,
            token::Token::Int(5),
            token::Token::LessThan,
            token::Token::Int(10),
            token::Token::Rparen,
            token::Token::Lbrace,
            token::Token::Return,
            token::Token::Bool(true),
            token::Token::Semicolon,
            token::Token::Rbrace,
            token::Token::Else,
            token::Token::Lbrace,
            token::Token::Return,
            token::Token::Bool(false),
            token::Token::Semicolon,
            token::Token::Rbrace,
            // 10 == 10
            token::Token::Int(10),
            token::Token::Equal,
            token::Token::Int(10),
            // 10 != 9
            token::Token::Int(10),
            token::Token::NotEqual,
            token::Token::Int(9),
            token::Token::Eof,
        ];

        let mut l = Lexer::new(input);

        for expect in test_table {
            let tok = l.next_token();
            assert_eq!(expect, tok);
        }
    }
}
