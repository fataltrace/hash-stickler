use std::env;
use std::io;

fn hash () {

}

fn is_folder (folder_path: &str) -> io::Result<()> {
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let folder = args.get(1).unwrap();

    is_folder(folder);

    println!("{}", folder);
}
