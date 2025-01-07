use std::thread;

fn fibonacci(n: u64) -> u64 {
  match n {
    0 | 1 => n,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

fn main() {
  let mut threads = Vec::new();

  for i in 0..4 {
    let handle = thread::spawn(move || {
      let result = fibonacci(4000);
      println!("Thread {} result: {}", i, result);
    });
    threads.push(handle);
  }

  for handle in threads {
    handle.join().unwrap();
  }
}
