#![allow(unused)]

use lexer::simple_calculator;
use lexer::simple_lexer;
use lexer::simple_script;

mod lexer;

fn main() {
    // simple_lexer::test();

    simple_script::script_demo();
}

//
// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl Point {
//     fn move_to(&mut self, x: i32, y: i32) {
//         self.x = x;
//         self.y = y;
//     }
// }


// let mut p = Point { x: 0, y: 0 };
// let r = &mut p;
// let rr: &Point = &*r; // 基于可变引用构造不可变引用
//
// println!("{:?}", rr);
// r.move_to(10, 10);
// println!("{:?}", r);

// let mut p = Point { x: 0, y: 0 };
// let r = &mut p;
// let rr: &Point = &*p; // 直接从p借用不可变引用
//
// println!("{:?}", rr);
// r.move_to(10, 10);
// println!("{:?}", r);