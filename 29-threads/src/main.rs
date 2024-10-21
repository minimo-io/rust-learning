//! Multi-threading, channels & mutex (shared memory)

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // We use channels to communicate between threads, using mpsc from the standard library
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

    // Another way for threads to communicate with each other is via shared-memory
    // The simplest way is via mutex: mutual exclusion.
    // The mutex is described as guarding the data it holds via the locking system.
    // allows only one thread to access some data at any given time.

    // Here is an example of Mutex being used without threads
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 292;
    }
    println!("m = {m:?}");

    // Now let's create 10 threads that share a value via Atomic Reference Counting
    // counter is inmmutable, but we could get a mutable reference to the value inside
    // meaning Mutex<T> provides interior mutability.
    // We use Mutex<T> to mutate contents inside an Arc<T>.
    let counter = Arc::new(Mutex::new(0));
    // Create a vector for storing the threads handles
    let mut handles = vec![];
    // A range to iterate from 0 to  9  (10 threads)
    for _ in 0..10 {
        // Clone the Reference to the counter (Atomic Referece Counting for thread safefty)
        let counter = Arc::clone(&counter);
        // Let's create the thread
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        // add to handlers
        handles.push(handle);
    }
    // We join each handle to make sure each thread finishes before the main thread ends
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    // Mutex<T> comes with the risk of creating deadlocks
    // These occur when an operation needs to lock two resources and two threads
    // have each acquired one of the locks, causing them to wait for each other forever.
}
