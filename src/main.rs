mod listeners;
mod judge;
mod constants;

use std::str;
use listeners::queue::QueueService;
use log::info;
use lapin::message::Delivery;
use judge::{
  run,
  JudgeResult
};

fn main() {
  let mut queue = QueueService{
    conn: None
  };
  queue.connect(&constants::AMQP_URL());

  info!("Connected to the queue");

  queue.subscribe_to_queue("test", &|delivery: &Delivery| {
    let result: JudgeResult = run(str::from_utf8(&delivery.data).unwrap());
    println!("{}", result.stdout);
    println!("{}", result.stderr);
  });
}
