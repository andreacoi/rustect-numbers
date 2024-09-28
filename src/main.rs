fn main() {
    let round = 10;
    // debug - verify the correctness of my function, step by step.
    // For now, I’m doing this without implementing complex tests.
     for x in 2..round {
        println!("Tentativo {x}:");
        let divisors = total_divisors(x);
        let exact_divisors = exact_divisors(x, divisors);
        let y = sum_divisors(&exact_divisors);
        let z = product_divisors(x, &exact_divisors);
        if compare_results(&x, &y, &z) == true {
            println!("{x} is a Perfect number!");
        } else {
            println!("nope.");
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

// This function sums the numbers in a vector and returns the result.
fn product_divisors(dividend: u128, divisors:&Vec<u128>) -> u128 {

    // To address the overflow issue, I perform a double-check. 
    // If the sum of the divisors differs from the original number, 
    // there's no point in calculating the product, so I pre-check 
    // if the sum equals the original number before proceeding with the product.

    let mut product_divisors: u128 = 0;
    let sum_divisors: u128 = sum_divisors(divisors);
    if dividend == sum_divisors {
        product_divisors = divisors.iter().product();
    };
    product_divisors
}

// This function compares values and returns the comparison result.
fn compare_results(number_to_check:&u128, sum:&u128, product:&u128) -> bool {
    if &number_to_check == &sum && &number_to_check == &product {
        true
    } else {
        false
    }
}

