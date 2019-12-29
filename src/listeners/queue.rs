use lapin::{
  message::DeliveryResult, options::*, types::FieldTable, BasicProperties, Connection,
  ConnectionProperties,
};

pub struct Queue {
  pub conn: Option<Connection>
}

impl Queue {
  pub fn connect(&mut self, addr: &str) {
    self.conn = Some(Connection::connect(&addr, ConnectionProperties::default())
      .wait()
      .expect("connection error"));
  }
} 
