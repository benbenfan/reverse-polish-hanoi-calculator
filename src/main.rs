#![allow(dead_code)]
#![forbid(unsafe_code)]

mod problem1;
mod problem2;
mod problem3;
mod problem4;

mod parser;
mod rpn;

fn main() {
    if let Err(err) = parser::rpn_repl() {
        println!("Error: {:?}", err);
    }
}
