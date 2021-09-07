use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA512};
use std::collections::hash_map::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use walkdir::WalkDir;

fn sha512_digest<R: Read>(mut reader: R) -> io::Result<Digest> {
    let mut context = Context::new(&SHA512);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = args.get(1).unwrap();
    let mut hashes: HashMap<String, Vec<String>> = HashMap::new();
    for file in WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.metadata().unwrap().is_file())
    {
        let file_path = file.path();
        let input = File::open(file_path).unwrap();
        let reader = BufReader::new(input);
        let digest = sha512_digest(reader).unwrap();
        let hash = HEXUPPER.encode(digest.as_ref());
        let file_path_as_string = file_path.to_str().unwrap().to_string();

        hashes
            .entry(hash)
            .and_modify(|e| e.push(file_path_as_string.clone()))
            .or_insert(vec![file_path_as_string]);
    }

    hashes.iter().for_each(|(index, paths)| {
        if paths.len() > 1 {
            println!("Found the same files with hash {} = {:?}", index, paths);
        }
    })
}
