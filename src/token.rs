use crate::token_type::TokenType; // chapter 7 rust book
use std::fmt::{Display, Formatter, Result};


#[derive(Debug, Clone)]
pub enum Literal {

    NumberLiteral(f64),
    StringLiteral(String),
    None
    
}

impl Literal {

    pub fn is_some(&self) -> bool {
      !matches!(self, Literal::None) 
    }
}

impl AsRef<str> for Literal {
    fn as_ref(&self) -> &str {
        match self {   
            Literal::StringLiteral(s) => &s,

            _ => "",
        }
    }
}

impl AsRef<f64> for Literal {

    fn as_ref(&self) -> &f64 {

        match self {
            Literal::NumberLiteral(f) => &f,

            _ => &0.0
        }
    }
}




#[derive(Debug)]
pub struct Token {

    token_type: TokenType,
    lexeme: String,
    // TODO replace this with sumn
    pub literal: Literal,
    line: u32,


}

impl Token {

    // constructor for token
    pub fn new(token_type:TokenType, literal: Literal, lexeme: &str, line: u32) -> Self {

        Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal: literal,
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
        
        Self { token_type: self.token_type.clone() , literal: self.literal.clone(),  lexeme: self.lexeme.clone(), line: self.line }
    }
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_token_printing() {
        
        let token_type = TokenType::EQUAL;
        let lexeme = "=";
        let literal = Literal::StringLiteral(String::from("literally"));
        let line = 0;

        let token = Token::new(token_type, literal, lexeme, line);

        let check = String::from("EQUAL = 0");

        assert_eq!(check, token.to_string());


    }
}