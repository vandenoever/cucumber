use cucumber::matcher::Matcher;

pub struct CucumberWorld {
  pub matcher: Matcher<u32>
}

impl CucumberWorld {
  pub fn new() -> CucumberWorld {
    CucumberWorld { matcher: Matcher::new() }
  }
}
