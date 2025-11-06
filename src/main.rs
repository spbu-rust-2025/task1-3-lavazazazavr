use std::fs::File;
use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let input = input.trim();

    let path = input.trim_end_matches('/');
    if path.is_empty() || path == "/" {
        println!("failure");
        return;
    }
    let file: Result<File, io::Error> = File::open(path);

    match file {
        Ok(_) => {
            println!("success");
        }
        Err(_) => {
            println!("failure");
        }
    }
}
