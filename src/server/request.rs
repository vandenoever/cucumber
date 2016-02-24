use rustc_serialize::{
  Decodable,
  Decoder
};

pub enum Request {
  StepMatches(StepMatchesRequest),
  Invoke(InvokeRequest),
  BeginScenario(BeginScenarioRequest),
  EndScenario(EndScenarioRequest),
  SnippetText(SnippetTextRequest),
}


// ["step_matches", {"name_to_match": "we're all wired"}]
pub struct StepMatchesRequest {
  name_to_match: String
}

// ["invoke", {"id":"1", "args": []}]
// ["invoke", {"id":"1", "args": ["wired"]}]
// ["invoke", {"id":"1", "args": ["we're",[["wired"],["high"],["happy"]]]}]
pub struct InvokeRequest {
  id: String,
  args: Vec<String>,
  table_arg: Option<Table>
}

pub type Table = Vec<Vec<String>>;


// ["begin_scenario"]
// ["begin_scenario", {"tags":["bar","baz","foo"]}]
pub struct BeginScenarioRequest {
  tags: Option<Vec<String>>
}


// ["end_scenario"]]
// ["end_scenario", {"tags":["bar","baz","foo"]}]
pub struct EndScenarioRequest {
  tags: Option<Vec<String>>
}

// ["snippet_text", {"step_keyword": "Given", "multiline_arg_class":"", "step_name":"we're all wired"}]
pub struct SnippetTextRequest {
  step_keyword: String,
  multiline_arg_class: String,
  step_name: String
}
