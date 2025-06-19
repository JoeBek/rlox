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

    /// advances current iterator and returns option for next char
    fn advance(&mut self) -> Option<char> {

        // map the iterator i,c to return the character, increase the char index.
        let opt = {
            self.iter_from_start().next().map(|(_,c)| c)
        };
        self.current += 1; 
        opt 

    }

    /// advance with no advancement
    fn peek(&mut self) -> Option<char> {

        // map the iterator i,c to return the character, increase the char index.
        let opt = {
            self.iter_from_current().next().map(|(_,c)| c)
        };
        opt 

    }

    /// advances only if the char returns a 
    fn check_and_advance(&mut self, c:char) -> Option<char> {
        
        let opt = {
            self.iter_from_current().next().map(|(_,c)| c)
        };

        match opt {
            // conditional advance
            Some(k) => {
                if k == c {
                    self.current += 1;
                    opt
                }
                else {
                    None // was Some(k), now is None. We narrow the option to Some(c) only
                }
            },
            None => None // equivalent to returning opt in this case, but I prefer to be clear

        }
    }



    


    

    fn scan_token(&mut self) {




        let c = if let Some(ch) = self.advance() {
            ch
        } 
        else {
            return;
        };


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
            '!' => {
                
                if let Some(_) = self.check_and_advance('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                }
                else {
                    self.add_token(TokenType::BANG)
                }
            },
            '=' => {
                
                if let Some(_) = self.check_and_advance('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                }
                else {
                    self.add_token(TokenType::EQUAL)
                }
            },
            '<' => {
                
                if let Some(_) = self.check_and_advance('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                }
                else {
                    self.add_token(TokenType::LESS)
                }
            },
            '>' => {
                
                if let Some(_) = self.check_and_advance('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                }
                else {
                    self.add_token(TokenType::GREATER)
                }
            },
            '/' => {

                if let Some(_) = self.check_and_advance('/') {

                    // stops at eof or newline
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();

                    }
                }
                else {
                    self.add_token(TokenType::SLASH);
                }
                

            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            
            
            // default is in error
            _ => error::error(self.line, "unexpected char"),

        };


        
    }

    fn match_token(&self, token:char) {

        

    } 

    fn add_token(&mut self, token_type: TokenType) {

        // we can move past end by accident 
        let end_idx: usize;


        // protects advance past end of source
        if self.at_end() {
            
            end_idx = self.source.chars().count() - 1;
            
        }
        else {
            end_idx = self.current;

        }

        let start_byte = self.source.char_indices().nth(self.start).map(|(idx, _)| idx).unwrap();
        let end_byte = self.source.char_indices().nth(end_idx).map(|(idx, _)| idx).unwrap();

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

    #[test]
    fn test_scan_equal() {

        let source = String::from("! = !=");

        let mut scanner = Scanner::new(&source);

        scanner.scan_tokens();

        assert_eq!(3, scanner.tokens.len());
    }

    #[test]
    fn test_scan_paren() {

        let source = String::from("() ( ) (()) \n () ()");

        let mut scanner = Scanner::new(&source);

        scanner.scan_tokens();

        assert_eq!(12, scanner.tokens.len());
    }

    #[test]
    fn test_comment() {

        let source = String::from("() // comment");

        let mut scanner = Scanner::new(&source);

        scanner.scan_tokens();

        assert_eq!(2, scanner.tokens.len());
    }

    #[test]
    fn test_comment_newline() {

        let source = String::from("() // comment \n ()");

        let mut scanner = Scanner::new(&source);

        scanner.scan_tokens();

        assert_eq!(4, scanner.tokens.len());
    }





}



