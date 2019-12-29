extern crate config;

mod listeners;

use std::collections::HashMap;
use listeners::queue::QueueService;
use log::info;
use lapin::message::Delivery;

fn main() {
  let mut settings = config::Config::default();
  settings
    .merge(config::File::with_name("config")).unwrap();
  let settings = settings.try_into::<HashMap<String, String>>()
    .unwrap();


  let mut queue = QueueService{
    conn: None,
    channel: None
  };
  queue.connect(settings.get("AMQP_URL").unwrap());

  info!("Connected to the queue");

  let consumer = queue.subscribeToQueue("test", &|delivery: &Delivery| {
    println!("received message: {:?}", delivery);
  });
}
