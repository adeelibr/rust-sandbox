use std::fs;
use std::path;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
  // The pattern to look for
  pattern: String,
  // The path to the file to read
  path: path::PathBuf,
}

fn find_matches(content: &str, pattern: &str) {
  for line in content.lines() {
    if line.contains(pattern) {
      println!("{}", line);
    }
  }
}

fn main() {
  let args = Cli::from_args();

  let content = fs::read_to_string(&args.path).expect("could not read file");
  find_matches(&content, &args.pattern);
}
