use std::cell::{Ref, RefCell, RefMut};
use std::error::Error;
use std::io;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::lexer::{ASTNode, ASTNodeType, simple_lexer, TokenReader, TokenType};
use crate::simple_calculator;
use crate::simple_calculator::SimpleASTNode;

// use crate::simple_calculator::invalid_input_err;

#[cfg(test)]
mod tests {
    use crate::lexer::{simple_lexer, TokenReader};
    use crate::lexer::simple_lexer::SimpleLexer;
    use crate::lexer::simple_parser::SimpleParser;

    #[test]
    pub fn test() {
        // let script = "int age = 45+2; age= 20; age+10*2;";
        // println!("解析变量甚么语句: {}", script);
        // let lexer = SimpleLexer::new();
        // let mut token_reader = lexer.tokenize(script);
        // lexer.dump(&mut token_reader);

        let script = "int age = 45+2; age= 20; age+10*2;";
        println!("解析变量甚么语句: {}", script);
        let parser = SimpleParser::new();
        let mut result = parser.parse(script);
        match result {
            Ok(root) => root.dump_ast(""),
            Err(e) => println!("parse failed : {}", e),
        }


        let script = "a = 2+3+4;";
        println!("解析变量甚么语句: {}", script);
        let parser = SimpleParser::new();
        let mut result = parser.parse(script);
        match result {
            Ok(root) => root.dump_ast(""),
            Err(e) => println!("parse failed : {}", e),
        }


        let script = "+2;";
        println!("解析变量甚么语句: {}", script);
        let parser = SimpleParser::new();
        let mut result = parser.parse(script);
        match result {
            Ok(root) => root.dump_ast(""),
            Err(e) => println!("parse failed : {}", e),
        }
    }
}


pub struct SimpleParser {}

impl SimpleParser {
    pub fn new() -> SimpleParser {
        SimpleParser {}
    }


    // 解析脚本，并返回根节点
    pub fn parse(&self, code: &str) -> Result<SimpleASTNode, io::Error> {
        let lexer = simple_lexer::SimpleLexer::new();
        let mut tokens = lexer.tokenize(code);
        self.get_root(&mut tokens)
    }

    // 语法解析：根节点
    fn get_root<T: TokenReader>(&self, tokens: &mut T) -> Result<SimpleASTNode, io::Error> {
        let node = SimpleASTNode::new(ASTNodeType::Program, "SimpleParser");


        while !tokens.peek().is_none() {
            // 先看下，是不是 int 变量声明 e.g. int a = 1;
            let mut child = self.int_declare(tokens).expect("get int declare statement failed"); // 整形字面量 node

            if child.is_none() {// 不是 int 变量，看下是不是 普通的表达式。
                child = self.expression_statement(tokens).expect("get expression statement failed");
            }

            if child.is_none() { // 不是表达式，看下是不是赋值语句 e.g.  a = 100;
                child = self.assignment_statement(tokens).expect("get assignment statement failed");
            }

            if child.is_none() {
                return Err(simple_calculator::invalid_input_err("unknown statement"));
            }

            let child = child.unwrap();
            node.add_child(RefCell::new(Rc::new(child)))
        }

        Ok(node)
    }

    // 表达式语句，即表达式后面跟个分号。
    fn expression_statement<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let pos = tokens.get_position();
        let mut node = self.additive(tokens)?;
        if node.is_none() {
            return Ok(node);
        }

        let node = node.unwrap();
        let token = tokens.peek();
        if !token.is_none() && token.unwrap().get_type() == TokenType::SemiColon {
            let _ = tokens.read(); // 消耗分号
            return Ok(Some(node));
        }


        tokens.set_position(pos); // 回溯
        return Ok(None);
    }

    // 赋值语句，如age = 10*2;
    fn assignment_statement<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let token = tokens.peek();
        if token.is_none() || token.unwrap().get_type() != TokenType::Identifier { // 标识符
            return Ok(None);
        }

        // token.type = TokenType::Identifier

        let token = tokens.read().unwrap(); // 消耗标识符
        let mut node = SimpleASTNode::new(ASTNodeType::AssignmentStmt, token.get_text());

        let token = tokens.peek();
        if !token.is_none() && token.unwrap().get_type() == TokenType::Assignment { // =
            let _ = tokens.read(); // 消耗 =
            let child = self.additive(tokens)?;
            if child.is_none() {
                return Err(simple_calculator::invalid_input_err("invalid assignment statement, expecting an expression"));
            }

            let child = child.unwrap();
            node.add_child(RefCell::new(Rc::new(child)));

            let token = tokens.peek();
            if token.is_none() || token.unwrap().get_type() != TokenType::SemiColon {
                return Err(simple_calculator::invalid_input_err("invalid statement, expecting semicolon"));
            }

            let _ = tokens.read(); // 消耗;
            return Ok(Some(node));
        }

        tokens.unread();// 回溯
        return Ok(None);
    }


    /// int_declare 返回整形的字面量 如：
    /// int a;
    ///  int b = 2*3;
    fn int_declare<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let token = tokens.peek();
        if token.is_none() || token.unwrap().get_type() != TokenType::Int {
            return Ok(None);
        }

        //  token.Type = TokenType::Int

        let _ = tokens.read(); // 消耗掉int
        let e = io::Error::new(io::ErrorKind::InvalidInput, "variable name expected");
        let token = tokens.peek().ok_or(e)?;

        if token.get_type() != TokenType::Identifier { // 标识符
            return Err(simple_calculator::invalid_input_err("variable name expected"));
        }

        // token.Type = TokenType::Identifier

        let token = tokens.read().unwrap(); // 消耗掉 Identifier
        let mut node = SimpleASTNode::new(ASTNodeType::IntDeclaration, token.get_text());

        let token = tokens.peek();
        if !token.is_none() && token.unwrap().get_type() == TokenType::Assignment {
            let _ = tokens.read(); // 消耗掉 =

            let e = simple_calculator::invalid_input_err("invalid variable initialization, expecting an expression");
            let child = self.additive(tokens)?.ok_or(e)?;
            node.add_child(RefCell::new(Rc::new(child)));
        }

        let token = tokens.peek();
        if token.is_none() || token.unwrap().get_type() != TokenType::SemiColon {
            return Err(simple_calculator::invalid_input_err("invalid statement, expecting semicolon"));
        }

        let _ = tokens.read(); // 消耗掉 ;

        Ok(Some(node))
    }


    // 加法表达式 add -> mul add' add' -> + mul add' | ε
    // add -> mul (+ mul)*
    fn additive<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let mut child1 = self.multiplicative(tokens)?;
        if child1.is_none() {
            return Ok(None);
        }

        let mut child1 = child1.unwrap();

        loop {
            let token = tokens.peek();
            // if !token.is_none() {
            //     let token_text = token.unwrap().get_text();
            //     println!("additive - loop - token_text : {}", token_text);
            // }


            if token.is_none() || (token.unwrap().get_type() != TokenType::Plus &&
                token.unwrap().get_type() != TokenType::Minus) {
                break;
            }

            // token => + 或者 -
            let token_text = tokens.read().unwrap().get_text();
            let node = SimpleASTNode::new(ASTNodeType::Additive, token_text);
            let e = simple_calculator::invalid_input_err("invalid additive expression, expecting the right part.");
            let child2 = self.multiplicative(tokens)?.ok_or(e)?;

            node.add_child(RefCell::new(Rc::new(child1)));
            node.add_child(RefCell::new(Rc::new(child2)));

            child1 = node;
        }

        Ok(Some(child1))
    }


    // 语法解析：乘法表达式 mul -> pri | mul * pri | mul / pri
    fn multiplicative<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let mut child1 = self.primary(tokens)?;
        if child1.is_none() {
            return Ok(None);
        }

        let mut child1 = child1.unwrap();

        loop {
            let token = tokens.peek();
            if token.is_none() || (token.unwrap().get_type() != TokenType::Star &&
                token.unwrap().get_type() != TokenType::Slash) {
                break;
            }

            // token => / 或者 *
            let token_text = tokens.read().unwrap().get_text();
            let node = SimpleASTNode::new(ASTNodeType::Multiplicative, token_text);
            let e = simple_calculator::invalid_input_err("invalid additive expression, expecting the right part.");
            let child2 = self.primary(tokens)?.ok_or(e)?;

            node.add_child(RefCell::new(Rc::new(child1)));
            node.add_child(RefCell::new(Rc::new(child2)));

            child1 = node;
        }

        Ok(Some(child1))
    }

    /// 语法解析：基础表达式, pri -> Id | Literal | (exp)
    fn primary<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let token = tokens.peek();
        if token.is_none() {
            return Ok(None);
        }

        let token = token.unwrap();
        let txt = token.get_text();
        // println!("primary token {}", txt);
        match token.get_type() {
            TokenType::IntLiteral => { // 整型字面量
                let token = tokens.read().unwrap();
                Ok(Some(SimpleASTNode::new(ASTNodeType::IntLiteral, token.get_text())))
            }
            TokenType::Identifier => { // 变量名
                let token = tokens.read().unwrap();
                Ok(Some(SimpleASTNode::new(ASTNodeType::Identifier, token.get_text())))
            }
            TokenType::LeftParen => { // (
                let _ = tokens.read().unwrap(); // 消耗掉 (

                let node = self.additive(tokens)?;
                if node.is_none() {
                    return Err(simple_calculator::invalid_input_err("expecting an additive expression inside parenthesis"));
                }

                let token = tokens.peek();
                if token.is_none() || token.unwrap().get_type() != TokenType::RightParen {
                    return Err(simple_calculator::invalid_input_err("expecting right parenthesis"));
                }

                // token.type = TokenType::RightParen

                let _ = tokens.read(); // 消耗掉 )
                return Ok(node);
            }
            _ => return Ok(None)
        }
    }
}

