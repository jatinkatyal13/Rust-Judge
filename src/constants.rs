use std::env;

pub fn AMQP_URL() -> String { 
  std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://judge:judge@127.0.0.1:5672/%2f".into())
}
