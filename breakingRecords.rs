use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breakingRecords' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    // initialize the highest and lowest scores to the first score in the list.
    let mut highest = scores[0];
    let mut lowest = scores[0];
    // initialize counters for the number of times the record is broken.
    let mut high_count = 0;
    let mut low_count = 0;

    // start from the second element
    for &score in &scores[1..] {  
        // if the current score is higher than the highest record
        if score > highest {
            highest = score;
            high_count += 1;
        // if the current score is lower than the lowest record
        } else if score < lowest {
            lowest = score;
            low_count += 1;
        }
    }

    // return a vector containing the count of high and low record breaks
    vec!(high_count, low_count)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breakingRecords(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
