use crate::lexer::Token;
use std::{iter::Peekable};

#[derive(Debug, PartialEq)]
pub enum Type {
    Object(Vec<(String, Type)>),
    Array(Vec<Type>),
    String(String),
    Number(i32),
    Boolean(bool),
    Null,
}

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}
//parser takes in a stream of tokens (produced through lexer) and turns it into a data structure, Val enum,
//where each token represents a piece of JSON syntax

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    // Constructor
    pub fn new(iter: I) -> Self {
        Parser {
            tokens: iter.peekable(),
        }
    }

    // Peek at next token without consuming
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    // Consume next token
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn parse(&mut self) -> Result<Type, String> {
    let value = self.parse_val()?;
    if self.peek().is_some() {
        return Err("Extra tokens after valid JSON value".into());
    }
    Ok(value)
}

    //decides what kind are we parsing next, parsing the next value in the token stream
    pub fn parse_val(&mut self) -> Result<Type, String> {
        match self.next() {
            Some(Token::Number(n)) => Ok(Type::Number(n)),
            Some(Token::String(s)) => Ok(Type::String(s)),
            Some(Token::Boolean(b)) => Ok(Type::Boolean(b)),
            Some(Token::Null) => Ok(Type::Null),
            Some(Token::LBraces) => self.parse_object(),
            Some(Token::LBracket) => self.parse_array(),
            Some(tok) => Err(format!("Unexpected token: {:?}", tok)),
            None => Err("Unexpected end of input".into()),
        }
    }
    //Parses object structure {} from token. Expects key-value pairs where each key is a string, followed by :, and a value. Pairs are seprated by ,
    //and ends with a }
    pub fn parse_object(&mut self) -> Result<Type, String> {
      
        let mut kv_pairs = Vec::new();
        if self.peek() == Some(&Token::RBraces) {
            self.next();
            return Ok(Type::Object(kv_pairs));
        }
        loop {
            //next value is a string and consume (extract the value) and assign to key
            let key = match self.next() {
                Some(Token::String(s)) => s,
                _ => return Err("Expected string key in object".into()),
            };

            //next value is a colon and consume
            match self.next() {
                Some(Token::Colon) => {}
                _ => return Err("Expected ':' after key".into()),
            }

            //parse value
            let value = self.parse_val()?;
            kv_pairs.push((key, value));

            //handle comma, closing braces
            match self.peek() {
                Some(Token::Comma) => {
                    self.next();
                }
                Some(Token::RBraces) => {
                    self.next();
                    break;
                }
                _ => return Err("Expected ',' or '}' after pair".into()),
            }
        }
        Ok(Type::Object(kv_pairs))
    }
    //Parses an array structure [] from token stream. Values are separated by , and enclose [], each element is parsed using parse_val, and nested objects
    pub fn parse_array(&mut self) -> Result<Type, String>{
      
        let mut vals:Vec<Type> = Vec::new();

        if self.peek() == Some(&Token::RBracket) {
            self.next();
            return Ok(Type::Array(vals));
        }

        loop {
            let value = self.parse_val()?;
            vals.push(value);

            match self.peek() {
                Some(Token::Comma) => {
                    self.next(); //consume and go on
                }
                Some(Token::RBracket) => {
                    self.next();
                    break;
                }
                _ => return  Err("Expected ',' or ']' after an array value".into())
            }
        }

        Ok(Type::Array(vals))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    fn parse_tokens(tokens: Vec<Token>) -> Result<Type, String> {
        let mut parser = Parser::new(tokens.into_iter());
        parser.parse_val()
    }

    #[test]
    fn test_parse_number() {
        let tokens = vec![Token::Number(42)];
        let result = parse_tokens(tokens).unwrap();
        assert_eq!(result, Type::Number(42));
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".into())];
        let result = parse_tokens(tokens).unwrap();
        assert_eq!(result, Type::String("hello".into()));
    }

    #[test]
    fn test_parse_boolean() {
        let tokens = vec![Token::Boolean(true)];
        let result = parse_tokens(tokens).unwrap();
        assert_eq!(result, Type::Boolean(true));
    }

    #[test]
    fn test_parse_null() {
        let tokens = vec![Token::Null];
        let result = parse_tokens(tokens).unwrap();
        assert_eq!(result, Type::Null);
    }

    #[test]
    fn test_parse_empty_object() {
        let tokens = vec![Token::LBraces, Token::RBraces];
        let result = parse_tokens(tokens).unwrap();
        assert_eq!(result, Type::Object(vec![]));
    }

    #[test]
    fn test_parse_object_with_values() {
        let tokens = vec![
            Token::LBraces,
            Token::String("a".into()),
            Token::Colon,
            Token::Number(1),
            Token::Comma,
            Token::String("b".into()),
            Token::Colon,
            Token::Boolean(false),
            Token::RBraces,
        ];
        let result = parse_tokens(tokens).unwrap();
        assert_eq!(
            result,
            Type::Object(vec![
                ("a".into(), Type::Number(1)),
                ("b".into(), Type::Boolean(false))
            ])
        );
    }

    #[test]
    fn test_parse_empty_array() {
        let tokens = vec![Token::LBracket, Token::RBracket];
        let result = parse_tokens(tokens).unwrap();
        assert_eq!(result, Type::Array(vec![]));
    }

    #[test]
    fn test_parse_array_with_values() {
        let tokens = vec![
            Token::LBracket,
            Token::Number(1),
            Token::Comma,
            Token::String("x".into()),
            Token::Comma,
            Token::Boolean(true),
            Token::RBracket,
        ];
        let result = parse_tokens(tokens).unwrap();
        assert_eq!(
            result,
            Type::Array(vec![
                Type::Number(1),
                Type::String("x".into()),
                Type::Boolean(true)
            ])
        );
    }

    #[test]
    fn test_parse_nested_structures() {
        let tokens = vec![
            Token::LBraces,
            Token::String("arr".into()),
            Token::Colon,
            Token::LBracket,
            Token::Number(1),
            Token::Comma,
            Token::Number(2),
            Token::Comma,
            Token::LBraces,
            Token::String("x".into()),
            Token::Colon,
            Token::Null,
            Token::RBraces,
            Token::RBracket,
            Token::RBraces,
        ];

        let result = parse_tokens(tokens).unwrap();
        assert_eq!(
            result,
            Type::Object(vec![(
                "arr".into(),
                Type::Array(vec![
                    Type::Number(1),
                    Type::Number(2),
                    Type::Object(vec![("x".into(), Type::Null)])
                ])
            )])
        );
    }

    #[test]
    fn test_parse_object_missing_colon_error() {
        let tokens = vec![Token::LBraces, Token::String("a".into()), Token::Number(1)];
        let result = parse_tokens(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_array_missing_comma_error() {
        let tokens = vec![Token::LBracket, Token::Number(1), Token::Number(2), Token::RBracket];
        let result = parse_tokens(tokens);
        assert!(result.is_err());
    }
}
