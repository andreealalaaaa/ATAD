use std::io;

// Function to compute the sum of 2 integers
fn solveMeFirst(a: i32, b: i32) -> i32 {
    // Add the two integers and return the result
    a + b
}

fn main() {
    // variable declaration
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();

    // read variables
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut _num_str_2).ok().expect("read error");

    // parse integers
    let mut _num_1 : i32 = _num_str_1.trim().parse().ok().expect("parse error");
    let mut _num_2 : i32 = _num_str_2.trim().parse().ok().expect("parse error");

    // print the sum
    // Hint: Type println!("{}", _num_1 + _num_2); below 
    let result = solveMeFirst(_num_1, _num_2);
    println!("{}", result);
}