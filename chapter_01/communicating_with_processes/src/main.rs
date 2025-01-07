use std::io;
use std::{io::BufRead, process};

fn main() {
  let pid = process::id();
  println!("process ID: {}", pid);

  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  loop {
    let line = match lines.next() {
      Some(Ok(line)) => line,
      _ => {
        eprintln!("Failed to read from stdin");
        break;
      }
    };
    println!("Received: {}", line);
  }
}
