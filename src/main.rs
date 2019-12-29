mod listeners;
mod judge;

use std::str;
use listeners::queue::QueueService;
use log::info;
use lapin::message::Delivery;
use judge::{
  run,
  JudgeResult
};
use serde::{
  Serialize,
  Deserialize
};

#[derive(Serialize, Deserialize)]
struct Config {
  AMQP_URL: String
}
impl ::std::default::Default for Config {
  fn default() -> Self { Self { AMQP_URL: "amqp://guest:guest@127.0.0.1:5672/%2f".to_string() } }
}

fn main() {
  let cfg: Config = confy::load("worker").unwrap();

  let mut queue = QueueService{
    conn: None
  };
  queue.connect(&cfg.AMQP_URL);

  info!("Connected to the queue");

  queue.subscribe_to_queue("test", &|delivery: &Delivery| {
    let result: JudgeResult = run(str::from_utf8(&delivery.data).unwrap());
    println!("{}", result.stdout);
    println!("{}", result.stderr);
  });
}
