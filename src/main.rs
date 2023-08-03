use std::env;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::path::Path;

fn read_file(path: &Path) {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
        Ok(_) => print!("{}", s),
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
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn handle_args(args: env::Args) {
    let arg_list: Vec<String> = args.collect();

    let last = arg_list.last();

    match arg_list.len() {
        1 => print_files_in_cwd().unwrap(),
        2 => read_single_file(&last.unwrap()),
        _ => read_multiple_files(&arg_list[1..].to_vec()),
    }
}

fn main() {
    let args: env::Args = env::args();
    handle_args(args);
}
