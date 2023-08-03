use std::env;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::Path;

fn read_file(path: &Path) -> io::Result<()> {
    let mut file = File::open(&path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    println!("{}", file_content);
    Ok(())
}

fn handle_single_argument(arg: &String) -> io::Result<()> {
    let path = Path::new(arg.as_str());
    if !path.exists() {
        return Ok(());
    }

    if path.is_file() {
        read_file(path)?;
    } else if path.is_dir() {
        read_directory(arg.as_str())?;
    }
    Ok(())
}

fn handle_multiple_arguments(args: &[String]) -> io::Result<()> {
    for arg in args {
        handle_single_argument(arg)?;
    }
    Ok(())
}

fn handle_no_argument() -> io::Result<()> {
    read_directory(".")?;
    Ok(())
}

fn read_directory(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }
    Ok(())
}

fn handle_args(args: env::Args) -> io::Result<()> {
    let arg_list: Vec<String> = args.collect();

    match arg_list.len() {
        1 => handle_no_argument()?,
        2 => handle_single_argument(&arg_list.last().unwrap())?,
        _ => handle_multiple_arguments(&arg_list[1..])?,
    }
    Ok(())
}

fn main() {
    let args: env::Args = env::args();
    if let Err(err) = handle_args(args) {
        eprintln!("Error: {}", err);
    }
}
