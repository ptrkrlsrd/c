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

fn read_single_file(arg: &String) {
    let path = Path::new(arg.as_str());
    if path.exists() && path.is_file() {
        read_file(path);
    }
}

fn read_multiple_files(args: &Vec<String>) {
    args.into_iter().for_each(|i| {
        read_single_file(i);
    });
}

fn print_files_in_cwd() -> io::Result<()> {
    let entries = fs::read_dir(".")?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("{}", entry.file_name().to_string_lossy());
        }
    }
    Ok(())
}

fn handle_args(args: env::Args) {
    let arg_list: Vec<String> = args.collect();

    let last = arg_list.last().unwrap();

    match arg_list.len() {
        1 => print_files_in_cwd().unwrap(),
        2 => read_single_file(&last),
        _ => read_multiple_files(&arg_list[1..].to_vec()),
    }
}

fn main() {
    let args: env::Args = env::args();
    handle_args(args);
}
