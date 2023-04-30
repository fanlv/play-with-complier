#![allow(unused)]

use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::rc::{Rc, Weak};

use simple_calculator::SimpleASTNode;

pub mod simple_lexer;
pub mod simple_calculator;


pub trait Token {
    // Token的类型
    fn get_type(&self) -> TokenType;
    // Token的文本值
    fn get_text(&self) -> &str;
}

pub trait TokenReader {
    // 返回Token流中下一个Token，并从流中取出。 如果流已经为空，返回null;
    fn read(&mut self) -> Option<&Box<dyn Token>>;
    // 返回Token流中下一个Token，但不从流中取出。 如果流已经为空，返回null;
    fn peek(&self) -> Option<&Box<dyn Token>>;
    // Token流回退一步。恢复原来的Token。
    fn unread(&mut self);
    // 获取Token流当前的读取位置。
    fn get_position(&self) -> usize;
    // 设置Token流当前的读取位置
    fn set_position(&mut self, position: usize);
}

// impl PartialEq for TokenType {
//     fn eq(&self, other: &Self) -> bool {
//         std::mem::discriminant(self) == std::mem::discriminant(other)
//     }
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Plus,
    Minus,
    Star,
    // /
    Slash,
    // >=
    GE,
    // >=
    GT,
    // ==
    EQ,
    // <=
    LE,
    // <
    LT,
    // ;
    SemiColon,
    // (
    LeftParen,
    // )
    RightParen,
    // =
    Assignment,
    If,
    Else,
    Int,
    //标识符
    Identifier,
    //整型字面量
    IntLiteral,
    //字符串字面量
    StringLiteral,
}


#[derive(Debug, PartialEq)]
enum DfaState {
    Initial,
    If,
    IdIf1,
    IdIf2,
    Else,
    IdElse1,
    IdElse2,
    IdElse3,
    IdElse4,
    Int,
    IdInt1,
    IdInt2,
    IdInt3,
    Id,
    GT,
    GE,
    Assignment,
    Plus,
    Minus,
    Star,
    Slash,
    SemiColon,
    LeftParen,
    RightParen,
    IntLiteral,
}

// pub trait ASTNode<T: ASTNode<T> + Sized> {
//     // 父节点
//     fn get_parent(&self) -> RefCell<Weak<T>>;
//     // 子节点
//     fn get_children(&self) -> RefMut<Vec<Rc<T>>>;
//     // AST 类型
//     fn get_type(&self) -> ASTNodeType;
//     // 文本值
//     fn get_text(&self) -> &str;
// }


pub trait ASTNode {
    // // 父节点
    // fn get_parent(&self) -> Option<Rc<Self>>;
    // 子节点
    fn get_children(&self) -> RefMut<Vec<Rc<Self>>>;
    // AST 类型
    fn get_type(&self) -> ASTNodeType;
    // 文本值
    fn get_text(&self) -> &str;
}

#[derive(Clone)]
pub enum ASTNodeType {
    /// 程序入口，根节点
    Program,
    /// 整型变量声明
    IntDeclaration,
    /// 表达式语句，即表达式后面跟个分号
    ExpressionStmt,
    /// 赋值语句
    AssignmentStmt,
    /// 基础表达式
    Primary,
    /// 乘法表达式
    Multiplicative,
    /// 加法表达式
    Additive,
    /// 标识符
    Identifier,
    /// 整型字面量
    IntLiteral,
}

impl fmt::Display for ASTNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTNodeType::Program => write!(f, "Program"),
            ASTNodeType::IntDeclaration => write!(f, "IntDeclaration"),
            ASTNodeType::ExpressionStmt => write!(f, "ExpressionStmt"),
            ASTNodeType::AssignmentStmt => write!(f, "AssignmentStmt"),
            ASTNodeType::Primary => write!(f, "Primary"),
            ASTNodeType::Multiplicative => write!(f, "Multiplicative"),
            ASTNodeType::Additive => write!(f, "Additive"),
            ASTNodeType::Identifier => write!(f, "Identifier"),
            ASTNodeType::IntLiteral => write!(f, "IntLiteral"),
            _ => write!(f, "unknown AST node type"),
        }
    }
}
