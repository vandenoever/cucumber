use cucumber::definitions::registration::CucumberRegistrar;
use cucumber::response::InvokeResponse;
use support::env::CucumberWorld;

#[allow(dead_code)]
pub fn register_steps(c: &mut CucumberRegistrar<CucumberWorld>) {
  When!(c; "^I add a \"(Given|When|Then)\" step definition with the regex \"([^\"]*)\"$", |_, world: &mut CucumberWorld, (ty, regex): (String, String)| {
    match ty.as_ref() {
      "Given" => Given!(world.matcher; regex.as_str(), |_, _, _| { InvokeResponse::Success }),
      "When" => When!(world.matcher; regex.as_str(), |_, _, _| { InvokeResponse::Success }),
      "Then" => Then!(world.matcher; regex.as_str(), |_, _, _| { InvokeResponse::Success }),
      _ => return InvokeResponse::with_fail_message("Unknown step definition type")
    };

    InvokeResponse::Success
  });
}
