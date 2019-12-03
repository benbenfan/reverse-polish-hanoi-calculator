/*
 * CS 538, Spring 2019: HW4
 * Problem 4
 *
 * You will write a small Towers of Hanoi solver. This is a classical puzzle, involving moving a
 * set of differently-sized discs from one pole to another pole, while making sure that smaller
 * discs sit on top larger discs at all time. See here for some animations:
 *
 * https://en.wikipedia.org/wiki/Tower_of_Hanoi
 *
 * We first make a type of pegs: there are three possible pegs, called A, B, C
 */

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/*
 * A move from peg1 to peg2 is represented by a pair (peg1, peg2).
 */
pub type Move = (Peg, Peg);

/*
 * Write a program that takes the number of discs (`num_discs`) and three pegs (`src`, `aux`,
 * `dst`) and outputs a vector of moves to move `num_discs` discs from `src` to `dst`, using the
 * `aux` peg. You don't need to come up with the algorithm. You can implement the one here:
 *
 * https://en.wikipedia.org/wiki/Tower_of_Hanoi#Simpler_statement_of_iterative_solution
 *
 * You may assume that `src`, `aux`, and `dst` are all different.
 * You may want to write some helper functions to avoid repeating lots of code.
 */
fn canSwap (start: &Vec<u32>, end: &Vec<u32>) -> bool{
    let mut s = false;
    if end.is_empty() {
        return true;
    } else if start.is_empty() {
        // println!("There is nothing in the start vector");
        return false;
    }
    if start[start.len()-1] < end[end.len()-1]{
        s = true;
    }
    s
}
fn exchange(start: u32, end :u32, my_disks: &Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    let src = start as usize;
    let dst = end as usize;
    let mut disks = Vec::new();
    for x in my_disks{
        disks.push(x.to_vec());
    }
    // new_disks.push(disks[0]);
    if !canSwap(&disks[src], &disks[dst]) {
        println!("Exchanging values at disks[{}] and disks[{}] cannot be done",start, end)
    } else {
        let length = disks[src].len();
        while let Some(top) = disks[src].pop() {
            // Prints 1,2,3
            disks[dst].push(top);
            // println!("in while {}", length);
            if disks[src].len() != length {
                // println!("breaks");
                break;
            }
        }
        // println!("{}", disks[src][disks.len()-1]);
    }
    
    // println!("testVal = {}", disks[src][disks[src].len()-1]);
    disks
}
// first pass in the peg you come from
fn findNext (src: u32, disks: &Vec<Vec<u32>>) -> usize{
    // returns the peg position that is free (0 for none and errors)
    if src < 1 || src > 3{
        println!("ERROR: Pegs must be from 1 to 3");
        return 0;
    }
    let p = 0;
    let pos = src as usize;
    let mut pos1 = 1 as usize;
    let mut pos2 = 2 as usize;
    let mut pos3 = 3 as usize;
    if src == 2 {
        pos1 = pos;
        pos2 = 1 as usize;
    } else if src == 3 {
        pos1 = pos;
        pos3 = 1 as usize;
    }
    if disks.is_empty() || disks[pos1].is_empty() {
        println!("ERROR: disks or disk[{}] is empty", pos1);
        return 0;
    } 
    else if disks[pos2].is_empty() && disks[pos3].is_empty() {
        println!("multiple pegs are free");
        return 0;
    } 
    else if canSwap(&disks[pos1], &disks[pos2]) && canSwap(&disks[pos1], &disks[pos3]) {
        println!("both pegs are free");
        return 0;
    } else if canSwap(&disks[pos1], &disks[pos2]) {
        return pos2;
    } else if canSwap(&disks[pos1], &disks[pos3]) {
        return pos3;
    }
    
    // if src.is_empty() {
    //     println!("src is empty");
    //     return 0;
    // } else if  aux.is_empty() && dest.is_empty() {
    //     println!("multiple pegs are free");
    //     return 1;
    // } else if canSwap(src, aux) && canSwap(src, dest) {
    //     println!("both pegs are free");
    //     return 1;
    // } if canSwap(src, aux) {
    //     p = 2;
    //     return p;
    // } if canSwap(src, dest) {
    //     p = 3;
    //     return p;
    // }
    p
}

pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut disks = Vec::new();
    let mut empty_vals = vec![num_discs];
    let mut src_disks = Vec::new();
    let mut aux_disks = Vec::new();
    let mut dst_disks = Vec::new();
    let mut counter = num_discs;
    let num_disks = num_discs as usize;
    for x in 0..num_discs{
        src_disks.push(counter);
        counter -= 1;
    }
    disks.push(empty_vals);
    disks.push(src_disks);
    disks.push(aux_disks);
    disks.push(dst_disks);

    
    let nextP = findNext(1, &disks);
    println!("nextP  = {}", nextP);

    let mut my_move;
    let mut moves = Vec::new();
    if num_discs%2 == 0{
        println!("I am even");
        while disks[3].len() != num_disks{
            counter += 1;
            println!("****************Loop Counter @ {} **************", counter);
            // make the legal move between pegs A and B (in either direction),
            if canSwap(&disks[1], &disks[2]){
                println!("Swapping 1 and 2");
                disks = exchange(1, 2, &disks);
                my_move = (src, aux);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
            } else if canSwap(&disks[2], &disks[1]){
                println!("Swapping 2 and 1");
                disks = exchange(2, 1, &disks);
                my_move = (aux, src);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
            }
       

        // make the legal move between pegs A and C (in either direction),
            if canSwap(&disks[1], &disks[3]){
                println!("Swapping 1 and 3");
                disks = exchange(1, 3, &disks);
                    my_move = (src, dst);
                    moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
                // println!("Swapping 1 and 3 {}");
            } else if canSwap(&disks[3], &disks[1]){
                println!("Swapping 3 and 1");
                disks = exchange(3, 1, &disks);
                my_move = (dst, src);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
            }

            // make the legal move between pegs B and C (in either direction),
            if canSwap(&disks[2], &disks[3]){
                println!("Swapping 2 and 3");
                disks = exchange(2, 3, &disks);
                my_move = (aux, dst);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
                // println!("The last value at disk[3] is {}",disks[3][disks[3].len()-1]);
            } else if canSwap(&disks[3], &disks[2]){
                println!("Swapping 3 and 2");
                disks = exchange(3, 2, &disks);
                my_move = (dst, aux);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
            }
            
            if counter == u32::pow(2,num_discs)-1 {
                let term = i64::pow(2,num_discs)-1;
                println!("CRITICAL ERROR, exceces 2^n − 1 = {} moves", term);
                println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
                break;
            }
        }
        counter = 0;
    } else{
        println!("I am odd");
        while disks[3].len() != num_disks{
            counter += 1;
            println!("****************Loop Counter @ {} **************", counter);
            // make the legal move between pegs A and C (in either direction),
            if canSwap(&disks[1], &disks[3]){
                println!("Swapping 1 and 3");
                println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
                disks = exchange(1, 3, &disks);
                    my_move = (src, dst);
                    moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
                // println!("Swapping 1 and 3 {}");
            } else if canSwap(&disks[3], &disks[1]){
                println!("Swapping 3 and 1");
                println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
                disks = exchange(3, 1, &disks);
                my_move = (dst, src);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
            }
            // make the legal move between pegs A and B (in either direction),
            if canSwap(&disks[1], &disks[2]){
                println!("Swapping 1 and 2");
                println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
                disks = exchange(1, 2, &disks);
                my_move = (src, aux);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
                // println!("The last value at disk[3] is {}",disks[3][disks[3].len()-1]);
            } else if canSwap(&disks[2], &disks[1]){
                println!("Swapping 2 and 1");
                println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
                disks = exchange(2, 1, &disks);
                my_move = (aux, src);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
            }
            // make the legal move between pegs B and C (in either direction),
            if canSwap(&disks[2], &disks[3]){
                println!("Swapping 2 and 3");
                println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
                disks = exchange(2, 3, &disks);
                my_move = (aux, dst);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
                // println!("The last value at disk[3] is {}",disks[3][disks[3].len()-1]);
            } else if canSwap(&disks[3], &disks[2]){
                println!("Swapping 3 and 2");
                println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
                disks = exchange(3, 2, &disks);
                my_move = (dst, aux);
                moves.push(my_move);
                if disks[3].len() == num_disks {
                    break;
                }
            }

            // if counter == 5 {
            //     print!("CRITICAL ERROR, Debugging Counter");
            //     println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
            //     break;
            // }
            
            if counter == u32::pow(2, num_discs)-1 {
                let term = i64::pow(2,num_discs)-1;
                println!("CRITICAL ERROR, exceces 2^n − 1 = {} moves", term);
                println!("Stack A = {} Stack B = {} Stack C = {}",disks[1].len(),disks[2].len(),disks[3].len());
                break;
            }
        }
        counter = 0;
    }
    // while let Some(top) = src_disks.pop() {
    //     // Prints 1,2,3
    //     println!("{}", top);
    //     if *top == 1 {
    //         break;
    //     }
    // }
    
    // aux_disks.push(1);
    // let top = aux_disks.last();
    // let myval = *top;
    // println!("last ele of aux = {}" , myval);

    // if canSwap(&src_disks, &aux_disks) {
    // if canSwap(&disks[1], &disks[2]) {
        // let length = src_disks.len();
        //  while let Some(top) = src_disks.pop() {
        //     // Prints 1,2,3
        //     aux_disks.push(top);
        //     println!("in while {}", length);
        //     if src_disks.len() != length {
        //         println!("breaks");
        //         break;
        //     }
        // }
        // println!("{}", src_disks[src_disks.len()-1]);
    // }


    println!("Total Moves : {}", moves.len());


    moves
}

/// You can put more tests here.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi() {
        let moves = vec![ (Peg::A, Peg::C)
                        , (Peg::A, Peg::B)
                        , (Peg::C, Peg::B)
                        , (Peg::A, Peg::C)
                        , (Peg::B, Peg::A)
                        , (Peg::B, Peg::C)
                        , (Peg::A, Peg::C) ];

        assert_eq!(hanoi(3, Peg::A, Peg::B, Peg::C), moves);
    }
}
