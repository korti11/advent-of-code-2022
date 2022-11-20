# advent-of-code-2022
This are my solutions for Advent of Code 2022 written in Rust: https://adventofcode.com/2022
## Prerequisite 
- Visual Studio Code
- Dev Containers Extension
- Docker

## Implementation notes
- Each day is implemented in a separate module. The naming schema for these module is `day{number_of_the_day}`. Example: `day1`
- In each of these day modules there is one `solution.rs` file. All with the same base structure. Look at the `sample/solution.rs` file for the base structure.
- The `data` module has two methods. The first one for loading the input data, the naming schema for input files is `day{number_of_the_day}-{level}.txt`. The second method is is to write out the solution, the naming schema for solution files is `day{number_of_the_day}-{level}-solution.txt`.

## Usage
To run the code use `cargo run`