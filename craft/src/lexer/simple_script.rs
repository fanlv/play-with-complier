use std::{env, io};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::rc::Rc;
use std::str::FromStr;

use crate::lexer::{ASTNode, ASTNodeType, simple_lexer, TokenReader, TokenType};
use crate::lexer::simple_calculator;
use crate::lexer::simple_calculator::SimpleASTNode;
use crate::lexer::simple_parser::SimpleParser;

pub fn script_demo() {
    println!("abc");
    // 使用 `env::args()` 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 打印所有参数
    println!("All arguments: {:?}", args);


    let mut v = false;
    // 遍历并打印每个参数
    for (index, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", index, arg);
        if arg == "-v" {
            v = true;
        }
    }

    // v = true;

    println!("Simple script language!");

    let parser = SimpleParser::new();
    let script = SimpleScript::new(v);

    let mut code = String::new();

    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed read line");

        let line = line.trim();

        if line == "exit();" {
            println!("\n good byte");
            break;
        }

        code.push_str(line);
        code.push_str("\n");

        if !line.ends_with(";") {
            continue;
        }

        let mut parse_result = parser.parse(code.as_str());
        if let Err(error) = parse_result {
            println!("parse failed: {}", error);
            code = String::new();
            continue;
        }

        let root = parse_result.unwrap();
        if script.verbose {
            root.dump_ast("")
        }

        let res = script.evaluate(&Rc::new(root), "");
        match res {
            Ok(val) => (),
            Err(err) => println!("script.evaluate failed : {}", err),
        }

        code = String::new();
    }
}


struct SimpleScript {
    // variables: HashMap<String, i32>,
    variables: RefCell<HashMap<String, i32>>,
    verbose: bool,
}

impl SimpleScript {
    fn new(verbose: bool) -> Self {
        SimpleScript {
            variables: RefCell::new(HashMap::new()),
            verbose,
        }
    }

    fn evaluate<T: ASTNode>(&self, node: &Rc<T>, indent: &str) -> Result<i32, io::Error> {
        if self.verbose {
            println!("{}Calculating: {}", indent, node.get_type())
        }

        let mut result = 0;

        match node.get_type() {
            ASTNodeType::Program => {
                for child in node.get_children().iter() {
                    result = self.evaluate(child, format!("{}\t", indent).as_str())?;
                }
            }
            ASTNodeType::Additive | ASTNodeType::Multiplicative => {
                let children = node.get_children();
                let child1 = children.get(0).expect("child 1 not found");
                let child2 = children.get(1).expect("child 2 not found");

                let num1 = self.evaluate(child1, format!("{}\t", indent).as_str())?;
                let num2 = self.evaluate(child2, format!("{}\t", indent).as_str())?;

                let operator = node.get_text();
                match operator {
                    "+" => result = num1 + num2,
                    "-" => result = num1 - num2,
                    "*" => result = num1 * num2,
                    "/" => result = num1 / num2,
                    _ => println!("found unsupported operator: {}", node.get_text()),
                }
            }
            ASTNodeType::IntLiteral => {
                result = i32::from_str(node.get_text()).unwrap_or_else(|e| {
                    println!("parse {} failed {}", node.get_text(), e);
                    0
                });
            }
            ASTNodeType::Identifier => {
                let var_name = node.get_text();
                let variables = self.variables.borrow();
                if variables.contains_key(var_name) {
                    let v = variables.get(var_name).unwrap();

                    result = *v;
                } else {
                    println!(" not found variable {}", var_name);
                }
            }
            ASTNodeType::AssignmentStmt | ASTNodeType::IntDeclaration => {
                let var_name = node.get_text();
                {
                    let mut variables = self.variables.borrow_mut(); // 1. 这里借用了一次。
                    let node_type = node.get_type();
                    if node_type == ASTNodeType::AssignmentStmt && !variables.contains_key(var_name) {
                        let msg = format!("you dont define variable {}", var_name);
                        return Err(simple_calculator::invalid_input_err(msg.as_str()));
                    }
                } // 3. 所以这里，需要用花括号，让上面借用出了这个代码块以后失效，不然下面再次借用会panic。


                let mut child_result = 0;
                let children = node.get_children();
                if children.len() > 0 {
                    let child = children.get(0);
                    let child = child.unwrap();
                    result = self.evaluate(child, format!("{}\t", indent).as_str())?; // 2. 这里面也会借用一次。
                    child_result = result
                }

                let mut variables = self.variables.borrow_mut();
                variables.insert(var_name.to_string(), child_result);
            }
            _ => ()
        }

        if self.verbose {
            println!("{} Result:{}", indent, result);
        } else if indent == "" {
            match node.get_type() {
                ASTNodeType::IntDeclaration | ASTNodeType::AssignmentStmt => println!("{} : {}", node.get_text(), result),
                ASTNodeType::Program => println!("{}", result),
                _ => ()
            }
        }

        Ok(result)
    }
}

