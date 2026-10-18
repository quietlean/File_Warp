use std::io::{self};
use std::{process};

// Functions are in "utils.rs".
mod utils;

fn main() {
    println!("\x1b[4mWelcome to File Warp!\x1b[0m");
    println!("\n");

    utils::print_options();

    loop {

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("fehler aufgetreten");

        match input.trim() {
            "q" => break,
            "1" => println!("opt 1"),
            "2" => println!("opt 2"),
            "3" => println!("opt 4"),
            "4" => {
                //file_read is called. If error occurs in function it's passed back to Err and printed.
                if let Err(err) = utils::file_read() {
                    println!("Error: {}", err);
                    process::exit(1);
                }
            }
            "5" => {
                utils::loading();
            },
            _ => println!("Choose valid input!"),
        }

    }


}
