use crate::parser::Parser;

mod expr;
mod parser;
mod tests;
mod op;

fn main() {
    let data = "2(3 - 1)";
    let mut parser = Parser::new(data);
    match parser.parse() {
        Ok(tree) => {
            match tree.eval() {
                Ok(val) => println!("Implicit multiplication (2(3 - 1)): {} => {}", tree, val),
                Err(e) => println!("Runtime Error: {}", e),
            }
        },
        Err(e) => println!("Parse Error: {}", e),
    }

    let data2 = "10 / (5 - 5)";
    let mut parser2 = Parser::new(data2);
    match parser2.parse() {
        Ok(tree) => {
            match tree.eval() {
                Ok(val) => println!("Result: {}", val),
                Err(e) => println!("Runtime Error for '{}': {}", tree, e),
            }
        },
        Err(e) => println!("Parse Error: {}", e),
    }
}
