use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'divisibleSumPairs' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 *  3. INTEGER_ARRAY ar
 */

fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    // array to hold counts of remainders
    let mut remainder_counts = vec![0; k as usize];  
    // variable to store the number of valid pairs
    let mut count = 0;  
    
    // count the remainder frequency for each element
    for &num in ar {
        let remainder = num % k;
        // the complement remainder to form divisible pair
        let complement = (k - remainder) % k;  
        
        // add the number of times we've seen the complement remainder
        count += remainder_counts[complement as usize];
        
        // increment the count of the current remainder
        remainder_counts[remainder as usize] += 1;
    }
    
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = divisibleSumPairs(n, k, &ar);

    writeln!(&mut fptr, "{}", result).ok();
}
