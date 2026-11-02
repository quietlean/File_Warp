//use std::io::{self};
//use std::{process};

use crate::utils::{art, edit_projects, file_check, menu_print, menu_print_intro, view_projects};

// Functions are in "utils.rs".
mod utils;

fn main() {

    art();

    file_check();

    menu_print_intro();

    //utils::open_log();

    loop {

        //let mut input = String::new();

        //io::stdin().read_line(&mut input).expect("fehler aufgetreten");
        let input = utils::print_input("");

        match input.trim() {
            "q" => break,
            "1" => {
                utils::file_check();
            },
            "2" => {
                utils::copy();
            },
            "3" => {
                edit_projects();
            },
            "4" => {
                //file_read is called. If error occurs in function it's passed back to Err and printed.
                //if let Err(err) = utils::file_read() {
                //    println!("Error: {}", err);
                //    process::exit(1);
                //}

                view_projects("y");
            },
            _ => println!("Choose valid input!"),
        }

        menu_print();
    }


}
