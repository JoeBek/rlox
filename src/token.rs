use crate::token_type::TokenType;
use std::fmt::Display;


struct Token {

    token_type: TokenType,
    lexeme: String,
    literal: Box<dyn Display>,
    line: u32,


}

impl Token {

    // constructor for token
    fn new(token_type:TokenType, lexeme: &str, literal: Box<dyn Display>, line: u32) -> Self {

        Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line
        }

    }
    
}

impl Display for Token {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let literal_content = *self.literal;
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal_content);
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

        let token = Token::new(token_type, lexeme, literal, line);

        let check = String::from("EQUAL = idk 0");

        assert_eq!(token.to_string(), check);


    }
}