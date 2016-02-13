use cucumber::CucumberRegistrar;
use cucumber::helpers::r;
use std::str::FromStr;
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(cuke: &mut CucumberRegistrar<CalculatorWorld>) {
  cuke.when(r("^The calculator is cleared$"), Box::new(move |ref mut world, _| {
    world.calculator.clear()
  }));

  cuke.given(r("^The calculator is clear$"), Box::new(move |ref mut world, _| {
    world.calculator.clear()
  }));

  cuke.when(r("^The number (\\d+) is entered$"), Box::new(move |ref mut world, captures| {
    let capture = u32::from_str(captures.at(1).unwrap()).unwrap();
    world.calculator.enter(capture)
  }));
}
