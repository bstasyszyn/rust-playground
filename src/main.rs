// use std::error::Error;
// use std::fs::File;
// use std::{io, result};

use std::fmt;
use std::fmt::Formatter;

fn match_it(card: &str) -> i32 {
    match card {
        "Jack" => 10,
        "Ace" => 11,
        _ => 0,
    }
}

// fn if_it(card: Option<i32>) -> i32 {
//     let v: i32 = 7;
//
//     if let Some(card) = None {
//         11
//     } else {
//         12
//     }
// }

#[derive(Debug, Clone)]
pub struct MyError {
    pub message: String,
    pub denominator: f64,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - Denominator {}", self.message, self.denominator)
    }
}

fn option_it() {
    fn divide(numerator: f64, denominator: f64) -> Result<f64, MyError> {
        if denominator == 0.0 {
            Err(MyError {
                message: "XXXXX".to_string(),
                denominator: denominator,
            })
        } else {
            Ok(numerator / denominator)
        }
    }

    // The return value of the function is an option
    // let result = divide(2.0, 3.0);
    let result = divide(2.0, 0.0).unwrap_or_else(|_err| {
        println!("Got error {_err}");

        return 1.1;
    });

    // // Pattern match to retrieve the value
    // match result {
    //     // The division was valid
    //     Some(x) => println!("Result: {x}"),
    //     // The division was invalid
    //     None => println!("Cannot divide by 0"),
    // }

    println!("Result {}", result)
}

fn main() {
    println!("Matched rank is {}", match_it("Ace"));

    // let card: i32 = 7;

    // println!("If'ed rank is {}", if_it(Some(card)));
    option_it();
}
