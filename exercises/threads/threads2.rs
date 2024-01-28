// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // needed mutex to make it thread safe
    // ﻿JobStatus isn't wrapped in a type that allows for interior mutability and also being shared among threads, like ﻿std::sync::Mutex.
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);

        // https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status_shared = status_shared.lock().unwrap();
            
            (*status_shared).jobs_completed += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Printed outside the loop
    // .unwrap() is general syntax to return the Ok value or error
    println!("jobs completed {}", (*status.lock().unwrap()).jobs_completed);
}

