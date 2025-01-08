use std::env;
use std::fs::File;
use std::cmp::min;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'getTotalX' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

// function to compute the Greatest Common Divisor (GCD) of all elements in B
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

// function to compute the Least Common Multiple (LCM) of all elements in A
fn lcm(a: i32, b: i32) -> i32 {
    return (a * b) / gcd(a, b);
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    // find the least common multiple of all elements in A
    let mut lcm_a = a[0];
    for &num in &a[1..] {
        lcm_a = lcm(lcm_a, num);
    }

    // find the greatest common divisor of all elements in A
    let mut gcd_b = b[0];
    for &num in &b[1..] {
        gcd_b = gcd(gcd_b, num);
    }

    // count how many multiples of lcm_a divide gcd_b
    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
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

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
