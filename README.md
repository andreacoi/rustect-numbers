# Rustect Numbers

A **command-line program** designed to check for perfect numbers and efficiently manage resources while performing mathematical operations.

## Features

- **Argument Parsing**: The program parses command-line arguments and provides different functionality based on the input. It requires exactly 3 arguments to run.
  - `-c <number>`: Counts all perfect numbers up to the specified number.
  - `-e <number>`: Checks if the input number is a perfect number.
  - `-h`: Displays the help guide, complete with ASCII art for a bit of fun.
- **Efficient Division Check**: The function to find exact divisors was rewritten to avoid iterating through all possible divisors. This was done by adding an efficient `if` condition to minimize unnecessary operations.
- **Optimized for Large Numbers**: While the program is optimized, be cautious when using very large numbers (e.g., exceeding 1 billion), as it may consume a lot of RAM and significantly slow down the process. In some edge cases (such as 8,589,869,056), the operating system might terminate the process and the terminal.

- **Low RAM Usage**: By checking if a divisor is perfect within a single function and delegating the summation to a dedicated function, the amount of allocated RAM remains stable and low, shifting the computational workload to the CPU.

## Usage

```bash
<program_name> -args <number>

```
