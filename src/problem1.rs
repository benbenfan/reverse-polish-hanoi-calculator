/*
 * CS 538, Spring 2019: HW4
 * Problem 1 : COMPLETE
 *
 * Write implementations of the following simple functions. You can use basic Vector operations
 * like `push`, `pop`, and `contains`, but do not use the built-in `dedup` or `filter` functions,
 * for instance.
 *
 * Take a look at the Vector documentation for a good intro to Rust Vectors:
 *
 * https://doc.rust-lang.org/std/vec/struct.Vec.html
 */

/// Compute the sum of a vector of integers
pub fn sum(vs: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for x in vs {
        sum += x;
        // println!("{}", sum);  
    }
    sum
}

/// Return a copy of the input vector, keeping only the first copy of each element.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
     // let bs = vec![1, 2, 3, 4, 5];;

    
    // let mut iter = vs[0];
    let mut vec = Vec::new();
    // println!("a  = {}", a);
 
    vec.push(vs[0]);
    let a = vs.len();
    let mut b = vec.len();
    // slow method comparison method
    let mut i = 0;
    let mut j = 0;
    let mut flag = 0;
    while i < a {
        while j < b {
            if vs[i] == vec[j]{
                flag += 1;
                println!("{} : vs[{}] != {} : vec[{}]",i,vs[i],j,vec[j]);
            } else {
                println!("{} : vs[{}] != {} : vec[{}]",i,vs[i],j,vec[j]);
            }
            j += 1;
        }
        if flag == 0 {
            vec.push(vs[i]);
            b += 1;
        }
        flag = 0;
        j = 0;
        i += 1;
    }
    // while i < b{
    //     if vec[i] != vs[j]{
    //         print!("vec[{}] = {} :: vs[{}] = {}",i,j,vec[i],vs[j]);
    //         let m = j;
    //         while j < a {
    //             if vec[i] == vs[j]{
    //                 flag += 1;
    //             }
    //         }
    //         if flag == 0 {
    //             vec.push(vs[m]);
    //         }
    //     }
    //     j = i;
    //     i += 1;
    //     flag = 0;
    // }
    
    // while i < a {
    //     while j < a {
    //         if vec[i] != vs[j] {
    //             // print!("pushing value = {} ",vs[j]);
    //             vec.push(vs[j]);
    //         }
    //         j += 1;
    //     }
    //     i += 1;
    // }
    // for x in &vec {
    //     println!("{}", x);
    // }
    // println!("len {}", vec.len());
    vec
}

/// Return a copy of the input vector keeping only elements where the predicate is true. The order
/// of elements should not be changed.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut eval = false;
    let mut i = 0;
    let mut vec = Vec::new();
    let myl = vs.len();
    while i < myl {
        eval = pred(vs[i]);
        // println!("Eval = {} ", eval);
        if eval == true {
            vec.push(vs[i]);
        }
        i += 1;
    }
    // for x in &vec {
    //     println!("{}", x);
    // }
    vec
}

/// You can put more tests here.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let vs = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&vs), 15);
    }

    #[test]
    fn test_sum_1() {
        let vs = vec![1, 2, 3, 4, 5, 5, 5, 5];
        assert_eq!(sum(&vs), 30);
    }

    #[test]
    fn test_dedup() {
        let vs = vec![5, 4, 3, 2, 1, 2, 3, 4, 5];
        assert_eq!(dedup(&vs), [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_dedup_1() {
        let vs = vec![5, 5, 5, 5, 5, 5, 5, 1, 5];
        assert_eq!(dedup(&vs), [5 , 1]);
    }

    #[test]
    fn test_dedup_2() {
        let vs = vec![1, 2, 3, 1];
        assert_eq!(dedup(&vs), [1, 2, 3]);
    }

    #[test]
    fn test_dedup_3() {
        let vs = vec![1, 2, 3, 1, 2, 3, 3, 2, 1];
        assert_eq!(dedup(&vs), [1, 2, 3]);
    }

    #[test]
    fn test_filter() {
        let vs = vec![5, 4, 3, 2, 1, 2, 3, 4, 5];
        assert_eq!(filter(&vs, &|i:i32| { i % 2 == 1 }), [5, 3, 1, 3, 5]);
    }
}
