use std::io::{self};
use std::io::Write;
//use std::ops::ControlFlow::Continue;
use std::thread::sleep;
use std::{error::Error, fs::File};
use std::time::Duration;
//use csv_to_table::iter::CsvRecords;
use indicatif::ProgressBar;
use std::process::Command;
use std::path::Path;
use serde::Deserialize;
use toml_edit::{DocumentMut}; //, Table, value
use mac_notification_sys::*; // Sending System Notifications



pub fn file_read() -> Result<(), Box<dyn Error>> {
    //Open the file
    let file = File::open("src/project-log-komma.csv")?;

    let table = csv_to_table::from_reader(file)?.to_string();

    println!("{}", table);

    Ok(())
}

pub fn print_input(message: &str) -> String{

    //If the message is empty it wont be printed
    if message.len() > 0 {
        println!("{}", message);
    }

    print!(">  ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        println!("Error while reading input: {}", error);
    }

    // trim creates a &str so you need to turn it into a String again
    input.trim().to_string()
}

pub fn print_options(){
    //User options are printed.
    // Escape Codes: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
    // ASCII Symboles: https://www.ee.ucl.ac.uk/mflanaga/java/HTMLandASCIItableC1.html
    println!("\n");
    println!("\x1b[48;5;225;30m    Welcome to File Warp!   \x1b[0m");  // println!("\x1b[47;30m⊛   Welcome to File Warp!  ⊛\x1b[0m");
    println!("│You have \x1b[1;95m0\x1b[0m projects.  \x1b[38;5;239mv1.0\x1b[0m│");
    println!("├──────────────────────────┤");
    println!("│\x1b[1;92m1:\x1b[0m safe a project         │");
    println!("│\x1b[1;92m2:\x1b[0m safe all projects      │");
    println!("│\x1b[1;92m3:\x1b[0m edit projects          │");
    println!("│\x1b[1;92m4:\x1b[0m view projects          │");
    //println!("│\x1b[1;92m5:\x1b[0m view activity log    │");
    println!("│\x1b[1;92mq:\x1b[0m quit application       │");
    println!("└──────────────────────────┘");

}

pub fn loading(){

    // spinner from the "indicatif" crate
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));

    sleep(Duration::from_secs(3));

    spinner.finish();
    println!("done xo");
}

pub fn copy(){

    let path_og = "/Users/lengather/Pictures/Bones/";
    let path_dest = "/Users/lengather/Pictures/furz";

    let out = match Command::new("rsync")
            .args(["-a", "--delete", path_og, path_dest])
            .output()
        {
            Ok(out) => out,
            Err(e) => {
                eprintln!("failed to run rsync: {e}");
                return;
            }
        };
        println!("{}", out.status);
}



// Checks whether or not the project log file exists.
pub fn file_check(){

    let path = "project_log.csv";
    let path_check = Path::new(path);

    // tell macOS which app "owns" the notification first
        let bundle = get_bundle_identifier_or_default("terminal"); // or "finder", your own bundle id, ...
        if let Err(error) = set_application(&bundle){ println!("error: {}", error);}

    //if let Err(error) = send_notification("File Bee", None, "Test Message", None) {
    //    println!("Notification couldnt be send: {}", error);
    //}

    let mut note = Notification::new();
    let note = note
        .content_image("/Users/lengather/Pictures/Bones/hello-fuckin-kitty-big-1.png")
        .title("File Bee yo")
        .message("Test Message EYOOOOOUU");

    if let Err(error) = note.send() {
        println!("Notification couldnt be send: {}", error);
    }

    println!("test");
    if path_check.exists(){
        //If the log file is found the amount of projects needs to be checked.
        println!{"File exists."};
    } else {
        // If the log file isn't found a new one is created.
        println!("File not found.");
    }
}




#[derive(Debug, Deserialize)]
struct Config {
    general: General,
}

#[derive(Debug, Deserialize)]
struct General {
    amount: String,
    version: String,
    developer: String,
}

fn load_config(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("can't read file: {e}"))?;
    toml::from_str(&content)
        .map_err(|e| format!("invalid TOML: {e}"))
}

//open up the project toml
fn open_log()-> Result<Config,()> {

    match load_config("projects.toml") {
        Ok(config) => {

            //println!("{config:#?}");
            Ok(config)

        },
        Err(e) => {
            eprintln!("config error: {e}");
            Err(())
        },
    }


}

//Es wird falsch in die toml geschrieben
fn set_amount(path: &str, new_config: Config) -> Result<(), String> {
    // 1. Read the raw file as a string
    let content = std::fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;

    // 2. Parse into a mutable document
    let mut doc = content.parse::<DocumentMut>().map_err(|e| format!("parse error: {e}"))?;

    // 3. Assign the new value
    //doc["General"]["amount"] = value(new_amount);
    let amount = new_config.general.amount;
    doc["general"]["amount"] = toml_edit::value(amount);

    // 4. Serialize back and overwrite the file
    std::fs::write(path, doc.to_string()).map_err(|e| format!("write error: {e}"))
}

// === ADDING PROJECT ===

fn add_project(mut config: Config){

    //Asking for Directory Paths
    //println!("\x1b[93;1mPlease enter the full path to the directory you want to safe:\x1b[0m");
    //let mes = "TEXT";
    let input_origin = print_input("\x1b[93;1mPlease enter the full path to the directory you want to safe:\x1b[0m");
    println!("received = {}", input_origin);

    //Loop where the path is beeing verified
    // If wrong youll be asked to enter again or go back
    // so actually the part where the amount is changed should first hapen after the paths have been verified
    //println!("\x1b[93;1mPlease enter the full path to the directory where the original should be safed to:\x1b[0m");
    let input_dest = print_input("\x1b[93;1mPlease enter the full path to the directory the original should be safed to:\x1b[0m");
    println!("received = {}", input_dest);

    let mut choice = print_input("\x1b[93;1mDo you want to add an additional Destination? [y/n]\x1b[0m");

    loop {
        match choice.trim() {
            "y" => {
                println!("Adding additional Destionation");
                break;
            },
            "n" => {
                break;
            }
            _ => {
                //println!("Enter valid Input!");
                choice = print_input("\x1b[93;1mEnter valid input! [y/n]\x1b[0m");
            }
        }
    }

    let choice = print_input("\x1b[93;1mDo you want to enable the daemon mode? [y/n]\x1b[0m");
    println!("received = {}", choice);

    let mut new_amount = 1;
    let amount = config.general.amount;
    let am_int: u32 = amount.trim().parse().unwrap();
    new_amount = am_int + new_amount;
    let am_char = new_amount.to_string();

    // amount value in the struct is changed
    config.general.amount = am_char;

    // printing struct with new value
    println!("\n");
    println!("amount = {}", config.general.amount);
    println!("version = {}", config.general.version);
    println!("developer = {}", config.general.developer);

    // new struct is passed to function which writes into the toml
    if let Err(err) = set_amount("projects.toml", config){
        println!("amount couldnt be changed: {}", err);
    }

}

// === ADDING PROJECT OPTION MENU ===

pub fn edit_projects(){

    //Options for project editing are printed
    println!("  \x1b[1;94ma:\x1b[0m add new project");
    println!("  \x1b[1;94md:\x1b[0m delete existing project/read toml");
    println!("  \x1b[1;94mb:\x1b[0m go back");

    let input = print_input("");

    match input.trim(){
        "a" => {

            match open_log() {
                Ok(config) => {
                    //string to int
                    add_project(config);

                },
                Err(()) => println!("couldnt add"),
            }
        },
        "d" => {
            println!("deleting project");
            //toml_reader();
        }
        "b" => {
            println!{"back to main menu"};

        },
        _ => {
            println!("bruh");
        },
    }

}
