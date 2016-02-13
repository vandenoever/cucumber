extern crate cucumber;

mod step_definitions;
mod support;

use cucumber::{ Runner, Server };

use support::env::CalculatorWorld;
use step_definitions::calculator_steps;
use step_definitions::display_steps;

use std::process::Command;
use std::thread;

fn main() {
  let mut runner = Runner::new(CalculatorWorld::new());

  // Register all steps
  calculator_steps::register_steps(&mut runner);
  display_steps::register_steps(&mut runner);

  /*
  runner.execute_given("The calculator is clear");
  runner.execute_then("The display says 0");
  runner.execute_when("The number 5 is entered");
  runner.execute_then("The display says 5");
  runner.execute_when("The calculator is cleared");
  runner.execute_then("The display says 0");
  */

  let server = Server::new(runner);
  let mut listener = server.start(Some("0.0.0.0:7878"));

  /*
  thread::spawn(move || {
    let output = Command::new("cucumber")
      .output()
      .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stdout));
  }).join();

  let _ = listener.close();
  */
  thread::spawn(move || {
    loop { thread::sleep(std::time::Duration::new(10000,0)); }
  }).join();
}
