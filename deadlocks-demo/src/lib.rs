//! # Create deadlock with Rust
//! If you’re interested in deadlocks, try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for mutexes in any language and have a go at implementing them in Rust. The standard library API documentation for Mutex<T> and MutexGuard offers useful information.
//!

/// The exact behavior on locking a mutex in the thread which already holds the lock is left unspecified. However, this function will not return on the second call (it might panic or deadlock, for example).
///
/// # Examples
///
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn _demo() {
    let data = Mutex::new(0);
    let _d1 = data.lock();
    let _d2 = data.lock(); // cannot lock, since d1 is still active
}

fn _demo2() {
    // Make a mutex-guarded value.
    let m = Arc::new(Mutex::new(0));
    // Duplicate the mutex reference for sending to the child.
    let mt = Arc::clone(&m);

    // Lock the mutex and get the guard; the mutex will
    // stay locked until almost the end of this block.
    let g = m.lock().unwrap();

    // Spawn a new thread that can only exit after *it*
    // locks the mutex and returns the guarded value.
    let t = thread::spawn(move || {
        let g = mt.lock().unwrap();
        *g
    });

    // Wait for the thread to finish. It cannot, because it
    // is waiting for us to drop `g` so that it can lock `M`.
    // Deadlock has been achieved.
    let n = t.join().unwrap() + *g;
    println!("{}", n);
}
