extern crate rust_fizz_buzz;

use rust_fizz_buzz::fizzbuzz::fizzbuzz;

fn main() {
    const MAX_VALUE : i32 = 30;
    for index in 1..=MAX_VALUE {
        println!("{} {}", index, fizzbuzz::fizzbuzz(index));
    }
}
