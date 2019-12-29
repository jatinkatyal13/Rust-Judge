use lapin::{
  options::*, 
  types::FieldTable,
  BasicProperties, 
  message::Delivery,
  Channel,
  Queue,
  Connection,
  ConnectionProperties,
};

pub struct QueueService {
  pub conn: Option<Connection>,
  pub channel: Option<Channel>
}

impl QueueService {
  pub fn connect(&mut self, addr: &str) {
    self.conn = Some(Connection::connect(&addr, ConnectionProperties::default())
      .wait()
      .expect("connection error"));
    self.channel = Some(self.conn.as_ref().unwrap().create_channel()
      .wait()
      .expect("create_channel"));
  }

  pub fn subscribeToQueue(&self, name: &str, cb: &dyn Fn(&Delivery)) {
    let queue = self.channel
      .as_ref()
      .unwrap()
      .queue_declare(
        &name,
        QueueDeclareOptions::default(),
        FieldTable::default(),
      )
      .wait()
      .expect("queue_declare");

    let consumer = self.channel
      .as_ref()
      .unwrap()
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
        self.channel
          .as_ref()
          .unwrap()
          .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
          .wait()
          .expect("basic_ack");
      }
    }
  }
} 
