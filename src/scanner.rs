use crate::tokens::Token;


struct Scanner {

    source: String,
    tokens: Vec<Token>
}

impl Scanner {

    fn new(source: &str) -> Self {

        Self {
            source: source.to_string(),
            tokens: Vec::new(),
        }
    }
}

