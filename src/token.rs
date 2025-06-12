use crate::token_type::TokenType; // chapter 7 rust book
use std::fmt::{Display, Formatter, Result};


pub struct Token {

    token_type: TokenType,
    lexeme: String,
    // TODO replace this with sumn
    // literal: Box<dyn Display>,
    line: u32,


}

impl Token {

    // constructor for token
    pub fn new(token_type:TokenType, lexeme: &str, line: u32) -> Self {

        Self {
            token_type,
            lexeme: lexeme.to_string(),
            //literal,
            line
        }

    }
    
}

impl Display for Token {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {

        //let literal_content = *self.literal;
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, self.line)
    }

}

impl Clone for Token {

    fn clone(&self) -> Self {
        
        Self { token_type: self.token_type.clone() , lexeme: self.lexeme.clone(), line: self.line }
    }
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_token_printing() {
        
        let token_type = TokenType::EQUAL;
        let lexeme = "=";
        let literal = Box::new("idk");
        let line = 0;

        let token = Token::new(token_type, lexeme, line);

        let check = String::from("EQUAL = 0");

        assert_eq!(check, token.to_string());


    }
}