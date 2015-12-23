use std::env;
use std::fs;
use std::path::Path;
use std::io;

// Main
fn main() {
    println!("Git Sync");
    // TODO read options

    // Current dir
    let dir = env::current_dir().unwrap();
    println!("The current directory is {:?}", dir);
    let result = process_dir(dir.as_path());
    let message = match result {
        Result::Ok(true) => format!("[OK] with update(s)"),
        Result::Ok(false) => format!("[OK] already synchronised"),
        Result::Err(err) => format!("[ERR] Oops! {}", err),
    };
    println!("{:?}", message);
}

// Process the root dir
fn process_dir(dir: &Path) -> io::Result<bool> {
    for entry in try!(fs::read_dir(dir)) {
        let dir = try!(entry);
        println!("{:?}", dir.path());
    }
    Ok(true)
}
