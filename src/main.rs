extern crate config;

mod listeners;

use std::collections::HashMap;
use listeners::queue::Queue;
use log::info;

fn main() {
  let mut settings = config::Config::default();
  settings
    .merge(config::File::with_name("config")).unwrap();
  let settings = settings.try_into::<HashMap<String, String>>()
    .unwrap();


  let mut queue = Queue{
    conn: None
  };
  queue.connect(settings.get("AMQP_URL").unwrap());

  info!("Connected to the queue")

}
