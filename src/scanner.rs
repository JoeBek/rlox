use crate::token::Token;
use crate::token_type::TokenType;


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

    fn scan_token(&self) {

        
        
    }

    fn add_token(&mut self, token_type: TokenType) {

        let start_byte = self.source.char_indices().nth(self.start).map(|(idx, _)| idx).unwrap();
        let end_byte = self.source.char_indices().nth(self.current).map(|(idx, _)| idx).unwrap();

        let lexeme = &self.source[start_byte..end_byte].to_string();


        self.tokens.push(Token::new(token_type, lexeme, self.line));

    } 

    


}



#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_iterator_construction() {

        let source = String::from("hello");

        let mut scanner = Scanner::new(&source);
        let equal_token = TokenType::EQUAL;

        scanner.add_token(equal_token);




        assert_eq!(1, scanner.tokens.len());

    }
}



