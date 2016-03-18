use cucumber::definitions::registration::CucumberRegistrar;
use cucumber::response::InvokeResponse;
use cucumber::matcher::Matcher;
use support::env::CucumberWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CucumberWorld>) {
  Given!(c; "^a new cucumber instance$", |_, world: &mut CucumberWorld, _| {
    world.matcher = Matcher::new();
    InvokeResponse::Success
  });

  Then!(c; "^cucumber can find a step matching \"([^\"]*)\"$", |_, world: &mut CucumberWorld, (string,): (String,)| {
    InvokeResponse::check_eq(1, world.matcher.find_match(&string).len())
  });
}
