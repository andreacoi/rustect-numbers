// include the std library for command-line argument parsing.
use std::env;

// Include the `math` module (absolutely) to access the functions that handle the mathematical operations.
use rustect_numbers::math;

fn main() {
    // fetch the args using env library, store them in a vector made up of string.
    let args:Vec<String> = env::args().collect();
     // Ensure that exactly 3 arguments are passed; otherwise, do not run the program and trigger the help guide.
    if args.len() != 3 {
        println!("The provided arguments are insufficient.");
        help();
    } else {
        // Using `match` to parse the literal argument at position 1 of the argument vector.
        // Position 0 is always reserved for the application name.
        // Possible cases:
        // - case `-c`: counts perfect numbers UP TO the given number;
        // - case `-e`: checks if the provided number is perfect.
        // Note: `-c` stands for counting and `-e` stands for exact.
        match &args[1].as_str() {
            &"-e" => println!("you provided -e arg."),
            &"-c" => println!("you provided -c arg."),
            _ => help(),
        }
    }
}

// This function simply shows an useful manual.
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
-e <number> - checks if <number> is perfect or not\n");
}
