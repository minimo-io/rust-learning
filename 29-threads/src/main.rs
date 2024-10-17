//! Multi-threading and channels

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // we use channels to communicate between threads, using mpsc from the standard library
    // it returns a tuple (that we de-construct) with a transmitter (tx) and receiver(rx)

    let (tx, rx) = mpsc::channel();

    // We could also clone the transmitter and use it to also transmit to the receiver
    // let other_transmitter = tx.clone();
    // This is alloweed and it is why "mpsc" stands for "multiple producer, single consumer"
    // Then we would create another threaad that takes ownership of this new transmitter

    let handle = thread::spawn(move || {
        // Sending multiple values from the main thread
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // attemos to receive or fail if the channel hangs up
    for received in rx {
        println!("Got: {}", received);
    }
}
