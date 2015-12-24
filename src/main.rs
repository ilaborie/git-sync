extern crate getopts;
extern crate git2;
use getopts::Options;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::io;
use std::error::Error;
use git2::Repository;
use git2::string_array::StringArray;

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
    println!("Git Sync {:?}", main_dir.to_str());

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
        let child = try!(entry);
        let path_buf = child.path();
        let path = path_buf.as_path();
        if path.is_dir() {
            let dir_result = process_dir(&path);
            if dir_result.is_ok() {
                res = dir_result.unwrap() || res;
            } else {
                let gitErr = dir_result.err().unwrap();
                return Err(io::Error::new(io::ErrorKind::Other, gitErr));
            }
        }
    }
    Ok(res)
}

fn process_dir(dir: &Path) -> Result<bool, git2::Error> {
    let repo_path = dir.to_str().unwrap();
    let repo = Repository::open(repo_path);
    if repo.is_err() {
        Ok(false)// Not a git repository
    } else {
        process_repository(repo.ok().unwrap())
    }
}

fn process_repository(repo: Repository) -> Result<bool, git2::Error> {
    println!("Detect Git repository {:?}", repo.path());
    // Remotes
    let remotes: StringArray = try!(repo.remotes());
    if remotes.len() == 0 {
        println!("  [SKIP] no remotes");
    } else {
        for remote in remotes.iter() {
            println!("  Sync {}", remote.unwrap());
        }
    }
    Ok(true) // FIXME
}

fn process_repository_remote(repo: Repository, path: &str) -> Result<bool, git2::Error> {
    Ok(true) // FIXME
}
