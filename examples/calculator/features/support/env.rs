pub struct CalculatorWorld {
  pub calculator: Calculator
}

impl CalculatorWorld {
  pub fn new() -> CalculatorWorld {
    CalculatorWorld { calculator: Calculator::new() }
  }
}

pub struct Calculator {
  displayed_value: i32
}

impl Calculator {
  pub fn new() -> Calculator {
    Calculator { displayed_value: 0 }
  }

  pub fn clear(&mut self) {
    self.displayed_value = 0;
  }

  pub fn enter(&mut self, number: u32) {
    self.displayed_value = number as i32;
  }

  pub fn display_contents(&self) -> i32 {
    self.displayed_value as i32
  }
}

