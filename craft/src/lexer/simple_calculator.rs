use std::cell::{Ref, RefCell, RefMut};
use std::error::Error;
use std::io;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::lexer::{ASTNode, ASTNodeType, TokenReader, TokenType};

#[cfg(test)]
mod tests {
    use crate::lexer::{simple_lexer, TokenReader};
    use crate::lexer::simple_lexer::SimpleLexer;

    #[test]
    pub fn test() {
        let script = "int a = b+3";
        println!("解析变量甚么语句: {}", script);

        let lexer = SimpleLexer::new();
        let mut token_reader = lexer.tokenize(script);
        lexer.dump(&mut token_reader);


    }
}


struct SimpleCalculator {}


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

    // 加法表达式
    fn additive<T: TokenReader>(&self, tokens: &mut T) -> Result<Option<SimpleASTNode>, io::Error> {
        let child1 = self.multiplicative(tokens)?;
        let token = tokens.peek();
        if token.is_none() {
            return Ok(child1);
        }

        let e = io::Error::new(io::ErrorKind::InvalidInput, "multiplicative invalid tokens");

        let token = token.unwrap();
        if token.get_type() != TokenType::Plus && token.get_type() != TokenType::Minus {
            return Ok(child1);
        }


        // 需要用花括号 , tokens.read 会返回借用，如果返回的 token 不结束，就不能再借用 tokens
        // 然后会报错 cannot borrow `*tokens` as mutable more than once at a time [E0499]
        // 还可以把 let child2 = self.additive(tokens)?.ok_or(e)?; 移到这句代码上面，就不用花括号包裹了
        let token_text;
        {
            let token = tokens.read().unwrap();
            token_text = token.get_text().clone();
        }

        let node = SimpleASTNode::new(ASTNodeType::Multiplicative, token_text);
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

        let e = io::Error::new(io::ErrorKind::InvalidInput, "multiplicative invalid tokens");
        let token = token.unwrap();
        if token.get_type() != TokenType::Star && token.get_type() != TokenType::Slash {
            return Ok(child1);
        }

        let child2 = self.multiplicative(tokens)?.ok_or(e)?;
        let token = tokens.read().unwrap();
        let node = SimpleASTNode::new(ASTNodeType::Multiplicative, token.get_text());
        let child1 = child1.unwrap();

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

        let e = io::Error::new(io::ErrorKind::InvalidInput, "primary invalid tokens");
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
                    return Err(e);
                }

                let token = tokens.peek();
                if token.is_none() {
                    return Err(e);
                }

                let token = token.unwrap();
                if token.get_type() == TokenType::RightParen {
                    let _ = tokens.read(); // 消耗掉 )
                    return Ok(node);
                }

                Err(e)
            }
            _ => Err(e)
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

fn dump_ast<T: ASTNode>(node: &T, indent: &str) {
    println!("{} {} {}", indent, node.get_text(), node.get_type());
    for child in node.get_children().iter() {
        dump_ast(child.as_ref(), indent);
    }
}
