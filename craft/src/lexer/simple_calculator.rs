use std::cell::{Ref, RefCell, RefMut};
use std::error::Error;
use std::io;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::lexer::{ASTNode, ASTNodeType, TokenReader, TokenType};

#[cfg(test)]
mod tests {
    use crate::lexer::{simple_lexer, TokenReader};

    #[test]
    pub fn test() {
        println!("Testing lexer");

        // let token: Option<simple_lexer::SimpleTokenReader> = None;
        //
        // let t = token.unwrap().peek();
        println!("end")
    }
}


struct SimpleCalculator {}


pub struct SimpleASTNode {
    parent: RefCell<Weak<SimpleASTNode>>,
    children: RefCell<Vec<Rc<SimpleASTNode>>>,
    node_type: ASTNodeType,
    text: String,
}

impl SimpleASTNode {
    pub fn new(node_type: ASTNodeType, text: &str) -> Self {
        SimpleASTNode {
            parent: RefCell::new(Weak::new()),
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

    pub fn set_parent(parent: &Rc<SimpleASTNode>, child: &SimpleASTNode) {
        *child.parent.borrow_mut() = Rc::downgrade(parent);
    }

    fn int_declare<T: TokenReader>(&self, tokens: &mut T) -> Result<SimpleASTNode, io::Error> {
        let token = tokens.peek();
        let e = io::Error::new(io::ErrorKind::InvalidInput, "invalid input");

        if token.is_none() || token.unwrap().get_type() != TokenType::Int {
            return Err(e);
        }

        let token = tokens.read().unwrap(); // 消耗掉int

        let token = tokens.peek();
        if token.is_none() || token.unwrap().get_type() != TokenType::Identifier {
            return Err(e);
        }

        let token = tokens.read().unwrap(); // 消耗掉 Identifier
        // 创建当前节点，并把变量名记到AST节点的文本值中，这里新建一个变量子节点也是可以的
        let mut node = SimpleASTNode::new(ASTNodeType::IntDeclaration, token.get_text());

        let token = tokens.peek();
        if token.is_none() || token.unwrap().get_type() != TokenType::Assignment {
            return Err(e);
        }

        let token = tokens.read().unwrap(); // 消耗掉等号

        let add_node = self.additive(tokens)?;

        //匹配一个表达式


        Ok(node)
    }

    // 加法表达式
    fn additive<T: TokenReader>(&self, tokens: &mut T) -> Result<SimpleASTNode, io::Error> {
        let e = io::Error::new(io::ErrorKind::InvalidInput, "invalid input");

        let token = tokens.peek();
        if token.is_none() || token.unwrap().get_type() != TokenType::Minus {
            return Err(e);
        }
        Err(e)
    }


    /*
        语法解析：乘法表达式
        multiplicativeExpression
            :   IntLiteral
            |   IntLiteral Star multiplicativeExpression
            ;
    */
    fn multiplicative<T: TokenReader>(&self, tokens: &mut T) -> Result<SimpleASTNode, io::Error> {
        let child1 = self.primary(tokens)?;
        let e = io::Error::new(io::ErrorKind::InvalidInput, "invalid input");

        let token = tokens.peek();
        if token.is_none() {
            return Err(e);
        }

        let token = token.unwrap();
        if token.get_type() != TokenType::Star && token.get_type() != TokenType::Slash {
            return Err(e);
        }

        // 返回Token流中下一个Token，并从流中取出。 如果流已经为空，返回null;
        // fn read(&mut self) -> Option<&Box<dyn Token>>;
        let token = tokens.read();
        if token.is_none() {
            return Err(e);
        }

        let token = token.unwrap();

        let child2 = self.multiplicative(tokens)?;
        let node = SimpleASTNode::new(ASTNodeType::Multiplicative, token.get_text());
        node.add_child(RefCell::new(Rc::new(child1)));
        node.add_child(RefCell::new(Rc::new(child2)));

        // set_parent

        match token.get_type() {
            _ => Err(e)
        }
    }

    // 语法解析：基础表达式
    fn primary<T: TokenReader>(&self, tokens: &mut T) -> Result<SimpleASTNode, io::Error> {
        let e = io::Error::new(io::ErrorKind::InvalidInput, "primary invalid tokens");

        let token = tokens.peek();
        if token.is_none() {
            return Err(e);
        }

        let token = token.unwrap();
        match token.get_type() {
            TokenType::IntLiteral => { // 整型字面量
                let token = tokens.read().unwrap();
                Ok(SimpleASTNode::new(ASTNodeType::IntLiteral, token.get_text()))
            }
            TokenType::Identifier => { // 变量名
                let token = tokens.read().unwrap();
                Ok(SimpleASTNode::new(ASTNodeType::Identifier, token.get_text()))
            }
            TokenType::LeftParen => { // (
                let token = tokens.read().unwrap(); // 消耗掉 (
                let node = self.additive(tokens)?;

                let token = tokens.peek();
                if token.is_none() {
                    return Err(e);
                }

                let token = token.unwrap();
                if token.get_type() == TokenType::RightParen {
                    let token = tokens.read();
                    return Ok(node);
                }

                Err(e)
            }
            _ => Err(e)
        }
    }
}

impl ASTNode for SimpleASTNode {
    fn get_parent(&self) -> Option<Rc<SimpleASTNode>> {
        self.parent.borrow().upgrade()
    }

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

fn dump_ast<T: ASTNode>(node: &T, indent: &str) {
    println!("{} {} {}", indent, node.get_text(), node.get_type());
    for child in node.get_children().iter() {
        dump_ast(child.as_ref(), indent);
    }
}
