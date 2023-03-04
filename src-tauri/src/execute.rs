use std::{process::Stdio, error::Error as StdError, io::{ErrorKind, Error, BufReader, BufRead}};
use crate::{dirs, color};
use ::execute;

pub fn color(command: &str) -> Result<(), Box<dyn StdError>> {
    let stdout = execute::command(command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;
    
    let reader = BufReader::new(stdout);
    
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", color::blue(&line)));
    Ok(())
}

pub fn default(command: &str) -> Result<(), Box<dyn StdError>> {
    let stdout = execute::command(command)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Could not capture standard output."))?;
    
    let reader = BufReader::new(stdout);
    
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
    Ok(())
}

pub fn string_return(command: &str) -> Result<String, Box<dyn StdError>> {
    let output = execute::command(command).output()?;
    Ok(String::from_utf8(output.stdout)?)
}

pub fn no_output(command: &str) -> Result<(), Box<dyn StdError>> {
    execute::command(command).output()?;
    Ok(())
}