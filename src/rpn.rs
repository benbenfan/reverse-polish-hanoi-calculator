/*
 * CS 538, Spring 2019: HW4
 *
 * Reverse Polish Notation: rpn.rs
 * See `rpn.md` for the overview.
 */

use std::io;
extern crate rand;

use rand::Rng;

// Stacks will work with Items, which either either integers or booleans
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Item {
    Int(i32),
    Bool(bool),
}

// List of possible errors
#[derive(Debug)]
pub enum Error {
    Empty,         // Tried to pop empty stack
    Extra,         // Stack ended with extra elements
    Type,          // Type mismatch
    Syntax,        // Syntax error, didn't recognize op
    IO(io::Error), // Some kind of IO error
    Quit,          // User quitting
}

// Base operations supported by calculator, see rpn.md
#[derive(Debug)]
pub enum Op {
    Add,
    Eq,
    Neg,
    Swap,
    Rand,
    Cond,
    Quit,
}

// We'll define a result type for our calculator: either a valid value, or a calculator Error
pub type Result<T> = std::result::Result<T, Error>;

// Define a type for Stacks
#[derive(Debug)]
pub struct Stack{
    len: usize,
    pos: usize,
    ilist: Vec<Item>,
    // list: Vec<Result<Item>>,
    // len: usize,
    // data: Item,
    // options: Op,
    // errors: Error,
    // result: Result<T,Error>,
}


// Implement the following functions on Stacks
impl Stack {
    // Make a new Stack
    pub fn new() -> Stack {
        Stack{
            len: 0,
            pos: 0,
            ilist: Vec::new(),
            // list: Vec::new()
            // list: Vec<<Result<T>>
            // outgoing_list: Vec<Vec<Result<T>>>,
           
            // data: Item,
            // options: Op,
            // errors: Error,
            // result: Result<T,Error>,
        }
    }

    // Check if a Stack is empty
    pub fn empty(&self) -> bool {
        let mut tf = true;
        if self.len > 0 && self.ilist.len() > 0{
            tf = false;
        }
        tf
    }

    // Push an item onto a stack (should never error)
    pub fn push(&mut self, item: Item) -> Result<()> {
        // stack.push(item);
        // self.list
        // let test = Item::Int(2);
        self.ilist.push(item);
        // println!("push myLenght: {}", self.ilist.len());
        self.len += 1;
        Ok(())  
    }

    // // Pop an item off the Stack; may result in Empty error
    pub fn pop(&mut self) -> Result<Item> {
        let e = Error::Empty;
        if self.empty() {
            // Ok(number)  => number,
            return Err(e);
        } else {
            // self.ilist[self.len-1]
            let tmp = Item::Int(0);
            let mut val = tmp;
            let curr_len = self.len;
            while let Some(top) = self.ilist.pop() {
                val = top;
                self.len -= 1;
                if curr_len != self.len{
                    break;
                }
            }
            Ok(val)
        } 
    }

    /*
     * Main evaluation function: apply an operation to a Stack
     *
     * Hint: Rust has a nice bit of syntax when writing functions returning Result:
     *
     * ```
     * let x = foo()?;
     * ```
     *
     * If `foo()` returns `Ok(ret)`, `ret` will be assigned to `x`. Otherwise if `foo()` returns
     * `Err(e)`, the assignment is not performed and `Err(e)` is returned from the enclosing
     * function.
     *
     * This syntax makes it easier to work with multiple operations that may fail, like:
     *
     * ```
     * let x = foo()?;
     * let y = bar(x)?;
     * let z = baz(x, y)?;
     * 
     * ...
     * ```
     *
     * The first one to fail will abort the whole function while returning the error.
     */
    pub fn eval(&mut self, op: Op) -> Result<()> {
        let e = Error::Empty;
        let type_error = Error::Type;
        match op{
            Op::Add => {
                // println!("Add function");
                if self.len < 2{
                    println!("Error::Empty len: {}", self.len);
                    return Err(e);
                }
                // let mut counter = 0;
                // while let Item::from(Some(top)) = self.pop()? {
                //     println!("inside while add loop");

                //     counter += 1;
                //     if counter == 2{
                //         break;
                //     }
                // }

                // let one =  Item::Int(self.pop() From i32);
                let mut first = self.pop()?;
                let mut second = self.pop()?;
                let mut sum = 0;
                let mut flag = 0;
                first = Item::from(first);
                second = Item::from(second);
                match first {
                    Item::Int(value) => {
                        // println!("value: {}", value);
                        sum += value;
                        },
                    Item::Bool(value)  => {
                        // println!("boolean value: {}", value);
                        flag +=1;
                        // return Err(type_error);
                    },
                }
                
                // trying a different way
                if let Item::Int(inside) = second {
                    // println!("inside: {}", inside);
                    sum += inside;
                } else if let Item::Bool(inside) = second {
                    // println!("inside: {}", inside);
                    flag += 1;
                    // return Err(type_error);
                }

                if flag > 0 {
                    // println!("flag = {}",flag);
                    self.push(second);
                    self.push(first);
                    return Err(type_error);
                }
                // println!("sum = {}",sum);
                let to_add = Item::Int(sum);
                self.push(to_add);
                
                // if first == Item::Int {
                //     // let sum = first + second;
                // } else {
                //     return Err(type_error);
                // }
                // first = first From i32;
                // second = second as i32;
            },
            Op::Cond => {
                println!("cond function");
                if self.len < 3{
                    println!("Error::Empty len: {}", self.len);
                    return Err(e);
                }
                let mut first = self.pop()?;
                let mut second = self.pop()?;
                let mut third = self.pop()?;
                let mut uno = 0;
                let mut dos = 0;
                let mut tf = false;
                let mut flag = 0;
                first = Item::from(first);
                second = Item::from(second);
                third =Item::from(third);
                match third {
                    Item::Int(value) => {
                        println!("value: {}", value);
                        flag +=1;
                        },
                    Item::Bool(value)  => {
                        // println!("boolean value: {}", value);
                        tf = value;
                    },
                }
                match first {
                    Item::Int(value) => {
                        // println!("FIRST: {}", value);
                        uno = value;
                        },
                    Item::Bool(value)  => {
                        // println!("FIRST boolean value: {}", value);
                        flag +=1;
                        // return Err(type_error);
                    },
                }
                match second {
                    Item::Int(value) => {
                        // println!("SECOND: {}", value);
                        dos = value;
                        },
                    Item::Bool(value)  => {
                        // println!("SECOND boolean value: {}", value);
                        flag +=1;
                        // return Err(type_error);
                    },
                }
                if flag > 0 {
                    // println!("flag = {}",flag);
                    self.push(third);
                    self.push(second);
                    self.push(first);
                    return Err(type_error);
                }
                if !tf {
                    // println!("tf is {} so we add the uno value: {}",tf,uno);
                    let to_add = Item::Int(uno);
                    self.push(to_add);
                } else {
                    // println!("tf is {} so we add the dos value: {}",tf,dos);
                    let to_add = Item::Int(dos);
                    self.push(to_add);
                }
            },
            Op::Eq => {
                if self.len < 2{
                    println!("Error::Empty len: {}", self.len);
                    return Err(e);
                }
                let mut first = self.pop()?;
                let mut second = self.pop()?;
                let mut tf = false;
                let mut flag = 0;
                let mut uno = 0;
                let mut dos = 0;
                let mut yi = tf;
                let mut er = tf;
                first = Item::from(first);
                second = Item::from(second);
                match first {
                    Item::Int(value) => {
                        // println!("value: {}", value);
                        uno = value;
                        flag = 1;
                        },
                    Item::Bool(value)  => {
                        // println!("boolean value: {}", value);
                        yi = value;
                    },
                }
                match second {
                    Item::Int(value) => {
                        // println!("SECOND: {}", value);
                        dos = value;
                        if   flag != 1 {
                            flag = 2;
                        }
                        },
                    Item::Bool(value)  => {
                        // println!("SECOND boolean value: {}", value);
                        er = value;
                        if flag != 0 {
                            flag =2;
                        }
                        // return Err(type_error);
                    },
                }
                if flag > 1 {
                    self.push(second);
                    self.push(first);
                    return Err(type_error);
                } else if flag == 1{
                    if uno == dos {
                        let to_add = Item::Bool(true);
                        self.push(to_add);
                    } else{
                        let to_add = Item::Bool(false);
                        self.push(to_add);
                    }
                } else if flag == 0 {
                    if yi == er {
                        let to_add = Item::Bool(true);
                        self.push(to_add);
                    } else {
                        let to_add = Item::Bool(false);
                        self.push(to_add);
                    }
                }
            },
            Op::Neg => {
                // println!("negate function");
                if self.len < 1{
                    // println!("Error::Empty len: {}", self.len);
                    return Err(e);
                }
                let mut first = self.pop()?;
                let mut tf = false;
                let mut flag = 0;
                first = Item::from(first);
                match first {
                    Item::Int(value) => {
                        // println!("value: {}", value);
                        flag +=1;
                        },
                    Item::Bool(value)  => {
                        // println!("boolean value: {}", value);
                        tf = value;
                    },
                }
                if flag > 0 {
                    // println!("flag = {}",flag);
                    self.push(first);
                    return Err(type_error);
                }
                if tf {
                    // println!("tf is {} so we flip the value",tf);
                    let to_add = Item::Bool(false);
                    self.push(to_add);
                } else {
                    // println!("tf is {} so we add the dos value",tf);
                    let to_add = Item::Bool(true);
                    self.push(to_add);
                }
            },
            Op::Quit => {
                // println!("q function");
                let q = Error::Quit;
                return Err(q);
            },
            Op::Rand => {
                // println!("rand");
                if self.len < 1{
                    // println!("Error::Empty len: {}", self.len);
                    return Err(e);
                }
                let mut first = self.pop()?;
                let mut uno = 0;
                let mut flag = 0;
                first = Item::from(first);
                match first {
                    Item::Int(value) => {
                        // println!("value: {}", value);
                        uno = value;
                        },
                    Item::Bool(value)  => {
                        // println!("boolean value: {}", value);
                        flag +=1;
                    },
                }
                if flag > 0 {
                    // println!("flag = {}",flag);
                    self.push(first);
                    return Err(type_error);
                }
                
                let mut rng = rand::thread_rng();
                let tmp = rng.gen_range(0, uno);
                // println!("Rand Integer: {}", tmp);
                let to_add = Item::Int(tmp);
                self.push(to_add);
                // self.push(first);
            },
            Op::Swap => {
                if self.len < 2{
                    println!("Error::Empty len: {}", self.len);
                    return Err(e);
                }
                // println!("swap function");
                let mut first = self.pop()?;
                let mut second = self.pop()?;
                self.push(first);
                self.push(second);
            },
        } 
        Ok(())
    }
}