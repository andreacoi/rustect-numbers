fn main() {
    let dividers = total_dividers(10);
    println!("{:?}", dividers);
}


// In this function, I extract ALL POSSIBLE DIVISORS, whether they are decimal or perfect divisors.
// Of course, I start from the number by subtracting one and NATURALLY excluding 0.
fn total_dividers(starting_number:u128) -> Vec<u128> {

    // initialize an empty vector to store the results of the countdown.
    let mut dividers = Vec::new();
    
    // use a reverse for loop to iterate over the divisors.
    // For each of them, use push to insert it into the vector in each round.
    for x in (2..starting_number).rev() {
        dividers.push(x);
    }
    dividers 
}
