/*
 * CS 538, Spring 2019: HW4
 *
 * Reverse Polish Notation: parser.rs
 * See `rpn.md` for the overview.
 */

extern crate rand;

use std::io::{self, Write};

use super::rpn;

pub fn rpn_repl() -> rpn::Result<()> {
    let mut stack = rpn::Stack::new();
    let mut input = String::new();

    // Read-eval-print loop
    loop {
        // Clear the input buffer
        input.clear();

        // Prompt the user
        print!("> ");
        io::stdout().flush().map_err(rpn::Error::IO)?;

        // Read a line and evaluate it
        io::stdin().read_line(&mut input).map_err(rpn::Error::IO)?;
        evaluate_line(&mut stack, &input)?;

        
        // stack.push(rpn::Item::Int(5));

        // A successful run should end with a stack with a exactly one item: the result
        let res = stack.pop()?;
        if stack.empty() {
            println!("Reply> {:?}", res);
        } else {
            return Err(rpn::Error::Extra);
        }
    }
}

fn evaluate_line(stack: &mut rpn::Stack, buf: &String) -> rpn::Result<()> {
    // Trim whitespace and split; this gives an iterator of tokens.
    let tokens = buf.trim().split_whitespace();

    /*
     * Write the main loop processing the tokens. The `parse` method for Strings will be useful for
     * parsing integers. See here for examples:
     *
     * https://doc.rust-lang.org/std/primitive.str.html#method.parse
     */
    for tok in tokens {
        if tok.is_empty(){
            // this should never happen
            println!("Empty {}",tok);
        }
        if tok.is_ascii(){
            // println!("valid input {}" ,tok);
            match tok.parse::<i32>() {
                Ok(i) => {
                    // println!("your integer input: {}", i);
                    stack.push(rpn::Item::Int(i));
                    // println!("stack.len = {}" , stack.empty());
                    // return Ok(stack);
                },
                Err(..) => {
                    // println!("this was not an integer: {}", tok);
                    // println!("stack.len = {}" , stack.empty());
                },
            };
            match tok.parse::<bool>() {
                Ok(i) => {
                    // println!("your bool input: {}", i);
                    stack.push(rpn::Item::Bool(i));
                    // println!("stack.len = {}" , stack.empty());
                    // return Ok(stack);
                },
                Err(..) => {
                    // println!("this was not a boolean: {}", tok);
                    // println!("stack.len = {}" , stack.empty());
                },
            };
            if tok == "+"{
                // println!("*********ADD OPERATOR TEST: {}", tok);
                stack.eval(rpn::Op::Add);
            } else if tok == "="{
                // println!("*********ADD OPERATOR TEST: {}", tok);
                stack.eval(rpn::Op::Eq);
            } else if tok == "~"{
                // println!("*********ADD OPERATOR TEST: {}", tok);
                stack.eval(rpn::Op::Neg);
            } else if tok == "<->"{
                // println!("*********ADD OPERATOR TEST: {}", tok);
                stack.eval(rpn::Op::Swap);
            } else if tok == "#"{
                // println!("*********ADD OPERATOR TEST: {}", tok);
                stack.eval(rpn::Op::Rand);
            } else if tok == "?"{
                // println!("*********ADD OPERATOR TEST: {}", tok);
                stack.eval(rpn::Op::Cond);
            } else if tok == "quit"{
                println!("QUIT INITIATED: {}", tok);
                stack.eval(rpn::Op::Quit);
                return Err(rpn::Error::Quit);
            } 
            // else {
            //     println!("Syntax");
            //     return Err(rpn::Error::Syntax);
            // }


            // let num = tok.parse::<i32>()?;
            // if let rpn::Item::Int(inside) = num {
            //         println!("inside: {}", inside);
            //         stack.push(rpn::Item::Int(inside));
            // } 
            
            // if let tok.parse::<i32>(i) = tok {
            //         println!("inside: {}", inside);
            //         stack.push(rpn::Item::Int(i));
            // }
            
            // else if let Item::Bool(inside) = second {
            //     println!("inside: {}", inside);
            //     flag += 1;
            //     // return Err(type_error);
            // }

        } else {
            println!("non ascii: {}",tok);
            return Err(rpn::Error::Syntax)
        }
    }
    Ok(())
}
