#![allow(unused)]

use crate::lexer::{DfaState, Token, TokenReader, TokenType};
use crate::lexer::TokenType::Plus;

#[cfg(test)]
mod tests {
    use super::SimpleLexer;

    #[test]
    pub fn test() {
        let lexer = SimpleLexer::new();
        let script = "int age = 45;";
        println!("parse = {}", script);
        let mut token_reader = lexer.tokenize(script);
        lexer.dump(&mut token_reader);

        let lexer = SimpleLexer::new();
        let script = "inta age = 45;";
        println!("parse = {}", script);
        let mut token_reader = lexer.tokenize(script);
        lexer.dump(&mut token_reader);

        let lexer = SimpleLexer::new();
        let script = "in age = 45;";
        println!("parse = {}", script);
        let mut token_reader = lexer.tokenize(script);
        lexer.dump(&mut token_reader);


        let lexer = SimpleLexer::new();
        let script = "age >= 45;";
        println!("parse = {}", script);
        let mut token_reader = lexer.tokenize(script);
        lexer.dump(&mut token_reader);


        let lexer = SimpleLexer::new();
        let script = "age > 45;";
        println!("parse = {}", script);
        let mut token_reader = lexer.tokenize(script);
        lexer.dump(&mut token_reader);

        let lexer = SimpleLexer::new();
        let script = "2+3*5";
        println!("parse = {}", script);
        let mut token_reader = lexer.tokenize(script);
        lexer.dump(&mut token_reader);
    }
}




#[derive(Debug)]
pub struct SimpleLexer {
    // pub active: bool,
    // username: String,
    // email: String,
    // sign_in_count: u64,
}


impl SimpleLexer {
    pub fn new() -> Self {
        SimpleLexer {}
    }

    pub fn tokenize(&self, script: &str) -> SimpleTokenReader {
        // let chars: Vec<char> = script.chars().collect();

        let mut state: DfaState = DfaState::Initial;
        let mut token_text = String::new();
        let mut chars = script.chars().peekable();
        let mut tokens: Vec<Box<dyn Token>> = Vec::new();
        let mut token = SimpleToken::new();

        while let Some(ch) = chars.next() {
            state = match state {
                DfaState::Initial => {
                    let x = init_token(ch, &mut token_text, &mut tokens, token);
                    token = x.1;
                    x.0
                }
                DfaState::Id => {
                    if ch.is_alphabetic() || ch.is_ascii_digit() {
                        token_text.push(ch);
                        DfaState::Id
                    } else {
                        let x = init_token(ch, &mut token_text, &mut tokens, token);
                        token = x.1;
                        x.0
                    }
                }
                DfaState::GT => {
                    if ch == '=' {
                        token.token_type = Some(TokenType::GE);
                        token_text.push(ch);
                        DfaState::GE
                    } else {
                        let x = init_token(ch, &mut token_text, &mut tokens, token);
                        token = x.1;
                        x.0
                    }
                }
                DfaState::GE | DfaState::Assignment | DfaState::Plus | DfaState::Minus | DfaState::Star |
                DfaState::Slash | DfaState::SemiColon | DfaState::LeftParen | DfaState::RightParen => {
                    let x = init_token(ch, &mut token_text, &mut tokens, token);
                    token = x.1;
                    x.0
                }
                DfaState::IntLiteral => {
                    if ch.is_ascii_digit() {
                        token_text.push(ch);
                        DfaState::IntLiteral
                    } else {
                        let x = init_token(ch, &mut token_text, &mut tokens, token);
                        token = x.1;
                        x.0
                    }
                }
                DfaState::IdInt1 => {
                    if ch == 'n' {
                        token_text.push(ch);
                        DfaState::IdInt2
                    } else {
                        let x = init_token(ch, &mut token_text, &mut tokens, token);
                        token = x.1;
                        x.0
                    }
                }
                DfaState::IdInt2 => {
                    if ch == 't' {
                        token_text.push(ch);
                        DfaState::IdInt3
                    } else {
                        let x = init_token(ch, &mut token_text, &mut tokens, token);
                        token = x.1;
                        x.0
                    }
                }
                DfaState::IdInt3 => {
                    if ch.is_ascii_whitespace() {
                        token.token_type = Some(TokenType::Int);
                        let x = init_token(ch, &mut token_text, &mut tokens, token);
                        token = x.1;
                        x.0
                    } else {
                        token_text.push(ch);
                        DfaState::Id
                    }
                }
                _ => panic!("Unhandled State!"),
            }
        }

        if !token_text.is_empty() {
            let x = init_token('_', &mut token_text, &mut tokens, token);
            token = x.1;
        }

        SimpleTokenReader::new(tokens)
    }

    pub fn dump(&self, token_reader: &mut SimpleTokenReader) {
        println!("text\ttype");
        while let Some(token) = token_reader.read() {
            println!("{}\t\t{:?}", token.get_text(), token.get_type());
        }

        println!(" ")
    }
}


fn init_token(ch: char,
              token_text: &mut String,
              tokens: &mut Vec<Box<dyn Token>>,
              mut token: SimpleToken) -> (DfaState, SimpleToken) {
    if !token_text.is_empty() {
        token.text = token_text.clone();
        tokens.push(Box::new(token));

        token_text.clear();
        token = SimpleToken::new()
    }

    let mut new_state = DfaState::Initial;


    token_text.push(ch);

    match ch {
        ch if ch.is_alphabetic() => {
            if ch == 'i' {
                new_state = DfaState::IdInt1;
            } else {
                new_state = DfaState::Id;
            }
            token.token_type = Some(TokenType::Identifier);
        }
        ch if ch.is_ascii_digit() => {
            new_state = DfaState::Id;
            token.token_type = Some(TokenType::IntLiteral);
        }
        '>' => {
            new_state = DfaState::GT;
            token.token_type = Some(TokenType::GT);
        }
        '+' => {
            new_state = DfaState::Plus;
            token.token_type = Some(TokenType::Plus);
        }
        '-' => {
            new_state = DfaState::Minus;
            token.token_type = Some(TokenType::Minus);
        }
        '*' => {
            new_state = DfaState::Star;
            token.token_type = Some(TokenType::Star);
        }
        '/' => {
            new_state = DfaState::Slash;
            token.token_type = Some(TokenType::Slash);
        }
        ';' => {
            new_state = DfaState::SemiColon;
            token.token_type = Some(TokenType::SemiColon);
        }
        '(' => {
            new_state = DfaState::LeftParen;
            token.token_type = Some(TokenType::LeftParen);
        }
        ')' => {
            new_state = DfaState::RightParen;
            token.token_type = Some(TokenType::RightParen);
        }
        '=' => {
            new_state = DfaState::Assignment;
            token.token_type = Some(TokenType::Assignment);
        }
        _ => {
            new_state = DfaState::Initial;
            token_text.pop();
        }
    }


    (new_state, token)
}


// ------------------------- SimpleToken -------------------------
pub struct SimpleToken {
    token_type: Option<TokenType>,
    text: String,
}

impl SimpleToken {
    pub fn new() -> Self {
        SimpleToken {
            token_type: None,
            text: String::new(),
        }
    }

    pub fn new2(tt: TokenType, txt: String) -> Self {
        SimpleToken {
            token_type: Some(tt),
            text: txt,
        }
    }
}


impl Token for SimpleToken {
    fn get_type(&self) -> TokenType {
        // self.token_type.unwrap_or(TokenType::Unknown)
        self.token_type.unwrap()
    }

    fn get_text(&self) -> &str {
        &self.text
    }
}
// ------------------------- SimpleToken -------------------------


// ------------------------- SimpleTokenReader -------------------------

pub struct SimpleTokenReader {
    tokens: Vec<Box<dyn Token>>,
    pos: usize,
}

impl SimpleTokenReader {
    fn new(tokens: Vec<Box<dyn Token>>) -> SimpleTokenReader {
        SimpleTokenReader {
            tokens,
            pos: 0,
        }
    }
}

impl TokenReader for SimpleTokenReader {
    fn read(&mut self) -> Option<&Box<dyn Token>> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            self.tokens.get(self.pos - 1)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&Box<dyn Token>> {
        if self.pos < self.tokens.len() {
            self.tokens.get(self.pos)
        } else {
            None
        }
    }

    fn unread(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    fn get_position(&self) -> usize {
        self.pos
    }

    fn set_position(&mut self, position: usize) {
        if position < self.tokens.len() {
            self.pos = position;
        }
    }
}

// ------------------------- SimpleTokenReader -------------------------


