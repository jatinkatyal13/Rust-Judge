/*
  1. Take JSON payload from queue
  2. parse
  3. create source, stdout, stdin, sterr
  4. compile and run with runguard
  5. read stdout and stderr
  6. return result
*/

use std::process::Command;
use serde::{Deserialize, Serialize};
use base64::{
  decode,
  encode
};
use std::fs::{
  create_dir,
  File
};
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Judging {
    id: i32,
    lang: String,
    source: String,
    stdin: String
}

pub struct JudgeResult {
  pub stdout: String,
  pub stderr: String
}

pub fn run(payload: &str) -> JudgeResult {
  let judging: Judging = serde_json::from_str(payload).unwrap();

  let running_directory = format!("/tmp/run/{}", &judging.id);
  let source = format!("{}/source.{}", &running_directory, &judging.lang);
  let input = format!("{}/stdin", &running_directory);
  let output = format!("{}/stdout", &running_directory);
  let error = format!("{}/stderr", &running_directory);

  create_dir(&running_directory);

  let mut source_file = File::create(&source).unwrap();
  let content = decode(&judging.source).unwrap();
  source_file.write_all(&content);
  
  let mut input_file = File::create(&input).unwrap();
  let content = decode(&judging.stdin).unwrap();
  input_file.write_all(&content);

  // let domjudge user be the owner of running directory
  let _chown = 
    Command::new("chown")
      .arg("-R")
      .arg("domjudge:domjudge")
      .arg(&running_directory)
      .output();
  let _compile_and_run = 
    Command::new("runguard")
      .arg("-u")
      .arg("domjudge")
      .arg("sh")
      .arg(format!("languages/{}-run.sh", judging.lang))
      .arg(&running_directory)
      .output()
      .expect("failed to execute process");
  // ownership back to root
  let _chown = 
    Command::new("chown")
      .arg("-R")
      .arg("$USER:$USER")
      .arg(&running_directory)
      .output();

  let mut result = JudgeResult {
    stdout: String::new(),
    stderr: String::new()
  };

  let mut stdout = String::new();
  let mut stderr = String::new();

  if let Ok(mut output_file) = File::open(&output) {
    output_file.read_to_string(&mut stdout).unwrap();
  } else {
    println!("cannot open output file");
  }

  if let Ok(mut error_file) = File::open(&error) {
    error_file.read_to_string(&mut stderr).unwrap();
  } else {
    println!("cannot open error file");
  }

  // delete run directory
  let _chown = 
    Command::new("rm")
      .arg("-rf")
      .arg(&running_directory)
      .output();

  result.stdout = encode(&stdout);
  result.stderr = encode(&stderr);

  result
}
