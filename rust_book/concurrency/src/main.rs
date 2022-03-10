use std::sync::mpsc; // multiple producer, single consumer.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn spawn_demo() {
    let join_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    join_handle.join().unwrap();
}

fn spawn_move_demo() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn channel_demo() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(String::from("hello")).unwrap();
        tx.send(String::from("world")).unwrap();
    });

    println!("Got: {}", rx.recv().unwrap());
    println!("Got: {}", rx.recv().unwrap());
}

fn channel_iter_mp_demo() {
    let (tx, rx) = mpsc::channel();

    // multiple producers by cloning.
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("HI"),
            String::from("FROM"),
            String::from("THE"),
            String::from("THREAD"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    // use receiver as an iterator.
    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_multi_thread() {
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

fn main() {
    spawn_move_demo();
    spawn_demo();

    channel_demo();
    channel_iter_mp_demo();

    mutex_multi_thread();
}
