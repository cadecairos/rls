extern crate term;
extern crate getopts;

use std::os;
use std::path;
use std::io::fs;

pub fn sort_paths(a: &Path, b: &Path) -> Ordering {
  let a_str = match a.filename_str() {
    Some(s) => { s },
    None => { fail!("Not a path") }
  };

  let b_str = match b.filename_str() {
    Some(s) => { s },
    None => { fail!("not a path") }
  };

  return (a_str.trim_left_chars('.')).cmp(&(b_str.trim_left_chars('.')));
}

pub fn get_args() -> Vec<String> {
  return os::args();
}

pub fn output_to_term(paths: Vec<Path>, show_hidden: bool) {
  let mut t = term::stdout().unwrap();

  let mut paths_iterator = paths.iter();

  loop {
    match paths_iterator.next() {
      Some(path) => {
        let filename = match path.filename_str() {
          Some(filename) => {
            if !show_hidden && filename.char_at(0) == '.' {
              continue;
            }
            filename
          }
          None => continue
        };

        match path.is_dir() {
          true => {
            t.fg(term::color::BLUE).unwrap();
            (writeln!(t, "{}", filename)).unwrap();
          },
          false => {
            t.reset().unwrap();
            (writeln!(t, "{}", filename)).unwrap();
          }
        }
      },
      None => { break }
    }
  }
}

// get the target directory, either the first command line argument or CWD
pub fn get_target(matches: getopts::Matches) -> Path {
  if !matches.free.is_empty() {
    path::posix::Path::new(matches.free[0].clone())
  } else {
    get_cwd()
  }
}

pub fn read(dir: Path) -> Vec<Path> {
  match fs::readdir(&dir) {
    Ok(paths) => { paths },
    Err(e) => { fail!(e.to_string()) }
  }
}

pub fn get_matches(args: Vec<String>, opts: &[getopts::OptGroup]) -> getopts::Matches {
  match getopts::getopts(args.tail(), opts) {
    Ok(m) => { m },
    Err(f) => { fail!(f.to_string()) }
  }
}

pub fn get_cwd() -> Path {
  os::getcwd()
}

pub fn rls_usage(opts: &[getopts::OptGroup]) {
  println!("{}", getopts::usage("Usage: rls [options]", opts).as_slice());
}
