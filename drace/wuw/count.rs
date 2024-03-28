use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

type Shared<T> = Arc<Mutex<T>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    println!("which amount of threads do you want? ");
    io::stdin().read_line(&mut buffer)?;
    let n_threads: usize = buffer.trim().parse()?;
    buffer.clear();
    println!("amount to count to for each thread? ");
    io::stdin().read_line(&mut buffer)?;
    let count_to: usize = buffer.trim().parse()?;
    let mut a  = 0;
    // let mut a: Shared<usize> = Default::default();
    let mut children = vec![];
    for _ in 0..n_threads {
        // let a = a.clone();
        children.push(thread::spawn(move || {
          for _ in 0..count_to {
            a = a + 1;
            // let mut a = a.lock().unwrap();
            // *a = *a + 1;
          }
        }))
    }
    for child in children {
        let _ = child.join();
    }
    println!("we counted to {:?} and expected {}", a, count_to * n_threads);
    Ok(())
}
