extern crate config;

mod listeners;
mod judge;

use std::{
  str,
  collections::HashMap
};
use listeners::queue::QueueService;
use log::info;
use lapin::message::Delivery;
use judge::{
  run,
  JudgeResult
};

fn main() {
  let mut settings = config::Config::default();
  settings
    .merge(config::File::with_name("config")).unwrap();
  let settings = settings.try_into::<HashMap<String, String>>()
    .unwrap();


  let mut queue = QueueService{
    conn: None
  };
  queue.connect(settings.get("AMQP_URL").unwrap());

  info!("Connected to the queue");

  queue.subscribe_to_queue("test", &|delivery: &Delivery| {
    let result: JudgeResult = run(str::from_utf8(&delivery.data).unwrap());
    println!("{}", result.stdout);
    println!("{}", result.stderr);
  });
}
