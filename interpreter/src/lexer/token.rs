#[derive(Debug, PartialEq)]
pub enum TokenType {
    ASSIGN,
    COMMA,
    EOF,
    FUNCTION,
    IDENT,
    ILLEGAL,
    INT,
    LBRACE,
    LET,
    LPAREN,
    PLUS,
    RBRACE,
    RPAREN,
    SEMICOLON,
}

impl TokenType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenType::ASSIGN => "=",
            TokenType::COMMA => ",",
            TokenType::EOF => "",
            TokenType::FUNCTION => "FUNCTION",
            TokenType::IDENT => "IDENT",
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::INT => "INT",
            TokenType::LBRACE => "{",
            TokenType::LET => "LET",
            TokenType::LPAREN => "(",
            TokenType::PLUS => "+",
            TokenType::RBRACE => "}",
            TokenType::RPAREN => ")",
            TokenType::SEMICOLON => ";",
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}
