use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LBraces,
    RBraces,
    LBracket,
    RBracket,
    LParenthesis,
    RParenthesis,
    Colon,
    Comma,
    String(String),
    Boolean(bool),
    Null,
    Unknown(char),
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        // skip whitespace
        let next_char = loop {
            let ch = self.chars.next()?;
            if !ch.is_whitespace() {
                break ch;
            }
        };

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
            '0'..='9' => Some(self.read_number(next_char)),
            '"' => Some(self.read_string()),
            't' | 'f' | 'n' => self.read_keyword(next_char),
            _ => Some(Token::Unknown(next_char)),
        }
    }

    fn read_number(&mut self, first: char) -> Token {
        //transforms char into number
        let mut number = first.to_digit(10).unwrap() as i32;
        //until char is a number run this
        while let Some(&ch) = self.chars.peek() {
            if let Some(digit) = ch.to_digit(10) {
                number = number * 10 + digit as i32; //handle multi digit number 
                self.chars.next();
            } else {
                break;
            }
        }
        Token::Number(number)
    }

    fn read_string(&mut self) -> Token {
        let mut content = String::new();
        //while char is still a string
        while let Some(ch) = self.chars.next() {
            if ch == '"' {
                return Token::String(content); //handle unclosed quotes by returning string  
            }
            content.push(ch);
        }
       Token::Unknown('"')
    }

    fn read_keyword(&mut self, first: char) -> Option<Token> {
        //adds firts char to buff
        let mut buf = String::new();
        buf.push(first);

        while let Some(&ch) = self.chars.peek() {
            //adds all letter chars to buff 
            if ch.is_ascii_alphabetic() {
                buf.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }

        match buf.as_str() {
            "true" => Some(Token::Boolean(true)),
            "false" => Some(Token::Boolean(false)),
            "null" => Some(Token::Null),
            _ => Some(Token::Unknown(first)),
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
        #[test]
    fn test_numbers() {
        let input = "42 007 1234";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Number(42),
                Token::Number(7),   // leading zeros are fine — 007 → 7
                Token::Number(1234),
            ]
        );
    }

    #[test]
    fn test_simple_string() {
        let input = r#""hello""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![Token::String("hello".to_string())]);
    }

    #[test]
    fn test_unclosed_string() {
        let input = r#""hello"#; // missing closing quote
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        // your lexer currently treats it as Token::Unknown('"')
        assert_eq!(tokens, vec![Token::Unknown('"')]);
    }

    #[test]
    fn test_booleans_and_null() {
        let input = "true false null";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Boolean(true),
                Token::Boolean(false),
                Token::Null,
            ]
        );
    }

    #[test]
    fn test_unknown_keyword() {
        let input = "truth";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        // “truth” is not a valid JSON keyword, should mark first char as unknown
        assert_eq!(tokens, vec![Token::Unknown('t')]);
    }

    #[test]
    fn test_operators_and_punctuation() {
        let input = "+ - * / : , { } [ ] ( )";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Plus,
                Token::Minus,
                Token::Multiply,
                Token::Divide,
                Token::Colon,
                Token::Comma,
                Token::LBraces,
                Token::RBraces,
                Token::LBracket,
                Token::RBracket,
                Token::LParenthesis,
                Token::RParenthesis,
            ]
        );
    }

    #[test]
    fn test_mixed_json_like_structure() {
        let input = r#"[{"id":1,"ok":false},null]"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        let expected = vec![
            Token::LBracket,
            Token::LBraces,
            Token::String("id".to_string()),
            Token::Colon,
            Token::Number(1),
            Token::Comma,
            Token::String("ok".to_string()),
            Token::Colon,
            Token::Boolean(false),
            Token::RBraces,
            Token::Comma,
            Token::Null,
            Token::RBracket,
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert!(tokens.is_empty());
    }

}
