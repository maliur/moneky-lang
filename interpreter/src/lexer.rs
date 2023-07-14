pub mod token;

pub struct Lexer<'a> {
    position: usize,
    read_position: usize,
    ch: &'a str,
    pub input: &'a str,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 1,
            ch: "",
        };

        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position - 1 >= self.input.len() {
            self.ch = "";
        } else {
            self.ch = &self.input[self.position..self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;

        match self.ch {
            "=" => tok = new_token(token::TokenType::ASSIGN, self.ch),
            "," => tok = new_token(token::TokenType::COMMA, self.ch),
            "" => tok = new_token(token::TokenType::EOF, self.ch),
            "{" => tok = new_token(token::TokenType::LBRACE, self.ch),
            "(" => tok = new_token(token::TokenType::LPAREN, self.ch),
            "+" => tok = new_token(token::TokenType::PLUS, self.ch),
            "}" => tok = new_token(token::TokenType::RBRACE, self.ch),
            ")" => tok = new_token(token::TokenType::RPAREN, self.ch),
            ";" => tok = new_token(token::TokenType::SEMICOLON, self.ch),
            _ => tok = new_token(token::TokenType::ILLEGAL, self.ch),
        }

        self.read_char();
        tok
    }
}

fn new_token(token_type: token::TokenType, ch: &str) -> token::Token {
    token::Token {
        token_type,
        literal: ch,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token().token_type, token::TokenType::ASSIGN);
        assert_eq!(l.next_token().token_type, token::TokenType::PLUS);
        assert_eq!(l.next_token().token_type, token::TokenType::LPAREN);
        assert_eq!(l.next_token().token_type, token::TokenType::RPAREN);
        assert_eq!(l.next_token().token_type, token::TokenType::LBRACE);
        assert_eq!(l.next_token().token_type, token::TokenType::RBRACE);
        assert_eq!(l.next_token().token_type, token::TokenType::COMMA);
        assert_eq!(l.next_token().token_type, token::TokenType::SEMICOLON);
    }
}
