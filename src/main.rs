use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::io;
use std::result;

// Main
fn main() {
    println!("Git Sync");
    // TODO read options

    // Current dir
    let dir = env::current_dir().unwrap();
    println!("The current directory is {:?}", dir);

    // Process
    let result = process_dirs(dir.as_path());
    // FIXME do not use format! for OK case
    let message = match result {
        Ok(true) => format!("[OK] with update(s)"),
        Ok(false) => format!("[OK] already synchronised"),
        Err(err) => format!("[ERR] Oops! {}", err),
    };
    // FIXME chain call
    println!("{:?}", message);
}

// Process the root dir (flat)
fn process_dirs(parent_dir: &Path) -> io::Result<bool> {
    let mut res = false;
    for entry in try!(fs::read_dir(parent_dir)) {
        let child: DirEntry = try!(entry);
        let path_buf = child.path();
        let path = path_buf.as_path();
        res = process_dir(&path) || res;
    }
    Ok(res)
}

fn process_dir(dir: &Path) -> bool {
    println!("{:?}", dir);
    // println!("{:?}", dir.path().as_path());
    return true;
}

// fn is_git_repository(dir: &Path) -> bool {
//   unimplemented!()
// }
