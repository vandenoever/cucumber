use cucumber::CucumberRegistrar;
use cucumber::helpers::r;
use std::str::FromStr;
use support::env::CalculatorWorld;

#[allow(dead_code)]
pub fn register_steps(cuke: &mut CucumberRegistrar<CalculatorWorld>) {

  cuke.then(r("^The display says (\\d+)"), Box::new(move |ref mut world, captures| {
    let displayed_value = world.calculator.display_contents();
    let capture = i32::from_str(captures.at(1).unwrap()).unwrap();

    assert_eq!(displayed_value, capture);
  }));

}
