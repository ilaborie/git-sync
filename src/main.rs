extern crate getopts;
use getopts::Options;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::io;

// Main
fn main() {
    // read options
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    // opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }
    // let output = matches.opt_str("o");
    let input_dir = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        ".".to_owned()
    };

    // let dir = env::current_dir(input_dir);
    let main_dir = Path::new(&input_dir);
    println!("Git Sync {:?}", main_dir);

    // Process
    let result = process_dirs(&main_dir);
    // FIXME do not use format! for OK case
    let message = match result {
        Ok(true) => format!("[OK] with update(s)"),
        Ok(false) => format!("[OK] already synchronised"),
        Err(err) => format!("[ERR] Oops! {}", err),
    };
    // FIXME chain call
    println!("{}", message);
}

fn print_usage(opts: Options) {
    let brief = format!("Usage: git-sync [options] DIR");
    print!("{}", opts.usage(&brief));
}


// Process the root dir (flat)
fn process_dirs(parent_dir: &Path) -> io::Result<bool> {
    let mut res = false;
    for entry in try!(fs::read_dir(parent_dir)) {
        let child: DirEntry = try!(entry);
        let path_buf = child.path();
        let path = path_buf.as_path();
        if path.is_dir() {
            res = process_dir(&path) || res;
        }
    }
    Ok(res)
}

fn process_dir(dir: &Path) -> bool {
    println!("{:?}", dir);
    return true;
}

// fn is_git_repository(dir: &Path) -> bool {
//   unimplemented!()
// }
