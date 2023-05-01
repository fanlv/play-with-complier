use std::cell::{Ref, RefCell, RefMut};
use std::error::Error;
use std::io;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::lexer::{ASTNode, ASTNodeType, simple_lexer, TokenReader, TokenType};
use crate::simple_calculator;

#[cfg(test)]
mod tests {
    use crate::lexer::{simple_lexer, TokenReader};
    use crate::lexer::simple_calculator::SimpleCalculator;
    use crate::lexer::simple_lexer::SimpleLexer;

    #[test]
    pub fn test() {
        let script = "int a = b+3;";
        println!("解析变量甚么语句: {}", script);

        let lexer = SimpleLexer::new();
        let mut token_reader = lexer.tokenize(script);
        // lexer.dump(&mut token_reader);

        let calculator = SimpleCalculator::new();
        let node = calculator.int_declare(&mut token_reader);
        match node {
            Ok(value) => {
                if value.is_none() {
                    println!("value is none")
                } else {
                    value.unwrap().dump_ast("")
                }
            }
            Err(error) => println!("Error: {}", error),
        }


        // let script = "2+3*5";
        // let lexer = SimpleLexer::new();
        // let mut token_reader = lexer.tokenize(script);
        // lexer.dump(&mut token_reader);

        // let script = "2+3*5";
        // println!("计算：{} ", script);
        // calculator.evaluate(script);
        //
        //
        // let script = "2+";
        // println!("计算：{} ", script);
        // calculator.evaluate(script);

        // let script = "2-1-5*3";
        // println!("计算：{} ", script);
        // calculator.evaluate(script);


        // let script = "2+3+4";
        // println!("计算：{} ", script);
        // calculator.evaluate(script);
        //
        let script = "1+2+3+4+5*6+8+9+10";
        println!("计算：{} ", script);
        calculator.evaluate(script);
    }
}


struct SimpleCalculator {}

impl SimpleCalculator {
    fn new() -> SimpleCalculator {
        SimpleCalculator {}
    }

    fn demo() -> Result<i32, io::Error> {
        let e = io::Error::new(io::ErrorKind::InvalidInput, "variable name expected");
        return Err(e);
    }

    // 打印 AST Tree
    // 打印 计算结果
    fn evaluate(&self, code: &str) {
        let tree = self.parse(code);
        println!("dump ASTTree :");
        tree.dump_ast("");
        println!(" ");

        let _ = self.calculate_and_print(&Rc::new(tree), "");
    }

    fn calculate_and_print<T: ASTNode>(&self, node: &Rc<T>, indent: &str) -> i32 {
        let mut result = 0;
        println!("{} Calculating: {}", indent, node.get_type());
        match node.get_type() {
            ASTNodeType::Program => {
                for child in node.get_children().iter() {
                    result = self.calculate_and_print(child, format!("{}\t", indent).as_str());
                }
            }
            ASTNodeType::Additive | ASTNodeType::Multiplicative => {
                let children = node.get_children();
                let child1 = children.get(0).expect("child 1 not found");
                let child2 = children.get(1).expect("child 2 not found");

                let num1 = self.calculate_and_print(child1, format!("{}\t", indent).as_str());
                let num2 = self.calculate_and_print(child2, format!("{}\t", indent).as_str());

                match node.get_text() {
                    "+" => result = num1 + num2,
                    "-" => result = num1 - num2,
                    "*" => result = num1 * num2,
                    "/" => result = num1 / num2,
                    _ => println!("found unsupported operator: {}", node.get_text()),
                }
            }
            ASTNodeType::IntLiteral => {
                result = i32::from_str(node.get_text()).unwrap_or_else(|e| {
                    panic!("parse {} failed {}", node.get_text(), e);
                });
            }
            _ => { println!("found unhandled node: {}", node.get_type()) }
        };

        println!("{}Result: {}", indent, result);
        result
    }

    // 解析脚本，并返回根节点
    fn parse(&self, code: &str) -> SimpleASTNode {
        let lexer = simple_lexer::SimpleLexer::new();
        let mut tokens = lexer.tokenize(code);
        self.get_root(&mut tokens)
    }

    // 语法解析：根节点
    fn get_root<T: TokenReader>(&self, tokens: &mut T) -> SimpleASTNode {
        let node = SimpleASTNode::new(ASTNodeType::Program, "program");
        let child = self.additive(tokens);
        match child {
            Ok(child) => {
                if !child.is_none() {
                    node.add_child(RefCell::new(Rc::new(child.unwrap())));
                }
            }
            Err(err) => { println!("get root child failed: {} ", err) }
        }

        node
    }


    fn int_declare<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let token = tokens.peek();
        if token.is_none() || token.unwrap().get_type() != TokenType::Int {
            return Ok(None);
        }

        //  token.Type = TokenType::Int

        let _ = tokens.read(); // 消耗掉int
        let e = io::Error::new(io::ErrorKind::InvalidInput, "variable name expected");
        let token = tokens.peek().ok_or(e)?;

        if token.get_type() != TokenType::Identifier {
            let e = io::Error::new(io::ErrorKind::InvalidInput, "variable name expected");
            return Err(e);
        }

        // token.Type = TokenType::Identifier

        let token = tokens.read().unwrap(); // 消耗掉 Identifier
        // 创建当前节点，并把变量名记到AST节点的文本值中，这里新建一个变量子节点也是可以的
        let mut node = SimpleASTNode::new(ASTNodeType::IntDeclaration, token.get_text());

        let token = tokens.peek();
        if !token.is_none() && token.unwrap().get_type() == TokenType::Assignment {
            let _ = tokens.read(); // 消耗掉 =

            let e = io::Error::new(io::ErrorKind::InvalidInput,
                                   "invalid variable initialization, expecting an expression");
            let child = self.additive(tokens)?.ok_or(e)?;
            node.add_child(RefCell::new(Rc::new(child)));
        }

        let token = tokens.peek();
        if token.is_none() || token.unwrap().get_type() != TokenType::SemiColon {
            let e = io::Error::new(io::ErrorKind::InvalidInput, "invalid statement, expecting semicolon");
            return Err(e);
        }

        let _ = tokens.read(); // 消耗掉 ;


        Ok(Some(node))
    }


    /*
    加法表达式
    additiveExpression
    :   multiplicativeExpression
    |   additiveExpression Plus multiplicativeExpression
    ;
    */
    fn additive<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let child1 = self.multiplicative(tokens)?;
        let token = tokens.peek();
        if token.is_none() {
            return Ok(child1);
        }


        let token = token.unwrap();
        if token.get_type() != TokenType::Plus && token.get_type() != TokenType::Minus {
            return Ok(child1);
        }

        let e = io::Error::new(io::ErrorKind::InvalidInput, "invalid additive expression, expecting the right part.");
        let node = SimpleASTNode::new(ASTNodeType::Multiplicative, tokens.read().unwrap().get_text());
        let child1 = child1.unwrap();
        let child2 = self.additive(tokens)?.ok_or(e)?;

        node.add_child(RefCell::new(Rc::new(child1)));
        node.add_child(RefCell::new(Rc::new(child2)));
        // let node_rc = Rc::new(node);
        // *child1.parent.borrow_mut() = Rc::downgrade(&node_rc);
        Ok(Some(node))
    }


    /*
        语法解析：乘法表达式
        multiplicativeExpression
            :   IntLiteral
            |   IntLiteral Star multiplicativeExpression
            ;
    */
    fn multiplicative<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let child1 = self.primary(tokens)?;
        let token = tokens.peek();
        if token.is_none() {
            return Ok(child1);
        }

        let token = token.unwrap();
        if token.get_type() != TokenType::Star && token.get_type() != TokenType::Slash {
            return Ok(child1);
        }


        let node = SimpleASTNode::new(ASTNodeType::Multiplicative, tokens.read().unwrap().get_text());
        let e = io::Error::new(io::ErrorKind::InvalidInput, "invalid additive expression, expecting the right part.");
        let child1 = child1.unwrap();
        let child2 = self.multiplicative(tokens)?.ok_or(e)?;

        node.add_child(RefCell::new(Rc::new(child1)));
        node.add_child(RefCell::new(Rc::new(child2)));
        // let node_rc = Rc::new(node);
        // *child1.parent.borrow_mut() = Rc::downgrade(&node_rc);
        Ok(Some(node))
    }

    // 语法解析：基础表达式
    fn primary<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let token = tokens.peek();
        if token.is_none() {
            return Ok(None);
        }

        let token = token.unwrap();

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
                let token = tokens.read().unwrap(); // 消耗掉 (

                let node = self.additive(tokens)?;
                if node.is_none() {
                    return Err(simple_calculator::invalid_input_err("expecting an additive expression inside parenthesis"));
                }

                let token = tokens.peek();
                if token.is_none() {
                    return Err(simple_calculator::invalid_input_err("expecting right parenthesis"));
                }

                let token = token.unwrap();
                if token.get_type() == TokenType::RightParen {
                    let _ = tokens.read(); // 消耗掉 )
                    return Ok(node);
                }

                Err(simple_calculator::invalid_input_err("expecting right parenthesis"))
            }
            _ => {
                // invalid_input_err("unknown token type")
                Err(simple_calculator::invalid_input_err("unknown token type"))
            }
        }
    }
}


pub fn invalid_input_err(err: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, err)
}


pub struct SimpleASTNode {
    // parent: RefCell<Weak<SimpleASTNode>>,
    children: RefCell<Vec<Rc<SimpleASTNode>>>,
    node_type: ASTNodeType,
    text: String,
}

impl SimpleASTNode {
    pub fn new(node_type: ASTNodeType, text: &str) -> Self {
        SimpleASTNode {
            // parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
            node_type,
            text: text.to_string(),
        }
    }

    pub fn add_child(&self, child: RefCell<Rc<SimpleASTNode>>) {
        let mut child = child.borrow_mut();
        // let rc_node = Rc::new(self);
        // *child.parent.borrow_mut() = Rc::downgrade(&rc_node);
        self.children.borrow_mut().push(child.clone());
    }

    // pub fn set_parent(parent: &Rc<SimpleASTNode>, child: &SimpleASTNode) {
    //     *child.parent.borrow_mut() = Rc::downgrade(parent);
    // }

    fn dump_ast(&self, indent: &str) {
        println!("{}{} {}", indent, self.get_text(), self.get_type());
        for child in self.get_children().iter() {
            child.dump_ast(format!("{}\t", indent).as_str());
        }
    }
}

impl ASTNode for SimpleASTNode {
    // fn get_parent(&self) -> Option<Rc<SimpleASTNode>> {
    //     self.parent.borrow().upgrade()
    // }

    fn get_children(&self) -> RefMut<Vec<Rc<SimpleASTNode>>> {
        self.children.borrow_mut()
    }

    fn get_type(&self) -> ASTNodeType {
        self.node_type.clone()
    }

    fn get_text(&self) -> &str {
        &self.text
    }
}

// fn dump_ast<T: ASTNode>(node: &T, indent: &str) {
//     println!("{} {} {}", indent, node.get_text(), node.get_type());
//     for child in node.get_children().iter() {
//         dump_ast(child.as_ref(), indent);
//     }
// }
