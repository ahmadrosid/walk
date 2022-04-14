use clap::{arg, App};
use crossbeam::channel;
use ignore::WalkBuilder;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
enum DirEntry {
    Y(ignore::DirEntry),
}

impl DirEntry {
    fn path(&self) -> &Path {
        match *self {
            DirEntry::Y(ref y) => y.path(),
        }
    }
}

fn main() {
    let matches = App::new("walk")
        .version("1.0")
        .author("by Ahmad Rosid <alahmadrosid@gmail.com>")
        .about("Walk into directories and files!")
        .arg(arg!([path] "Directories"))
        .arg(arg!(-d --dir "Only walk directories"))
        .arg(arg!(-f --file "Only walk files"))
        .arg(arg!(-i --ignore "Exclude ignore file"))
        .get_matches();

    let dir = match matches.value_of("path") {
        Some(val) => val,
        _ => ".",
    };

    let is_dir = matches.is_present("dir");
    let ignore = matches.is_present("ignore");

    let (tx, rx) = channel::bounded::<DirEntry>(100);
    let stdout_thread = thread::spawn(move || {
        let mut stdout = io::BufWriter::new(io::stdout());
        for dent in rx {
            write_path(&mut stdout, dent.path(), is_dir);
        }
    });

    let walker = WalkBuilder::new(dir)
        .standard_filters(ignore)
        .threads(6)
        .build_parallel();
    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            use ignore::WalkState::*;

            tx.send(DirEntry::Y(result.unwrap())).unwrap();
            Continue
        })
    });
    drop(tx);
    stdout_thread.join().unwrap();
}

#[cfg(unix)]
fn write_path<W: Write>(mut wtr: W, path: &Path, is_dir: bool) {
    use std::os::unix::ffi::OsStrExt;
    if is_dir && path.is_dir() {
        wtr.write(path.as_os_str().as_bytes()).unwrap();
        wtr.write(b"\n").unwrap();
        return;
    }

    if !is_dir && path.is_file() {
        wtr.write(path.as_os_str().as_bytes()).unwrap();
        wtr.write(b"\n").unwrap();
    }
}

#[cfg(not(unix))]
fn write_path<W: Write>(mut wtr: W, path: &Path, is_dir: bool) {
    if is_dir && path.is_dir() {
        wtr.write(path.to_string_lossy().as_bytes()).unwrap();
        wtr.write(b"\n").unwrap();
        return;
    }

    if !is_dir && path.is_file() {
        wtr.write(path.to_string_lossy().as_bytes()).unwrap();
        wtr.write(b"\n").unwrap();
    }
}
