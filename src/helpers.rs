use std::process::{Command, Stdio};
use regex::Regex;

#[allow(dead_code)]
pub fn r(str: &'static str) -> Regex {
  Regex::new(str).unwrap()
}

pub fn cucumber_command() -> Command {
  let mut command = Command::new("cucumber");
  command.stdout(Stdio::inherit());
  command.stderr(Stdio::inherit());
  command
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_makes_a_regex() {
    let regex = r("^Hello Regex$");
    assert!(regex.is_match("Hello Regex"));
  }
}
