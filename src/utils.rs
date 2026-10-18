use std::thread::sleep;
use std::{error::Error, fs::File};
use std::time::Duration;
use indicatif::ProgressBar;

pub fn file_read() -> Result<(), Box<dyn Error>> {
    //Open the file
    let file = File::open("src/project-log-komma.csv")?;

    //The CSV Reader is build
    //let mut reader = csv::Reader::from_reader(file);

    //for result in reader.records() {
        //let record = result?;
        //println!("{:?}", record);
        //}

    let table = csv_to_table::from_reader(file)?.to_string();

    println!("{}", table);


    Ok(())
}

pub fn print_options(){
    //User options are printed.
    println!("\x1b[1;95m1:\x1b[0m safe a project");
    println!("\x1b[1;95m2:\x1b[0m safe all projects");
    println!("\x1b[1;95m3:\x1b[0m edit projects");
    println!("\x1b[1;95m4:\x1b[0m view projects");
    println!("\x1b[1;95m5:\x1b[0m view activity log");
    println!("\x1b[1;95mq:\x1b[0m quit application");

}

pub fn loading(){

    // spinner from the "indicatif" crate
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));

    sleep(Duration::from_secs(3));

    spinner.finish();
    println!("done xo");
}
