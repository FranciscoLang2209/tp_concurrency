use std::thread;
use std::time::Duration;

fn hello() {
    thread::scope(|s| {
        s.spawn(|| {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from thread 1")
        });
        s.spawn(|| {
            thread::sleep(Duration::from_secs(2));
            println!("Hello from thread 2")
        });
    });
}

fn main() {
    hello();

    thread::sleep(Duration::from_secs(1));
}