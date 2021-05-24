use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

// Can't use an Rc across threads (that's what Atomic Reference Count (Arc) is for)
// use std::rc::Rc;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!, vec: {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // This is now an error, since v is moved to the thread:
    // println!("v after threads: {:?}", v);

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Channels:
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // Since val has transfered ownership to the channel,
        // the line below won't compile
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // open a new channel
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi1"),
            String::from("from1"),
            String::from("the1"),
            String::from("thread1"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1500));
        }
    });

    thread::spawn(move || {
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

    for received in rx {
        println!("Got: {}", received);
    }

    // Mutexes
    let m = Mutex::new(5);
    let n = Mutex::new(String::from("test"));
    {
        let mut num = m.lock().unwrap();
        *num = 6;

        let mut s = n.lock().unwrap();
        (*s) = String::from("inside the mutex");
    }

    println!("m = {:?}, n = {:?}", m, n);


    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
