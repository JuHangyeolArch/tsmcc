use std::fmt;
use std::error::Error;
use super::Compiler;
use super::CompileError;
use super::token::{Token,TokenList,TokenType};


#[derive(Debug, PartialEq)]
pub struct LexicalAnalyzer {
    text_pos: usize,
    eof_reached: bool,
}

impl LexicalAnalyzer {
    pub fn new() -> Self {
        LexicalAnalyzer {text_pos: 0, eof_reached: false}
    }

    fn is_over(&self) -> bool {
        self.eof_reached
    }


    pub fn step(&mut self,source: &str,token_list: & mut TokenList) -> Option<Result<(),CompileError>> {
        if self.is_over() {
            None
        }
        else {
            match self.get_token(source) {
                Ok(token) => {token_list.push(token); Some(Ok(()))},
                Err(lex_error) => {Some(Err(CompileError::LexError(lex_error)))},
            }
            
        } 
        
    }

    fn get_token(&mut self, source: &str) -> Result<Token,LexError>{

        let c = match self.peek_ch(source) {
            Some(c) => c,
            None =>{
                self.eof_reached = true;
                return Ok(
                    Token::new(
                        TokenType::Eof,
                        self.text_pos,
                        self.text_pos
                    )
                )},
        };

        let token = match c {
            '+' => {self.advance_ch(source);
                Token::new(
                    TokenType::Plus,
                    self.text_pos -1,
                    self.text_pos
            )}
            '-' => {self.advance_ch(source);
                Token::new(
                    TokenType::Minus,
                    self.text_pos -1,
                    self.text_pos
                )},
            '*' => {self.advance_ch(source);
                Token::new(
                    TokenType::Star,
                    self.text_pos -1,
                    self.text_pos
                )},
            '/' => {self.advance_ch(source);
                Token::new(
                    TokenType::Slash,
                    self.text_pos -1,
                    self.text_pos
                )},
            '&' => {self.advance_ch(source);
                Token::new(
                    TokenType::Amper,
                    self.text_pos -1,
                    self.text_pos
                )},
            '(' => {self.advance_ch(source);
                Token::new(
                    TokenType::LParenth,
                    self.text_pos -1,
                    self.text_pos
                )},
            ')' => {self.advance_ch(source);
                Token::new(
                    TokenType::RParenth,
                    self.text_pos -1,
                    self.text_pos
                )},
            '{' => {self.advance_ch(source);
                Token::new(
                    TokenType::LBrace,
                    self.text_pos -1,
                    self.text_pos
                )},
            '}' => {self.advance_ch(source);
                Token::new(
                    TokenType::RBrace,
                    self.text_pos -1,
                    self.text_pos
                )},
            ';' => {self.advance_ch(source);
                Token::new(
                    TokenType::Semic,
                    self.text_pos -1,
                    self.text_pos
                )},
            ',' => {self.advance_ch(source);
                Token::new(
                    TokenType::Comma,
                    self.text_pos -1,
                    self.text_pos
                )},

            '=' =>  {
                let start_pos = self.text_pos;
                
                match self.slice_while(source,|ch| ch == '=').as_str() {
                    "=" => {
                        Token::new(
                            TokenType::Comma,
                            self.text_pos -1,
                            self.text_pos
                        )},
                    "==" => {
                        Token::new(
                            TokenType::Comma,
                            start_pos,
                            self.text_pos,
                        )},
                    _ => {return Err(LexError::UnexpectedError);}
                }
            },

            'a'..='z' | 'A'..='Z' | '_' => {
                let start_pos = self.text_pos;
                
                let lexeme =  self.slice_while(
                    source,
                    |ch| ch.is_alphanumeric() || ch == '_'
                );
                match lexeme.as_str() {
                    "int" => {
                        Token::new(
                            TokenType::Int,
                            start_pos,
                            self.text_pos
                        )},
                    "if" => {
                        Token::new(
                            TokenType::If,
                            start_pos,
                            self.text_pos
                        )},
                    "else" => {
                        Token::new(
                            TokenType::Else,
                            start_pos,
                            self.text_pos
                        )},
                    "while" => {
                        Token::new(
                            TokenType::While,
                            start_pos,
                            self.text_pos
                        )},
                    "return" => {
                        Token::new(
                            TokenType::Return,
                            start_pos,
                            self.text_pos
                        )},
                    _ => {
                        Token::new(
                            TokenType::Id(lexeme),
                            start_pos,
                            self.text_pos
                        )},
                }
            }

            '1'..='9' => {
                let start_pos = self.text_pos;
                let lexeme = self.slice_while(
                    source,
                    |ch| ch.is_ascii_digit()
                );
                Token::new(
                    TokenType::Number(lexeme.parse().unwrap()),
                    start_pos,
                    self.text_pos
                )
                
            }

            c if  c.is_whitespace() => {
                self.advance_ch(source);
                return self.get_token(source);
            }

            _ => {
                return Err(LexError::InvalidToken(self.text_pos));
            }

        };


        Ok(token)
        
    }

    
    fn peek_ch(&self, source: &str) -> Option<char> {
        source.chars().nth(self.text_pos)
    }

    

    fn advance_ch(&mut self, source: &str) {
        if let Some(c) = self.peek_ch(source) {
            self.text_pos += c.len_utf8();
        } 
    }

    fn slice_while<F>(&mut self, source: &str, condition: F) -> String
    where
    F: Fn(char) -> bool
    {
        let start_pos = self.text_pos;

        while let Some(c) = self.peek_ch(source) {
            if condition(c) {
                self.advance_ch(source);                
            } else {
                break;
            }
        }
        source[start_pos..self.text_pos].to_string()
    }

}

#[derive(Debug,Clone,PartialEq)]
pub enum LexError {
    InvalidToken(usize),
    UnexpectedError,
}
