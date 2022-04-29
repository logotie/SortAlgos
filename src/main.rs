mod Mutils;
mod SAlgo;

use std::io;

fn main() {
    println!("Please enter an amount of integers to generate (amount must be greater than 0):");

    //Read user input, trim and then parse to a number
    let user_input = read_input();
    let trimmed_user_input = trim_input(user_input);
    let amountOfNums = parse_to_int32(trimmed_user_input);

    //Generate amount of random numbers
    let nums = Mutils::gen_random_numbers(amountOfNums);

    //sort using bubbleswap
    let sortedNum = SAlgo::bubble_swap(nums);

}

//Reads and takes in user input
//source: https://users.rust-lang.org/t/how-to-read-an-integer-from-stdin/57538
fn read_input() -> String {
    //Variable declared to store the input
    let mut input_line = String::new();
    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line) // actually read the line
        .expect("Failed to read line"); // which can fail, however
    input_line
}

//removes trailing whitespace
fn trim_input(input: String) -> String {
    let t = input.trim();
    //We convert to string so the object that is returned is mutable
    t.to_string()
}

//attempts to convert to integer
//if unable to parse, panics with "Input not an integer"
fn parse_to_int32(user_input: String) -> i32 {
    let number: i32 = user_input.parse().expect("Input not an integer");
    number
}
