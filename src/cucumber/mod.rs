use regex::{Regex, Captures};

pub mod helpers;

pub trait SendableStep<World>: Send + Fn(&mut World, Captures) {}
impl<T, World> SendableStep<World> for T where T: Send + Fn(&mut World, Captures) {}

pub type Step<World> = Box<SendableStep<World, Output=()>>;

pub type Match<'a, 'b, World> = (Captures<'b>, &'a Step<World>);

pub type MatchResult<'a, 'b, World: 'a> = Result<Match<'a, 'b, World>, MatchError>;

pub trait CucumberRegistrar<World> {
  fn given(&mut self, Regex, Step<World>);
  fn when(&mut self, Regex, Step<World>);
  fn then(&mut self, Regex, Step<World>);
}

#[derive(Debug, Eq, PartialEq)]
pub enum MatchError {
  NoMatchingSteps,
  SeveralMatchingSteps
}

pub struct Cucumber<World> {
  givens: Vec<(Regex, Step<World>)>,
  whens: Vec<(Regex, Step<World>)>,
  thens: Vec<(Regex, Step<World>)>
}

impl <World> Cucumber<World> {

  pub fn new() -> Cucumber<World> {
    Cucumber {
      givens: Vec::new(),
      whens: Vec::new(),
      thens: Vec::new()
    }
  }

  pub fn match_given<'a,'b> (&'a self, str: &'b str) -> MatchResult<'a, 'b, World> {
    Cucumber::match_steps(&self.givens, str)
  }

  pub fn match_when<'a,'b> (&'a self, str: &'b str) -> MatchResult<'a, 'b, World> {
    Cucumber::match_steps(&self.whens, str)
  }

  pub fn match_then<'a,'b> (&'a self, str: &'b str) -> MatchResult<'a, 'b, World> {
    Cucumber::match_steps(&self.thens, str)
  }

  pub fn match_steps<'a,'b> (possible_matches: &'a Vec<(Regex, Step<World>)>, str: &'b str) -> MatchResult<'a, 'b, World> {
    let mut matches: Vec<(Captures, &Step<World>)> =
      possible_matches.iter()
        .filter(|&&(ref regex, _)| regex.is_match(str))
        .map(|&(ref regex, ref step)| (regex.captures(str).unwrap(), step))
        .collect();

    if matches.len() == 0 {
      Err(MatchError::NoMatchingSteps)
    } else if matches.len() > 1 {
      Err(MatchError::SeveralMatchingSteps)
    } else {
      Ok(matches.pop().unwrap())
    }
  }
}

impl <World> CucumberRegistrar<World> for Cucumber<World> {
  fn given(&mut self, regex: Regex, step: Step<World>) {
    self.givens.push((regex, step));
  }

  fn when(&mut self, regex: Regex, step: Step<World>) {
    self.whens.push((regex, step));
  }

  fn then(&mut self, regex: Regex, step: Step<World>) {
    self.thens.push((regex, step));
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use super::helpers::*;
  use std::str::FromStr;

  #[test]
  fn cuke_instantiates() {
    type World = u32;

    let _: Cucumber<World> = Cucumber::new();
  }

  #[test]
  fn cuke_add_steps() {
    type World = u32;

    let mut world = 5;
    let mut cuke: Cucumber<World> = Cucumber::new();

    cuke.given(r("^I do a basic thing$"), Box::new(move |_, _| {}));
    let (capture, step) = cuke.match_given("I do a basic thing").unwrap();
    step(&mut world, capture);
    let (capture, step) = cuke.match_given("I do a basic thing").unwrap();
    step(&mut world, capture);
    let (capture, step) = cuke.match_given("I do a basic thing").unwrap();
    step(&mut world, capture);
    //assert!(cuke.match_given("I do a basic thing").is_ok());
  }

  // For lack of an easy way to test fn equivalence
  #[test]
  fn cuke_execute_steps() {
    type World = u32;
    let mut world = 0;

    let mut cuke: Cucumber<World> = Cucumber::new();

    cuke.given(r("^My call count is (\\d+)$"), Box::new(move |call_count, capture|  {
      let new_call_count = u32::from_str(capture.at(1).unwrap()).unwrap();
      *call_count = new_call_count;
    }));

    let (capture, step) = cuke.match_given("My call count is 10").unwrap();

    assert_eq!(world, 0);
    step(&mut world, capture);
    assert_eq!(world, 10);
  }
}
