use clap::{arg, App};
use walkdir::{DirEntry, WalkDir};

fn main() {
    let matches = App::new("walk")
        .version("1.0")
        .author("by Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Walk into directories and files!")
        .arg(arg!([path] "Directories"))
        .arg(arg!(-d --dir "Only walk directories"))
        .arg(arg!(-f --file "Only walk files"))
        .get_matches();

    let mut dir = ".";
    let mut is_file = true;
    let mut is_dir = false;
    if let Some(val) = matches.value_of("dir") {
        dir = val;
    }

    if matches.is_present("file") {
        is_file = true;
        is_dir = false;
    }

    if matches.is_present("dir") {
        is_file = false;
        is_dir = true;
    }

    println!("Path: {:?}", dir);
    println!("Is dir: {:?}", is_dir);
    println!("Is file: {:?}", is_file);

    if is_dir {
        WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|v| v.ok())
            .filter(|item| item.path().is_dir())
            .for_each(|entry| println!("{}", entry.path().display()));
    }

    if is_file {
        WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|v| v.ok())
            .filter(|item| item.path().is_file())
            .for_each(|entry| println!("{}", entry.path().display()));
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".git"))
        .unwrap_or(false)
}
