use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    // initialize min and max to the first element of the array
    let mut min = arr[0];
    let mut max = arr[0];
    
    // initialize sum as a 64-bit integer 
    // to store the total sum of the array elements
    let mut sum: i64 = 0;

    for &num in arr {
        // compute sum of the array
        sum += num as i64;
        
        // finds the minimum
        if num < min {
            min = num;
        }
        
        //finds the maximum
        if num > max {
            max = num;
        }
    }
    
    // calculates the sum of the 4 smallest elements (min_sum)
    let min_sum = sum - max as i64;
    // calculates the sum of the 4 largest elements (max_sum)
    let max_sum = sum - min as i64;
    
    // print the min_sum and max_sum as space-separated values
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
