/*
use cucumber::{ Step, Cucumber, MatchError, CucumberRegistrar };
use regex::Regex;

pub use cucumber::helpers::r;

pub type ExecuteResult = Result<(), ExecuteError>;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum ExecuteError {
  ExecuteFailure,
  MatchFailure(MatchError)
}

#[allow(dead_code)]
pub struct Runner<World> {
  cuke: Cucumber<World>,
  world: World
}

#[allow(dead_code)]
impl <World> Runner<World> {
  pub fn new(world: World) -> Runner<World> {
    Runner {
      cuke: Cucumber::new(),
      world: world
    }
  }

  pub fn world(&mut self) -> &mut World {
    &mut self.world
  }

  pub fn cuke(&mut self) -> &mut Cucumber<World> {
    &mut self.cuke
  }

  pub fn execute_given(&mut self, str: &str) -> ExecuteResult {
    match self.cuke.match_given(str) {
      Ok((captures, step)) => {
        step(&mut self.world, captures);
        Ok(())
      },
      Err(error) => {
        Err(ExecuteError::MatchFailure(error))
      }
    }
  }

  pub fn execute_when(&mut self, str: &str) -> ExecuteResult {
    match self.cuke.match_when(str) {
      Ok((captures, step)) => {
        step(&mut self.world, captures);
        Ok(())
      },
      Err(error) => {
        Err(ExecuteError::MatchFailure(error))
      }
    }
  }

  pub fn execute_then(&mut self, str: &str) -> ExecuteResult {
    match self.cuke.match_then(str) {
      Ok((captures, step)) => {
        step(&mut self.world, captures);
        Ok(())
      },
      Err(error) => {
        Err(ExecuteError::MatchFailure(error))
      }
    }
  }
}

impl <World> CucumberRegistrar<World> for Runner<World> {
  fn given(&mut self, regex: Regex, step: Step<World>) {
    self.cuke.given(regex, step)
  }

  fn when(&mut self, regex: Regex, step: Step<World>) {
    self.cuke.when(regex, step)
  }

  fn then(&mut self, regex: Regex, step: Step<World>) {
    self.cuke.then(regex, step)
  }
}


#[cfg(test)]
mod test {
  use super::*;
  use cucumber::MatchError;
  use cucumber::CucumberRegistrar;

  #[test]
  fn runner_instantiates() {
    let _: Runner<u32> = Runner::new(0);
  }

  #[test]
  fn runner_executes_steps() {
    let world: u32 = 0;
    let mut runner = Runner::new(world);

    runner.when(r("^I increment my world$"), Box::new(move |world, _| {
      *world = *world + 1
    }));

    let result = runner.execute_when("I increment my world");

    assert!(result.is_ok());
    assert_eq!(*runner.world(), 1);
  }

  /* TODO
  #[test]
  fn runner_fails_when_step_fails() {
  }
  */

  #[test]
  fn runner_fails_when_ambiguous() {
    let world: u32 = 0;
    let mut runner = Runner::new(world);

    runner.when(r("^I match"), Box::new(move |_, _| {}));
    runner.when(r("two things$"), Box::new(move |_, _| {}));

    let result = runner.execute_when("I match two things");

    assert_eq!(result, Err(ExecuteError::MatchFailure(MatchError::SeveralMatchingSteps)));
  }

  #[test]
  fn runner_fails_when_no_matches() {
    let world: u32 = 0;
    let mut runner = Runner::new(world);

    let result = runner.execute_when("I match no things");

    assert_eq!(result, Err(ExecuteError::MatchFailure(MatchError::NoMatchingSteps)));
  }
}
*/
