//pub mod solution;

use solution::{hello, is_leap_year};

fn main() {
    // You can use main to write tests for your solutions
    // nothing written in main will be used for grading:
    // make sure to put it all in solution.rs!
    println!("{}", hello());
    println!("testing is_leap_year(100): {}", is_leap_year(100));
    println!("testing is_leap_year(2020): {}", is_leap_year(2020));
    println!("testing is_leap_year(1557): {}", is_leap_year(1557));
    println!("testing is_leap_year(1900): {}", is_leap_year(1900));
}
