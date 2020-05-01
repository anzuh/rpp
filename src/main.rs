use std::env;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::fs::File;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let mut shell_command: String = String::from("sh");

    let args = env::args()
        .collect::<Vec<String>>();

    // check if filename was provided
    let filename = match args.len() {
        2 => Ok(&args[1]),
        _ => Err("Filename not specified") 
    };

    let file = File::open(filename?)?;
    let buffer = BufReader::new(file);

    for line in buffer.lines() {
        if let Ok(line) = line {
            if line.starts_with("!!") {
                println!("{}", process_line(line, &shell_command)?.trim_end());
            } else if line.starts_with("!#") {
                shell_command = String::from(line[2..].trim());                
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

fn process_line(line: String, shell_command: &String) -> Result<String, Box<dyn Error>>{
    let args = line[2..].trim_start()
        .split(" ")
        .collect::<Vec<_>>();    

    match args.len() {
        0 => Ok(String::from("NO COMMAND PROVIDED HERE")),
        _ => run(&args, shell_command)
    }

}

fn run(args: &[&str], shell_command: &String) -> Result<String, Box<dyn Error>> {
    let command = &args.join(" ");

    let output = Command::new(shell_command)
        .arg("-c")
        .arg(command)
        .output()?
        .stdout;

    Ok(String::from_utf8(output)?)
}
