use std::env;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'formingMagicSquare' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY s as parameter.
 */

fn formingMagicSquare(s: &[Vec<i32>]) -> i32 {
    // predefined 8 possible magic squares
    let magic_squares = vec![
        vec![vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]],
        vec![vec![6, 1, 8], vec![7, 5, 3], vec![2, 9, 4]],
        vec![vec![4, 9, 2], vec![3, 5, 7], vec![8, 1, 6]],
        vec![vec![2, 9, 4], vec![7, 5, 3], vec![6, 1, 8]],
        vec![vec![8, 3, 4], vec![1, 5, 9], vec![6, 7, 2]],
        vec![vec![4, 3, 8], vec![9, 5, 1], vec![2, 7, 6]],
        vec![vec![6, 7, 2], vec![1, 5, 9], vec![8, 3, 4]],
        vec![vec![2, 7, 6], vec![9, 5, 1], vec![4, 3, 8]],
    ];

    let mut min_cost = i32::MAX;

    // Calculate the cost for each magic square
    for magic in magic_squares {
        let mut cost = 0;
        for i in 0..3 {
            for j in 0..3 {
                cost += (s[i][j] - magic[i][j]).abs();
            }
        }
        min_cost = cmp::min(min_cost, cost);
    }

    min_cost
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s: Vec<Vec<i32>> = Vec::with_capacity(3_usize);

    for i in 0..3_usize {
        s.push(Vec::with_capacity(3_usize));

        s[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = formingMagicSquare(&s);

    writeln!(&mut fptr, "{}", result).ok();
}