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

    let dir = match matches.value_of("path") {
        Some(val) => val,
        _ => ".",
    };

    if matches.is_present("dir") {
        WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|v| v.ok())
            .filter(|item| item.path().is_dir())
            .for_each(|entry| println!("{}", entry.path().display()));
    }

    if matches.is_present("file") {
        walk_file(dir);
    }

    // If option not present walk file as default!
    walk_file(dir);
}

fn walk_file(dir: &str) {
    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|v| v.ok())
        .filter(|item| item.path().is_file())
        .for_each(|entry| println!("{}", entry.path().display()));
    std::process::exit(0);
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".git"))
        .unwrap_or(false)
}
