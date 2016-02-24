pub enum Response {
  StepMatches(StepMatchesResponse),
  Invoke(InvokeResponse),
  BeginScenario,
  EndScenario,
  SnippetText,
}

// ["success", []"]
// ["success", []"]
// ["success", [{"id": "1", "args":[]]
// ["success", [{"id": "1", "args":[{"val": "wired", "pos": 10}]}]]
// https://www.relishapp.com/cucumber/cucumber/docs/wire-protocol/invoke-message
pub enum StepMatchesResponse {
  NoMatch,
  Match(Vec<Step>)
}

struct Step {
  id: String,
  args: Vec<StepArg>
}

struct StepArg {
  val: String,
  pos: u32
}

// ["pending", "I'll do it later"]
// ["success"]
// ["fail", {"message": "The wires are down", "exception": "Some.Foreign.ExceptionType"}]
pub enum InvokeResponse {
  Pending(String),
  Success,
  Fail(FailMessage)
}

struct FailMessage {
  message: String,
  exception: String
}

// ["success"]
// pub struct BeginScenarioResponse;

// ["success"]
// pub struct EndScenarioResponse;

// ["success","foo() bar; baz"]
pub type SnippetTextResponse = String;


