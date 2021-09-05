use std::env;
use std::io;
use std::collections::HashMap;
use walkdir::{ DirEntry, WalkDir };

struct File {
    path: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = args.get(1).unwrap();
    let nested_files = WalkDir::new(folder)
        .into_iter()
        .filter_entry(|entry| entry.metadata().unwrap().is_file());

    let mut hashes: HashMap<&str, File> = HashMap::new();
}
