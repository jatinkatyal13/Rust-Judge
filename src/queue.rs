use lapin::{
  options::*, 
  types::FieldTable,
  message::Delivery,
  Connection,
  ConnectionProperties,
};

pub struct QueueService {
  pub conn: Option<Connection>
}

impl QueueService {
  pub fn connect(&mut self, addr: &str) {
    self.conn = Some(Connection::connect(&addr, ConnectionProperties::default())
      .wait()
      .expect("connection error"));
  }

  pub fn subscribe_to_queue(
    &self, 
    name: &str, 
    cb: &dyn Fn(&Delivery)
  ) {
    let channel = self.conn.as_ref().unwrap().create_channel()
      .wait()
      .expect("create_channel");
    let queue = channel
      .queue_declare(
        &name,
        QueueDeclareOptions::default(),
        FieldTable::default(),
      )
      .wait()
      .expect("queue_declare");

    let consumer = channel
      .basic_consume(
          &queue,
          "my_consumer",
          BasicConsumeOptions::default(),
          FieldTable::default(),
      )
      .wait()
      .expect("basic_consume");

    for delivery in consumer {
      cb(&delivery.as_ref().unwrap());
      if let Ok(delivery) = delivery {
        channel
          .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
          .wait()
          .expect("basic_ack");
      }
    }
  }
} 
