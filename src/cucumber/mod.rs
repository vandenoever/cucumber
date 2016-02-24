use regex::{Regex, Captures};
use std::collections::HashMap;

pub mod helpers;

pub trait SendableStep<World>: Send + Fn(&mut World, Captures) {}
impl<T, World> SendableStep<World> for T where T: Send + Fn(&mut World, Captures) {}

pub type Step<World> = Box<SendableStep<World, Output=()>>;

pub type StepId = u32;

pub type Match<'a, 'b, World> = (Captures<'b>, &'a Step<World>);

pub type MatchResult<'a, 'b, World: 'a> = Result<Match<'a, 'b, World>, MatchError>;

pub type FindResult = Result<StepId, MatchError>;

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
  step_regexes: Vec<Regex>,
  step_ids: HashMap<String, StepId>,
  steps: HashMap<StepId, Step<World>>
}

impl <World> Cucumber<World> {

  pub fn new() -> Cucumber<World> {
    Cucumber {
      step_regexes: Vec::new(),
      step_ids: HashMap::new(),
      steps: HashMap::new()
    }
  }

  pub fn insert_step(&mut self, regex: Regex, step: Step<World>) {
    let str_rep = regex.as_str().to_owned();
    self.step_regexes.push(regex);

    let this_id = self.step_ids.values().max().map(|res| res + 1).unwrap_or(0);
    // TODO: handle existing str_reps in hash
    self.step_ids.insert(str_rep, this_id.clone());

    self.steps.insert(this_id, step);
  }

  pub fn find_match<'a,'b> (&self, str: &str) -> FindResult {
    let mut matches: Vec<&Regex> =
      self.step_regexes.iter()
        .filter(|ref regex| regex.is_match(str))
        .collect();

    if matches.len() == 0 {
      Err(MatchError::NoMatchingSteps)
    } else if matches.len() > 1 {
      Err(MatchError::SeveralMatchingSteps)
    } else {
      let match_str = matches.pop().unwrap().as_str().to_owned();
      let id = self.step_ids.get(&match_str).unwrap().clone();
      Ok(id)
    }
  }

  pub fn step(&self, id: StepId) -> Option<&Step<World>> {
    self.steps.get(&id)
  }

}

impl <World> CucumberRegistrar<World> for Cucumber<World> {
  fn given(&mut self, regex: Regex, step: Step<World>) {
    self.insert_step(regex, step)
  }

  fn when(&mut self, regex: Regex, step: Step<World>) {
    self.insert_step(regex, step)
  }

  fn then(&mut self, regex: Regex, step: Step<World>) {
    self.insert_step(regex, step)
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
  fn cuke_add_step() {
    type World = u32;
    let mut cuke: Cucumber<World> = Cucumber::new();
    cuke.given(r("^I do a basic thing$"), Box::new(move |_, _| {}));
  }

  #[test]
  fn cuke_find_match() {
    type World = u32;
    let mut cuke: Cucumber<World> = Cucumber::new();
    cuke.given(r("^I do a basic thing$"), Box::new(move |_, _| {}));

    let step_id = cuke.find_match("I do a basic thing");
    assert!(step_id.is_ok());
  }

  #[test]
  fn cuke_get_step() {
    type World = u32;

    let mut cuke: Cucumber<World> = Cucumber::new();
    cuke.given(r("^I do a basic thing$"), Box::new(move |_, _| {}));

    let step_id = cuke.find_match("I do a basic thing").unwrap();

    let step = cuke.step(step_id);
    let mut world = 5;
  }

  /*
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
  */
}
