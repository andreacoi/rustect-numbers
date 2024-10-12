pub mod math {
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
    
    // This function checks if a single number is perfect or not. The number to be checked out is
    // passed by args in main.
    pub fn single_check(number_to_check:&u128) {
        if *number_to_check > 10000 {
            println!("Warning! You have entered a very large number! Even though the program is optimized, it may take a considerable amount of time to determine whether the number you chose is perfect or not.");
        }
        let t_divisors =  total_divisors(*number_to_check);
        let e_divisors = exact_divisors(*number_to_check, t_divisors);
        let sum_divisors = sum_divisors(&e_divisors);
        if compare_results(
            &number_to_check, &sum_divisors) == true {
            println!("Woohoo! It's a perfect number!");
        } else {
            println!("Nope! Unlucky try! It's a normal number.");
        }

    }

    pub fn counting_check(limit_to_check: &u128) {

    }

}



