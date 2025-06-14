use crate::token::Token;
use crate::token_type::TokenType;
use crate::error;



struct Scanner {

    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,

}

impl Scanner {

    fn new(source: &str) -> Self {

        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    
    /// returns the iterator from the current char index. 
    /// 
    /// This is necessary to prevent problems with storing the iterator in the struct
    /// due to borrowing rules
    fn iter_from_start(&self) -> impl Iterator<Item = (usize, char)> + '_ {

        self.source.chars().enumerate().skip(self.start)
    }

     fn iter_from_current(&self) -> impl Iterator<Item = (usize, char)> + '_ {

        self.source.chars().enumerate().skip(self.current)
    }

   
    pub fn scan_tokens(&mut self) -> Vec<Token> { 

        while !self.at_end() {

            // reset the iterator index
            self.start = self.current;

            self.scan_token();

        }

        self.tokens.clone()

        
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn scan_token(&mut self) {

        let c = {
            let mut iter = self.iter_from_start();
            match iter.next() {
                Some((_, ch)) => ch,
                None => return, // return for at end (no char)
            }
        };

        self.current += 1;

        

        match c {
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '(' => self.add_token(TokenType::LEFT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            // default is in error
            _ => error::error(self.line, "unexpected char"),
        };

        
    }

    fn match_token(&self, token:char) {

        

    } 

    fn add_token(&mut self, token_type: TokenType) {

        let start_byte = self.source.char_indices().nth(self.start).map(|(idx, _)| idx).unwrap();
        let end_byte = self.source.char_indices().nth(self.current).map(|(idx, _)| idx).unwrap();

        let lexeme = &self.source[start_byte..end_byte];


        self.tokens.push(Token::new(token_type, lexeme, self.line));

    } 



}



#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_token_add() {

        let source = String::from("hello");

        let mut scanner = Scanner::new(&source);
        let equal_token = TokenType::EQUAL;

        scanner.add_token(equal_token);




        assert_eq!(1, scanner.tokens.len());

    }

    #[test]
    fn test_iter_create() {

        let source = String::from("hello");

        let mut scanner = Scanner::new(&source);
        let equal_token = TokenType::EQUAL;

        scanner.add_token(equal_token);

        let mut iter = scanner.iter_from_start();
        
        let (i, c) = iter.next().unwrap();

        assert_eq!('h', c);


    }
}



