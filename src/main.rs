use std::env;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::Path;

fn read_file(path: &Path) {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", display, err.to_string()),
        Ok(file) => file,
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Err(err) => panic!("couldn't read {}: {}", display, err.to_string()),
        Ok(_) => print!("{}", file_content),
    }
}

fn handle_single_argument(arg: &String) {
    let path = Path::new(arg.as_str());
    if !path.exists() {
        return;
    }

    if path.is_file() {
        read_file(path);
    } else if path.is_dir() {
        read_directory(arg.as_str()).unwrap();
    }
}

fn handle_multiple_arguments(args: &Vec<String>) {
    args.into_iter().for_each(|i| {
        handle_single_argument(i);
    });
}

fn handle_no_argument() -> io::Result<()> {
    read_directory(".")?;
    Ok(())
}

fn read_directory(path: &str) -> Result<(), io::Error> {
    let entries = fs::read_dir(path)?;
    Ok(for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("{}", entry.file_name().to_string_lossy());
        }
    })
}

fn handle_args(args: env::Args) {
    let arg_list: Vec<String> = args.collect();

    let last = arg_list.last().unwrap();

    match arg_list.len() {
        1 => handle_no_argument().unwrap(),
        2 => handle_single_argument(&last),
        _ => handle_multiple_arguments(&arg_list[1..].to_vec()),
    }
}

fn main() {
    let args: env::Args = env::args();
    handle_args(args);
}
