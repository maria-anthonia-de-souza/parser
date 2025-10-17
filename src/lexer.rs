use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LBraces,
    RBraces,
    LBracket, // [
    RBracket, // ]
    LParenthesis, //(
    RParenthesis, //)
    Colon,    // :
    Comma,
    String(String), // "..."
    Boolean(bool),
    Null,
    Unknown(char),
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>, //so that we can look at the next char without cloning the iterator every loop iteration
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }
    //tokenize input string and return vector of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new(); //vector to add token 
        // loop as long as next token returns some(token), when end of token return vector of token
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    //next token of the input string
    fn next_token(&mut self) -> Option<Token> {
        let mut next_char = self.chars.next()?;
        //skip whitespace
        while next_char.is_whitespace(){
            next_char = self.chars.next()?;
        }
        
        match next_char {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '{' => Some(Token::LBraces),
            '}' => Some(Token::RBraces),
            '[' => Some(Token::LBracket), 
            ']' => Some(Token::RBracket),
            '(' => Some(Token::LParenthesis), 
            ')' => Some(Token::RParenthesis), 
            ':' => Some(Token::Colon), 
            ',' => Some(Token::Comma),
            '0'..='9' => {
                let mut number = next_char.to_digit(10)? as i32;
                //continue the loop until the next char is not a digit number 
                while let Some(next_char) = self.chars.peek() {
                    if let Some(digit) = next_char.to_digit(10) {
                        number = number * 10 + digit as i32; //  multi digit numbers 
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                Some(Token::Number(number))
            }
            '"' => {
                let mut content = String::new();
                while let Some(ch) = self.chars.next() {
                    if ch == '"' {
                        break;
                    }
                    content.push(ch);
                }
                Some(Token::String(content))
            }
              // Boolean or null
            't' => {
                let mut buf = String::from("t");
                //if the next three chars spell true, it returns a bool token with value true 
                for _ in 0..3 {
                    if let Some(c) = self.chars.next() {
                        buf.push(c); //add chars to buffer 
                    }
                }
                if buf == "true" {
                    Some(Token::Boolean(true))
                } else {
                    None
                }
            }
            'f' => {
                let mut buf = String::from("f");
                for _ in 0..4 {
                    if let Some(c) = self.chars.next() {
                        buf.push(c);
                    }
                }
                if buf == "false" {
                    Some(Token::Boolean(false))
                } else {
                    None
                }
            }
            'n' => {
                let mut buf = String::from("n");
                for _ in 0..3 {
                    if let Some(c) = self.chars.next() {
                        buf.push(c);
                    }
                }
                if buf == "null" {
                    Some(Token::Null)
                } else {
                    None
                }
            }
            _ => Some(Token::Unknown(next_char)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_lexer() {
        let input = r#"{ "key": 123, "active": true, "items": [1, 2, null] }"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let expected = vec![
            Token::LBraces,
            Token::String("key".to_string()),
            Token::Colon,
            Token::Number(123),
            Token::Comma,
            Token::String("active".to_string()),
            Token::Colon,
            Token::Boolean(true),
            Token::Comma,
            Token::String("items".to_string()),
            Token::Colon,
            Token::LBracket,
            Token::Number(1),
            Token::Comma,
            Token::Number(2),
            Token::Comma,
            Token::Null,
            Token::RBracket,
            Token::RBraces,
        ];

        assert_eq!(tokens, expected);
    }
}
