mod queue;
mod judge;
mod constants;

use std::str;
use queue::QueueService;
use log::info;
use lapin::{
  message::Delivery,
  options::*,
  types::FieldTable,
  BasicProperties
};
use judge::{
  run,
  JudgeResult
};

fn main() {
  let mut queue = QueueService{
    conn: None
  };
  let uri = constants::AMQP_URL();
  queue.connect(&uri);

  info!("Connected to the queue");

  let subscriber_channel = queue.conn.as_ref().unwrap().create_channel()
    .wait()
    .expect("create_channel");
  let publisher_channel = queue.conn.as_ref().unwrap().create_channel()
    .wait()
    .expect("create_channel");
  publisher_channel
    .queue_declare(
      "success",
      QueueDeclareOptions::default(),
      FieldTable::default(),
    )
    .wait()
    .expect("queue_declare");

  queue.subscribe_to_queue(&subscriber_channel, "test", &|delivery: &Delivery| {
    let result: JudgeResult = run(str::from_utf8(&delivery.data).unwrap());
    let result = serde_json::to_string(&result).unwrap();
    publisher_channel
      .basic_publish(
        "",
        "success",
        BasicPublishOptions::default(),
        result.into(),
        BasicProperties::default(),
      )
      .wait()
      .expect("basic_publish");
  });
}
