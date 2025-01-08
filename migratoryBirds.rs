use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut bird_counts = HashMap::new();

    // count the frequency of each bird type
    for &bird in arr {
        *bird_counts.entry(bird).or_insert(0) += 1;
    }

    // find the bird with the maximum frequency, 
    // handling ties by choosing the smallest bird type
    let mut most_frequent_bird = i32::MAX;
    let mut max_count = 0;

    for (&bird, &count) in bird_counts.iter() {
        if count > max_count || (count == max_count && bird < most_frequent_bird) {
            most_frequent_bird = bird;
            max_count = count;
        }
    }

    most_frequent_bird
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
