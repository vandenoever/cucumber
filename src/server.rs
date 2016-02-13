use runner::Runner;

use std::sync::{Arc, Mutex};
use std::io::Read;

use hyper::Server as HyperServer;
use hyper::server::Request;
use hyper::server::Response;
use hyper::server::Handler;
use hyper::server::Listening;

#[allow(dead_code)]
pub struct Server<World: Send + Sync> {
  runner: Runner<World>,
  muh_msg: String
}

pub type ServerHandle = Listening;

#[allow(dead_code)]
impl <World: Send + Sync> Server<World> {

  pub fn new(runner: Runner<World>) -> Server<World> {
    Server { runner: runner, muh_msg: "Hullo".to_owned() }
  }

  pub fn start(self, addr: Option<&'static str>) -> ServerHandle
    where World: 'static {
    println!("A server was started");
    let addr = addr.unwrap_or("0.0.0.0:7878");
    let server = Arc::new(Mutex::new(self));
    HyperServer::http(&addr)
      .unwrap()
      .handle(move |mut req: Request, res: Response| {
        let mut body = String::new();
        req.read_to_string(&mut body).unwrap();
        println!("heres a req [{}]", body);
        println!("muh message is, as always, {}", server.lock().unwrap().muh_msg);
        res.send(b"[\"success\", []]").unwrap();
      }).unwrap()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use cucumber::helpers::r;
  use cucumber::CucumberRegistrar;
  use runner::Runner;

  use std::io::Read;

  use hyper::Client;
  use hyper::header::Connection;

  /*
  struct Dookie {
    swirl_count: u32
  }

  impl Dookie {
    fn new() -> Dookie {
      Dookie { swirl_count: 4 }
    }
  }

  #[test]
  fn it_makes_a_server() {
    let server = Server::new(Runner::new(Dookie::new()));
    let mut listener = server.start(Some("0.0.0.0:1234"));
    let _ = listener.close();
  }

  #[test]
  fn it_responds_correctly() {
    let server = Server::new(Runner::new(Dookie::new()));
    let mut listener = server.start(Some("0.0.0.0:22134"));

    let client = Client::new();

    let mut res = client.post("http://0.0.0.0:22134")
        .header(Connection::close())
        .body("show me the hullo!")
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let _ = listener.close();

    assert_eq!(body, "Hullo");
  }

  #[test]
  fn it_handles_steps() {
    let mut runner = Runner::new(Dookie::new());
    runner.when(r("^I match a thing$"), Box::new(move |dookie, _| {
      dookie.swirl_count = dookie.swirl_count + 1;
    }));

    let server = Server::new(runner);
    let mut listener = server.start(Some("0.0.0.0:13523"));

    let client = Client::new();

    let mut res = client.post("http://0.0.0.0:13523")
        .header(Connection::close())
        .body("show me the hullo!")
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let _ = listener.close();

    assert_eq!(body, "Hullo");
  }
  */
}
