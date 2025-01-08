use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    // Loop over each row of the staircase
    for i in 1..=n {
        // First, print the spaces: n - i spaces
        for _ in 0..(n - i) {
            print!(" ");
        }
        // Then, print the hashes: i hashes
        for _ in 0..i {
            print!("#");
        }
        // After printing the row, move to the next line
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
