use crate::token::{Token, Literal};
use crate::token_type::{TokenType};
use crate::error;
use crate::scanner_utils::*;

fn keyword_token_type(ident: &str) -> Option<TokenType> {
    match ident {
        "and" => Some(TokenType::AND),
        "class" => Some(TokenType::CLASS),
        "else" => Some(TokenType::ELSE),
        "false" => Some(TokenType::FALSE),
        "for" => Some(TokenType::FOR),
        "fun" => Some(TokenType::FUN),
        "if" => Some(TokenType::IF),
        "nil" => Some(TokenType::NIL),
        "or" => Some(TokenType::OR),
        "print" => Some(TokenType::PRINT),
        "return" => Some(TokenType::RETURN),
        "super" => Some(TokenType::SUPER),
        "this" => Some(TokenType::THIS),
        "true" => Some(TokenType::TRUE),
        "var" => Some(TokenType::VAR),
        "while" => Some(TokenType::WHILE),
        _ => None,
    }
}

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

    fn peek_next(&mut self) -> Option<char> {
        let opt = {

            let mut iter = self.iter_from_current(); 
            if iter.next().is_none() {
                return None;
            }
            iter.next().map(|(_,c)| c)
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
            '"' => self.handle_string(),
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            
            
            // default is in error
            _ if is_digit(c) => {

                self.handle_number();

            },
            _ if is_alpha(c) => {
                self.handle_identifier();
            },

            _ => {
                error::error(self.line, "unexpected char")
            },

        };



    }

    fn handle_number(&mut self) {

      let mut dot = false;

      
      while let Some(c) = self.peek() {
        if !is_digit(c) {
            break;
        } 
        self.advance();

      }

      

      if matches!(self.peek(),Some('.'))  {

        dot = true;

        if let Some(k) = self.peek_next() {

        
        if is_digit(k) {
            
            self.advance();
            while matches!(self.peek(), Some(c) if is_digit(c)) {
                self.advance();
            }        
            }
        }
     }

      
      // parse start and end byte from source, convert to f64, yada yada
      let start_byte = self.source.char_indices().nth(self.start).map(|(i,_)| i).unwrap();
      // a little sketchy
      let end_byte = self.source.char_indices().nth(self.current - 1).map(|(i,_)| i).unwrap() + 1;

      let str_literal = self.source[start_byte..end_byte].to_string();

      // should never panic if the scanner works.. we just parsed it


      let literal: Literal;
      if dot {
          let dub: f32 = str_literal.parse().unwrap();
          literal = Literal::FloatLiteral(dub);

      }
      else {
        let integer: u32 = str_literal.parse().unwrap();
        literal = Literal::IntLiteral(integer);
      }

      self.add_token_literal(TokenType::NUMBER, literal);


        
    }

    fn handle_string(&mut self) {
       while let Some(c) = self.peek() {

         // check to see if string terminated
         if c == '"' {
            break;
         }

         if c == '\n' {
            self.line += 1;
         }

         self.advance();

       } 

       
       if self.at_end() {
            error::error(self.line, "unterminated string");
            return;
       }

       // consume second quote
       self.advance();
       
       let start = self.start + 1;
       let end = self.current - 1;

       let start_byte = self.source.char_indices().nth(start).map(|(i,_)| i).unwrap();
       let end_byte = self.source.char_indices().nth(end).map(|(i,_)| i).unwrap();

       let literal = self.source[start_byte..end_byte].to_owned();

       self.add_token_literal(TokenType::STRING, Literal::StringLiteral(literal));

    }

    fn handle_identifier(&mut self) {

        // advance while alphanumeric
        while let Some(c) = self.peek() {
            if !is_alphanumeric(c) {
                break;
            }
            self.advance();
        }
       // get identifier
       let start_byte = self.source.char_indices().nth(self.start).map(|(i,_)| i).unwrap();
       // I think it works..
       let end_byte = self.source.char_indices().nth(self.current - 1).map(|(i,_)| i + 1).unwrap();
        
        let text = &self.source[start_byte..end_byte];

        if let Some(ttype) = keyword_token_type(text) {
            self.add_token(ttype);
        }
        else {
            self.add_token(TokenType::IDENTIFIER);
        }


    }


    fn add_token(&mut self, token_type: TokenType) {

        self.add_token_literal(token_type, Literal::None);
    } 

    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal) {

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


        self.tokens.push(Token::new(token_type,  literal ,lexeme, self.line));

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
        
        let (_, c) = iter.next().unwrap();

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

    #[test]
    fn test_token_string_literal() {

        let source = String::from("\"hello\"");

        let mut scanner = Scanner::new(&source);

        scanner.scan_tokens();

        assert_eq!(1, scanner.tokens.len());

        let item = &scanner.tokens[0];

        let literal = &item.literal;

        assert!(literal.is_some());

        let s: &str = literal.as_ref();
        assert_eq!(s, "hello");

    }

    #[test]
    fn test_token_number_literal() {

        let source = String::from("11.0 11");

        let mut scanner = Scanner::new(&source);

        scanner.scan_tokens();

        assert_eq!(2, scanner.tokens.len());

        let item = &scanner.tokens[0];

        let literal = &item.literal;

        assert!(literal.is_some());

        let f: &f32 = literal.as_ref();
        let compare: f32 = 11.0;
        assert_eq!(*f, compare);

    }

    #[test]
    fn test_keyword() {

        let source = String::from("and");

        let mut scanner = Scanner::new(&source);

        scanner.scan_tokens();

        assert_eq!(1, scanner.tokens.len());

        let item = &scanner.tokens[0];

        let ttype = item.get_type();

        assert_eq!(TokenType::AND, ttype);
    }

    #[test]
    fn test_identifier() {

        let source = String::from("andi");

        let mut scanner = Scanner::new(&source);

        scanner.scan_tokens();

        assert_eq!(1, scanner.tokens.len());

        let item = &scanner.tokens[0];

        let ttype = item.get_type();

        assert_eq!(TokenType::IDENTIFIER, ttype);
    }





    


}



