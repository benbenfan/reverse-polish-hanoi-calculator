/*
 * CS 538, Spring 2019: HW4
 * Problem 3 : Completed
 *
 * Write a function that takes in an integer `n`, and returns a vector with all of the prime
 * numbers up to `n`. There are many ways to do this; one way is the Sieve of Eratothenes:
 *
 * https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Overview
 *
 * But you can choose any way you like.
 */

/// Return the list of all prime numbers up to and including n.
pub fn sieve(n: u32) -> Vec<u32> {
        let mut vec = Vec::new();
    
    if n < 0 {
        panic!("Input of  {} is invald!! Please enter something greater than 0",n);
    } else if n < 2 {
        println!("Input of {} returns no primes",n);
        return vec;
    }
    for x in 2..(n+1){
        // println!("{}",x);
        let mut flag = 0;
        let root = (x as f64).sqrt() as i64;
        // println!("root = {}", root);
        let limit = root as u32;
        // println!("x = {} and limit = {}", x, limit);
        for i in 2..limit+1 {
            // println!("x = {} divided by i = {} is remainder = {} ", i, i, x % i);
            if x % i == 0 {  
                // println!("x = {} divided by i = {} is remainder = {} ", i, i, x % i);
                // vec.push(x);
                flag += 1;
                // println!("non prime? = {}",x); 
                break;
            }  
        }
        if flag == 0{
                vec.push(x);
                // println!("non prime? = {}",x); 
        }
    }
    if vec.is_empty(){
        println!("Input of {} returns no primes",n);
    }

    // checker
    // for x in &vec {
    //     println!("{} is not prime", x);
    // }

    vec
}

/// You can put more tests here.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        let primes_twelve = vec![2, 3, 5, 7, 11];
        
        assert_eq!(sieve(12), primes_twelve);
    }
}
