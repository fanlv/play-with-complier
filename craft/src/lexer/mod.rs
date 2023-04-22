#![allow(unused)]

pub mod simple_lexer;


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

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Plus,
    // +s
    Minus,
    // -
    Star,
    // *
    Slash,
    // /
    GE,
    // >=
    GT,
    // >
    EQ,
    // ==
    LE,
    // <=
    LT,
    // <
    SemiColon,
    // ;
    LeftParen,
    // (
    RightParen,
    // )
    Assignment,
    // =
    If,
    Else,
    Int,
    Identifier,
    //标识符
    IntLiteral,
    //整型字面量
    StringLiteral,   //字符串字面量
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