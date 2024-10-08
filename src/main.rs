// include the std library for command-line argument parsing.
use std::env;


fn main() {
    // fetch the args using env library, store them in a vector made up of string.
    let args:Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("The provided arguments are insufficient.");
        help();
    } else {
        match args.len() {
            // Prepare the main function to check the arguments, ensuring they are always equal to 3.
            3 => { 
                // todo: Check the arguments provided by the user and act accordingly.
/*                 match args[1] {
                    "c" => "cia",
                    "e" => "ecco",
                    _ => "prova",
                    
                }; */
            },
            _ => {
                help();
            }
        }
    }
}


// In this function, I extract ALL POSSIBLE DIVISORS, whether they are decimal or perfect divisors.
// Of course, I start from the number by subtracting one and NATURALLY excluding 0.
fn total_divisors(starting_number:u128) -> Vec<u128> {

    // initialize an empty vector to store the results of the countdown.
    let mut divisors = Vec::new();
    
    // use a reverse for loop to iterate over the divisors.
    // For each of them, use push to insert it into the vector in each round.
    for x in (1..starting_number).rev() {
        divisors.push(x);
    }
    divisors 
}

// This function accepts two arguments: the dividend and a list of all divisors.
// As output, it should return the list of exact divisors.
// An exact divisor is calculated within this function using Rust's built-in modulo function.
// So, IF dividend % divisor == 0, then the divisor is exact.
// If the divisor is exact, I insert it into a list of exact divisors (vector).
fn exact_divisors(dividend:u128, divisors:Vec<u128>) -> Vec<u128> {

    // initialize an empty vector to store the results of the calculated exact divisors.
    let mut exact_divisors = Vec::new();
    // use a for cycle to iterate over divisors and test if a divisor is exact.
    for x in divisors {
        if dividend % x == 0 {
            // if divisor is exact insert into vector "exact_divisors"
            exact_divisors.push(x);
        }
    }
    exact_divisors
}

// This function sums the numbers in a vector and returns the result.
fn sum_divisors(divisors:&Vec<u128>) -> u128 {
    let sum_divisors: u128 = divisors.iter().sum();
    sum_divisors
}

// This function compares values and returns the comparison result.
fn compare_results(number_to_check:&u128, sum:&u128) -> bool {
    if &number_to_check == &sum {
        true
    } else {
        false
    }
}
fn help() {
    println!("

██████╗ ██╗   ██╗███████╗████████╗███████╗ ██████╗████████╗    ███╗   ██╗██╗   ██╗███╗   ███╗██████╗ ███████╗██████╗ ███████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝    ████╗  ██║██║   ██║████╗ ████║██╔══██╗██╔════╝██╔══██╗██╔════╝
██████╔╝██║   ██║███████╗   ██║   █████╗  ██║        ██║       ██╔██╗ ██║██║   ██║██╔████╔██║██████╔╝█████╗  ██████╔╝███████╗
██╔══██╗██║   ██║╚════██║   ██║   ██╔══╝  ██║        ██║       ██║╚██╗██║██║   ██║██║╚██╔╝██║██╔══██╗██╔══╝  ██╔══██╗╚════██║
██║  ██║╚██████╔╝███████║   ██║   ███████╗╚██████╗   ██║       ██║ ╚████║╚██████╔╝██║ ╚═╝ ██║██████╔╝███████╗██║  ██║███████║
╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚══════╝ ╚═════╝   ╚═╝       ╚═╝  ╚═══╝ ╚═════╝ ╚═╝     ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝╚══════╝
\n\n                                                                                                                               
Usage: <program_name> -args\n\n
Arguments:\n
-c <number> - finds all perfect numbers by iterating up to that <number>\n
-e <number> - checks if <number> is perfect or not\n
-h - displays this guide\n");
}
