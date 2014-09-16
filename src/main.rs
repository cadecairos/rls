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

  let matches = ls::get_matches(args, opts);

  if matches.opt_present("h") {
    ls::rls_usage(opts);
    return;
  }

  let show_hidden = matches.opt_present("a");

  let target_directory_path: Path = ls::get_target(matches);

  let mut paths = ls::read(target_directory_path);

  paths.sort_by(ls::sort_paths);

  ls::output_to_term(paths, show_hidden);

}




