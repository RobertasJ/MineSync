use std::{process::{Command, Stdio}, error::Error as StdError, io::{ErrorKind, Error, BufReader, BufRead}};

use crossterm::style::{Stylize};


pub mod config;

pub mod git;


pub fn run_command_with_stdout(command: &str, args: Vec<&str>, color: bool) -> Result<(), Box<dyn StdError>> {
    // Execute the command
    let stdout = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;

        let reader = BufReader::new(stdout);
    
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", if color { line.blue() } else { line.stylize() }));
    Ok(())
}

