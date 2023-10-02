### CS-410P, Fall 2023

# Homework 1: Modular Exponentiation 

The main goal of this assignment is to warm up with rust by implementing 
a command-line calculator `modexp`. 
The formula is for modular exponentiation is: ( x^y ) % m
 `https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method`

To compute `modexp(2,20,17)` invoke your calculator with `cargo run 2 20 17` and your
program should print `16` on standard output for this case.

## Assignment Setup

Before starting the assignment, I downloaded rust for windows.
I navigated to the directory I wanted my project to be in:
Desktop\rustProjects\hw1
I ran `cargo new modexp`, this will also make the crate name modexp. 
The main source code is located in the `src/main.rs`. 

### Compiling The Project

Before running the program, compile using rust compiler: 
`rustc main.rs`. Run ls to see the three executable files. 
from here go to `src` run main like this: `./main` to see how the 
program runs.

### Building The Project

`cargo build`

### Running The Project

`cargo run`

When making a change in the file compile first and then run the program.

## Requirements

- modexp implementation should contain adequate tests implemented using #[test]
unit testing

- Code must be formatted according to the official Rust formatting style 
Run `cargo fmt` within project directory to reformate the code in-place.

- Code must produce no compiler warnings, and `cargo clippy` must also produce
no warnings

- Must have Rustdoc at the start of the program and at the start of each function

- Should accept only non-negative x and y and positive m.

## Writeup

When starting the assignment, I was able to use chapters 1-3 to help me 
get started with how to assign a variable to a users input. This part was
a little difficult for me at first because I knew that the type I wanted
to assign the variable was an unsigned integer. And the `String`
attached to the code really threw me off until I read chapter 2. The 
parse method on strings converts a string to another type. 

Then I went into office hours and asked a question about matching my
input and reading from command line. I ended up changing my code to
instead of assigning the value to a string then parsing it,
I needed to store the user input in an array so that I can run the 
command `cargo run 2 20 17` on the command line and will get the result.

Implementing modexp:
When implementing the function for modexp. I found the pseudocode on the
homework instructions and wikipedia page helpful to guiding my 
implementation. To accept only non-negative x, y and m, I used u64/u128 types. 
I also used the hint on the homework using`u128::from(x)` to convert a 
`u64` value to `u128`, and `u64::try_from(result).unwrap()` to convert a 
`u128` back to a `u64`.

I had a little trouble understanding variable assignment. Because in
C++ we need to reassign the same value to a different variable.
But Rust does a good job handling this so it's something I don't 
really need to worry about.

## Testing

For testing I used the example provided in the homework to test to see
if my implementation worked. And some of my own edge cases. One 
edge case that I could think of was any number that is divisible
to itself should result in 0 using the modexp function. 

## Submission 

Submit a ZIP archive containing source files only: no `target/` 
directory or `.git` directory or garbage files.

The submission should contain a 
```
./README.md
./Cargo.toml
./src/main.rs
```