use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    // extract AM or PM part
    let ampm = &s[8..]; 
    // extract hours
    let mut hours: u32 = s[0..2].parse().unwrap(); 

    if ampm == "AM" {
        if hours == 12 {
            // 12 AM is 00:00
            hours = 0; 
        }
    } else {
        if hours != 12 {
            // add 12 hours for PM (except for 12 PM)
            hours += 12; 
        }
    }

    // format and return the new string with 24-hour time
    format!("{:02}:{:02}:{:02}", hours, &s[3..5], &s[6..8])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
