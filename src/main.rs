extern crate getopts; // FIXME use docopt http://docopt.org/
extern crate git2;
use getopts::Options;
use std::{env, fs, io};
// use std::fs::DirEntry;
use std::path::Path;
use std::error::Error;
use git2::{Repository, RemoteCallbacks, FetchOptions, AutotagOption, BranchType};
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
    println!("Git Sync {}", main_dir.to_str().unwrap());

    // Process
    let result = process_dirs(&main_dir);
    // FIXME do not use format! for OK case
    let message = match result {
        Ok(0) => format!("[OK] already synchronised"),
        Ok(1) => format!("[OK] with one update"),
        Ok(n) => format!("[OK] with {} updates", n),
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
fn process_dirs(parent_dir: &Path) -> io::Result<i32> {
    let mut updated: i32 = 0;
    for entry in try!(fs::read_dir(parent_dir)) {
        let child = try!(entry);
        let path_buf = child.path();
        let path = path_buf.as_path();
        if path.is_dir() {
            let dir_result = process_dir(&path);
            if dir_result.is_ok() {
                updated = updated + (dir_result.unwrap() as i32);
            } else {
                let git_err = dir_result.err().unwrap();
                return Err(io::Error::new(io::ErrorKind::Other, git_err));
            }
        }
    }
    Ok(updated)
}

// Process one dir
fn process_dir(dir: &Path) -> Result<bool, git2::Error> {
    let repo_path = dir.to_str().unwrap();
    let repo = Repository::open(repo_path);

    if repo.is_err() {
        Ok(false)// Not a git repository
    } else {
        process_repository(repo.ok().unwrap())
    }
}

// Process a repo
fn process_repository(repo: Repository) -> Result<bool, git2::Error> {
    println!("Detected git repository {:?}", repo.path());
    let mut res = false;
    let remotes: StringArray = try!(repo.remotes());
    if remotes.len() == 0 {
        println!("  [SKIP] no remotes");
    } else {
        for remote in remotes.iter() {
            if remote.is_some() {
                let result_remote = process_repository_remote(&repo, remote.unwrap());
                if result_remote.is_ok() {
                    res = result_remote.unwrap() || res;
                } else {
                    println!("  [WARN] unable to sync {}", result_remote.unwrap_err());
                }
            }
        }
    }
    Ok(res)
}

// process a repo (fetch + sync branch)
fn process_repository_remote(repo: &Repository, str_remote: &str) -> Result<bool, git2::Error> {
    let mut updated = 0;
    let mut remote = try!(repo.find_remote(str_remote));
    {
        // Extract refspecs
        let mut vec_refspecs = Vec::new();
        for refspec in remote.refspecs() {
            let rs = refspec.str();
            if rs.is_some() {
                let rfspc = rs.unwrap().to_owned().clone();
                vec_refspecs.push(rfspc);
            }
        }
        let refspecs: Vec<&str> = vec_refspecs.iter().map(|s| s.as_ref()).collect();
        println!("  Sync {} ({:?}) {:?}",
                 str_remote,
                 remote.url().unwrap(),
                 vec_refspecs);
        let msg = format!("git-sync {}", str_remote).to_owned();
        let reflog_msg: Option<&str> = Some(&*msg);

        let mut options = FetchOptions::new();
        options.download_tags(AutotagOption::All);
        let mut cbs = RemoteCallbacks::new();
        cbs.update_tips(|label, from, to| {
            println!("   {} {:?}..{:?}", label, from, to);
            updated += 1;
            true
        });
        options.remote_callbacks(cbs);
        try!(remote.fetch(&refspecs[..], Some(&mut options), reflog_msg));
        // try!(remote.fetch(&refspecs[..], None /* Some(&mut options) */, reflog_msg));
        // remote.fetch(&[refspecs], None, None);
    }

    // Branches
    let branches = vec!["master", "develop"];
    for branch in branches.iter() {
        match repo.find_branch(branch, BranchType::Local) {
            Ok(br) => println!("   try merge branch {:?}", br.name().unwrap().unwrap()),
            _ => println!("   {} not found, no sync", branch),
        }
    }
    // let result: bool = updated > 0;
    Ok(updated > 0)
}
