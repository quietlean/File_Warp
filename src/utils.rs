use std::io::{self};
use std::io::Write;
//use std::ops::ControlFlow::Continue;
use std::thread::sleep;
use std::{error::Error, fs::File};
use std::time::Duration;
//use csv_to_table::iter::CsvRecords;
use indicatif::ProgressBar;
//use toml_edit::Value::Array;
//use toml_edit::Item::Table;
use std::process::Command;
use std::path::{Path}; //, PathBuf
use serde::Deserialize;
use toml_edit::{Array, DocumentMut, Item, Table, Value}; //, value
use mac_notification_sys::*; // Sending System Notifications
use rfd; // Opening Finder to select Folder

#[derive(Debug, Deserialize)]
struct Config {
    general: General,
    project: Project,
}

#[derive(Debug, Deserialize)]
struct General {
    amount: i64,
    version: String,
    developer: String,
}

// gotta change them to vectors
#[derive(Debug, Clone, serde::Serialize, Deserialize)]
struct Project {
    name: Vec<String>,
    origin: Vec<String>,
    destination: Vec<Vec<String>>,
    daemon: Vec<i64>
}

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

pub fn menu_print_intro(){
    //User options are printed.
    //let mut count = String::new();
    let count = match open_toml() {
        Ok(config) => {
            //string to int
            config.general.amount
        },
        Err(()) => {
            println!("New config file needs to be build");
            write_new_config();
            0
        },
    };

    //let count_int: u32 = count.parse().unwrap();

    // Depending on the magnitude of "amount" I have to change the amount of spaces in the line where the "amount" is printed.
    // If amount gets bigger then 9 (x > 10 takes two spaces "--") I need less spaces ... the same with 100
    // Amount should be limited to 999 or the output will bug if amount is > 999
    let mut spaces = "";
    if count < 10 {
        spaces = "  ";
    } else if count < 100 {
        spaces = " ";
    }

    // Escape Codes: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
    // ASCII Symboles: https://www.ee.ucl.ac.uk/mflanaga/java/HTMLandASCIItableC1.html
    println!("\n");
    println!("\x1b[48;5;225;30m    Welcome to File Moth    \x1b[0m");  // println!("\x1b[47;30m⊛   Welcome to File Warp!  ⊛\x1b[0m");
    //println!("┌──────────────────────────┐");
    println!("│You have \x1b[1;95m{}\x1b[0m projects.    {}│", count, spaces); // \x1b[38;5;239mv1.0\x1b[0m
    println!("├──────────────────────────┤");
    println!("│\x1b[1;92m1:\x1b[0m safe a project         │");
    println!("│\x1b[1;92m2:\x1b[0m safe all projects      │");
    println!("│\x1b[1;92m3:\x1b[0m edit projects          │");
    println!("│\x1b[1;92m4:\x1b[0m view projects          │");
    //println!("│\x1b[1;92m5:\x1b[0m view activity log    │");
    println!("│\x1b[1;92mq:\x1b[0m quit application       │");
    println!("└──────────────────────────┘");

    //let input = print_input("hahahahahahp");
    //println!("{}", input);
}

pub fn menu_print () {
    println!("\n");
    println!("\x1b[107;30mBack to the main menu.\x1b[0m");
    println!("\x1b[1;92m1:\x1b[0m safe a project         ");
    println!("\x1b[1;92m2:\x1b[0m safe all projects      ");
    println!("\x1b[1;92m3:\x1b[0m edit projects          ");
    println!("\x1b[1;92m4:\x1b[0m view projects          ");
    println!("\x1b[1;92mq:\x1b[0m quit application       ");

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

    let path = "projects_test.toml";
    let path_check = Path::new(path);

    // I think this can only be done once -> otherwise error
    // tell macOS which app "owns" the notification first
        let bundle = get_bundle_identifier_or_default("terminal"); // or "finder", your own bundle id, ...
        if let Err(error) = set_application(&bundle){
            println!("error: {}", error);
        }

    // Builds and sends a System Notification
    // When Terminal is in Scope (aka you have the terminal-window in focus) the message won't be shown
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
        if let Err(error) = File::create(path) {
            println!("Error while creating config file: {}", error);
        }
    }
}

fn read_toml(path: &str) -> Result<Config, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("can't read file: {e}"))?;
    toml::from_str(&content)
        .map_err(|e| format!("invalid TOML: {e}"))
}

//open up the project toml
fn open_toml()-> Result<Config,()> {

    match read_toml("projects_test.toml") {
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

// Pause until enter is pressed
fn pause() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read input");
}

// Vectos are transformed into arrays
// With <T: Into<Value>> both Strings and i64 can be passed  to the function
fn vec_to_array<T: Into<Value>>(vec: Vec<T>) -> Array{
    let mut arr = Array::new();
    for x in vec {
        arr.push(x);
    }

    return arr;
}

fn write_new_config () {

    let path = "projects_test.toml";
    let mut doc = DocumentMut::new();

    let mut table_general = Table::new();
    table_general.insert("amount", 0.into());
    table_general.insert("version", "0.1".into());
    table_general.insert("developer", "quietlean".into());


    // This Table contains the Key names and empty arrays which can be filled afterwards
    let empty_array = || Item::Value(Value::Array(Array::new()));
    let mut table_project = Table::new();
    table_project.insert("name", empty_array());
    table_project.insert("origin", empty_array());
    table_project.insert("destination", empty_array());
    table_project.insert("daemon", empty_array());

    doc["general"] = Item::Table(table_general);
    doc["project"] = Item::Table(table_project);

    if let Err(error) = std::fs::write(path, doc.to_string()) {
        println!("Config File couldnt be renewt: {}", error);
    }
}

fn change_toml (proj: Project, amount: i64) -> Result<(), Box<dyn Error>>{
    let path = "projects_test.toml";
    // 1. Read the raw file as a string
    let content = std::fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;

    // 2. Parse into a mutable document
    let mut doc = content.parse::<DocumentMut>().map_err(|e| format!("parse error: {e}"))?;

    //println!("++ toml has been read");

    // turning Vec into Array
    // Toml can only take Arrays

    let mut arr = Array::new();
    for entry in proj.destination {
        arr.push(vec_to_array(entry));
    }
    // The struct is serealized to toml table
    //let mut doc = DocumentMut::new();

    // creating table
    let mut new_table = Table::new();
    new_table.insert("name", vec_to_array(proj.name).into());
    new_table.insert("origin", vec_to_array(proj.origin).into());
    new_table.insert("destination", arr.into());
    new_table.insert("daemon", vec_to_array(proj.daemon).into());

    //println!("++ struct to table");

    // New Table is inserted into the document
    doc["project"] = Item::Table(new_table);

    // Amount is changed + 1
    doc["general"]["amount"] = toml_edit::value(amount);

    //println!("++ create new doc");
    std::fs::write(path, doc.to_string())?;

    //println!("++ wrtiten to toml");

    Ok(())
}

// === ADDING PROJECT ===

fn add_project(mut config: Config){

    //let mut new_proj = Project {
    //    name: Vec::new(),
    //    origin: Vec::new(),
    //    destination: Vec::new(),
    //    daemon: Vec::new(),
    //};

    // I cloud also just write into the config thats passed
    let mut new_proj = config.project.clone();

    let input_string = print_input("\x1b[93;1mPlease enter a projectname:\x1b[0m");
    new_proj.name.push(input_string);

    println!("\x1b[93;1mPlease select the directory you want to safe.\x1b[0m \x1b[1;37m[Press Enter]\x1b[0m");
    //pause();

    // Finder is opend
    //let mut new_path = String::new();
    let folder = rfd::FileDialog::new()
            .set_title("Pick a folder")
            .pick_folder();

    let mut new_path = match folder {
        Some(path) => {
            println!("Selected: {}", path.display());
            path.to_string_lossy().into_owned()
        },
        None => {
            println!("New project cancled. Back to Menu.");
            return;
        },
    };

    new_proj.origin.push(new_path);

    println!("\x1b[93;1mPlease select the directory you want to safe the original to.\x1b[0m \x1b[1;37m[Press Enter]\x1b[0m");
    //pause();

    let mut dest_vec = Vec::new();

    let folder = rfd::FileDialog::new()
            .set_title("Pick a folder")
            .pick_folder();

    match folder {
        Some(path) => {
            println!("Selected: {}", path.display());
            new_path = path.to_string_lossy().into_owned();
        },
        None => {
            println!("New project cancled. Back to Menu.");
            return;
        },
    }
     // Path is added to destination Vector
    dest_vec.push(new_path);


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

    new_proj.destination.push(dest_vec);

    choice = print_input("\x1b[93;1mDo you want to enable the daemon mode? [y/n]\x1b[0m");
    //println!("received = {}", choice);
    loop {
        match choice.trim() {
            "y" => {
                println!("Daemon activated");
                new_proj.daemon.push(1);
                break;
            },
            "n" => {
                new_proj.daemon.push(0);
                break;
            }
            _ => {
                //println!("Enter valid Input!");
                choice = print_input("\x1b[93;1mEnter valid input! [y/n]\x1b[0m");
            }
        }
    }

    config.general.amount = config.general.amount + 1;

    let proj_pass = new_proj.clone();

    // the finished Project and the new amount are passed to adding function
    if let Err(error) = change_toml(proj_pass, config.general.amount) {
        println!("error while adding: {}", error);
    }

    // the position if the new project entrys in the Vectors
    // used to print the new entrys
    let position = new_proj.name.len() - 1;

    println!("\n");
    println!("\x1b[107;30mA new project has been created.\x1b[0m");
    println!("name:  \x1b[1;95m{}\x1b[0m", new_proj.name[position]);
    println!("origin:  \x1b[1;95m{}\x1b[0m", new_proj.origin[position]);
    println!("selected destinations:");
    //println!("destination: \x1b[1;95m{}\x1b[0m", new_proj.destination);
    let mut count = 1;
    for x in &new_proj.destination[position] {
        println!("{}]  \x1b[1;95m{}\x1b[0m",count , x);
        count = count + 1;
    }
}

// === DELETE PROJECT ===

fn delete_project (mut config: Config){

    if config.general.amount == 0 {
        println!("You don't have any projects.");
        return;
    }

    println!("\n\x1b[107;30mProject Overview:\x1b[0m");
    view_projects("n");
    println!("\n");
    println!("Press [b] if you want to go back.");

    // Array entrys can only be selected with usize numbers but amount is i64 -> its converted to usize
    let mut amount = match usize::try_from(config.general.amount) {
        Ok(v) => v,
        Err(_) => {
            println!("error during conversion of 'amount'");
            return
        },
    };

    // The index of the project in the array
    let mut index = 0;
    let mut choice_all = "n";
    let mut input = print_input("\x1b[93;1mEnter the Number of the project you want to delete. If you want to delte all projects enter [all].\x1b[0m");

    loop {

        if input == "b" {
            return;
        }

        if input == "all" {
            choice_all = "y";
            index = amount - 1;
            break;
        }

        match input.parse::<usize>() {
            Ok(num) => {


                // checking if the number is valid
                if num > amount || num < 1 {
                    input = print_input("\x1b[93;1mEnter valid input!\x1b[0m");
                    continue;
                }
                 // index is calculated
                index = num - 1;
                break;
            }
            Err(_) => {
                input = print_input("\x1b[93;1mEnter valid input!\x1b[0m");
                continue;
            }
        };
    }

    // Case all delete and normal delete
    if choice_all == "y" {
        input = print_input("\x1b[93;1mAre you sure you want to delete all projects? [y/n]\x1b[0m");
    } else {
        println!("You have selected project {}.", index + 1);
        input = print_input("\x1b[93;1mAre you sure you want to delete this project? [y/n]\x1b[0m");
    }

    loop {
        match input.trim() {
            "y" => {

                if choice_all == "y" {
                    println!("++ first index = {}", index);
                    // Removing all projects
                    // They need to be removed in reversed order. Otherwise the indexes wont exist no more after half the Vec is gone.
                    for i in (0..=index).rev() {
                        println!("++ index = {}", i);

                        config.project.name.remove(i);
                        config.project.origin.remove(i);
                        config.project.destination.remove(i);
                        config.project.daemon.remove(i);
                    }
                    println!("All projects have been removed");
                    amount = 0;
                    break;
                } else {
                    // Removing project at index
                    config.project.name.remove(index);
                    config.project.origin.remove(index);
                    config.project.destination.remove(index);
                    config.project.daemon.remove(index);

                    println!("The project has been removed.");
                    amount = amount - 1;
                    break;
                }

            },
            "n" => {
                return;
            }
            _ => {
                //println!("Enter valid Input!");
                input = print_input("\x1b[93;1mEnter valid input! [y/n]\x1b[0m");
            }
        }
    }

    let amount_int: i64 = amount as i64;
    if let Err(error) = change_toml(config.project, amount_int) {
        println!("Erorr while changing config file: {}", error);
    }

}

// === ADDING PROJECT OPTION MENU ===

pub fn edit_projects(){

    //Options for project editing are printed
    println!("  \x1b[1;94ma:\x1b[0m add new project");
    println!("  \x1b[1;94md:\x1b[0m delete existing project");
    println!("  \x1b[1;94mb:\x1b[0m go back");

    let input = print_input("");

    match input.trim(){
        "a" => {

            match open_toml() {
                Ok(config) => {
                    //string to int
                    add_project(config);

                },
                Err(()) => println!("couldnt add"),
            }
        },
        "d" => {
            match open_toml() {
                Ok(config) => {
                    //string to int
                    delete_project(config);

                },
                Err(()) => println!("Error while opening config file."),
            }
        }
        "b" => {
            println!{"back to main menu"};

        },
        _ => {
            println!("bruh");
        },
    }
}

// All the projectnames are printed
pub fn view_projects (addiotional: &str) {

    //let mut count = 0;
    match open_toml() {
        Ok(config) => {

            // Array entrys can only be selected with usize numbers but amount is i64 -> its converted to usize
            let amount = match usize::try_from(config.general.amount) {
                Ok(v) => v,
                Err(_) => {
                    println!("error during conversion of 'amount'");
                    return
                },
            };

            for proj in 0 .. amount {
                println!("\x1b[1;95m{}]\x1b[0m {}", proj + 1, config.project.name[proj]);

                // In case only the names are wanted ("n")the rest is not printed
                if addiotional == "y" {
                    println!("  origin: ");
                    println!("  - {}", config.project.origin[proj]);
                    println!("  destinations:");
                    for x in &config.project.destination[proj] {
                        println!("  - {}", x);
                    }
                    println!("\n");
                }


                //count = count + 1;
            }

        },
        Err(()) => println!("error while opening config file."),
    }
}

pub fn art (){

    let moth_3 = "\x1b[38;5;55m
                            ü.      _.
                             '\\-o ,/''
            ___,x.Y----D-----''.b.'._----....b___,_.
         ,-''    ,.,b  .     \\'i'',o'` .    ,__4'''x\\\\.
        a'   `'  |. |  `_  ,,  ''A'     _.  |'  \\      '(.
        \\.        'g'    ,-'    `     `.   '-v-'    '  ,|
          ' b. .       ,''      L.      `.            S'
             'B.'i'  ,Y,Y'@'. `/'`-  /F-\\'\\.   ,- ,_o''
                '`-''  |_ _'        |_  |  `...---'
                   \\. Fuck     da    System   |-
                    `\\_        ,,L         ,,
                      ''f--'r'o' ''`---u--''
    \x1b[0m";


println!("{}", moth_3);
//println!("Fuck da System");

}
