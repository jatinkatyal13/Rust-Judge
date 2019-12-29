use std::env;

fn requireFromEnvironment(key: String, default: String) -> String {
  let result = match env::var(key) {
    Ok(val) => val,
    Err(_e) => default.to_string()
  };

  result
}

pub fn AMQP_URL() -> String { 
  requireFromEnvironment("AMQP_URL".to_string(), "amqp://guest:guest@127.0.0.1:5672/%2f".to_string()) 
}
