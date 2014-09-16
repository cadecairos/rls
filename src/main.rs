extern crate rls;
extern crate getopts;

use rls::ls;

fn main() {
  let args: Vec<String> = ls::get_args();

  let opts = [
    getopts::optflag("l", "list", "long list format"),
    getopts::optflag("a", "all", "Do not ignore files starting in ."),
    getopts::optflag("h", "help", "print this help menu")
  ];

  // process command line arguments and flags
  let matches: getopts::Matches = ls::get_matches(args, opts);

  // show rls command usage
  if matches.opt_present("h") {
    ls::rls_usage(opts);
    return;
  }

  // Display files that begin with a '.'
  let show_hidden: bool = matches.opt_present("a");

  // the first argument is the directory to list, otherwise use CWD
  let target_directory_path: Path = ls::get_target(matches);

  // construct a Vector of Paths within the given directory
  let mut paths: Vec<Path> = ls::read(target_directory_path);

  // Mutate paths by sorting it lexicographically (ignoring the '.' of hidden files and folders)
  paths.sort_by(ls::sort_paths);

  // print the paths to the terminal
  ls::output_to_term(paths, show_hidden);

}




