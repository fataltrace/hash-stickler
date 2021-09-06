use std::env;
use std::io::{
    self,
    BufReader,
    Read,
    Write
};
use std::fs::File;
use std::collections::HashMap;
use walkdir::{ DirEntry, WalkDir };
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA512};

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
    let mut hashes: HashMap<String, String> = HashMap::new();
    
    for file in WalkDir::new(folder)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|entry| entry.metadata().unwrap().is_file()) {
        let input = File::open(file.path()).unwrap();
        let reader = BufReader::new(input);
        let digest = sha512_digest(reader).unwrap();
                        
        println!("SHA-512 digest is {}", HEXUPPER.encode(digest.as_ref()));

        hashes.insert(HEXUPPER.encode(digest.as_ref()), file.path().to_str().unwrap().to_string());
    }
}
