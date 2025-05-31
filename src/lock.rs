#![allow(unused_imports)]
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

pub fn mutex_function() {
    let mutex = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = mutex.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }

                // If you drop before sleep then the program should take slightly more than a second to complete since the mutex is not blocking the other threads.
                // Essentially all threads will go to sleep for 1 second at about the same time.
                drop(guard);
                // Will now take 10secs to complete the program.
                // thread::sleep(Duration::from_secs(1));
            });
        }
    });

    println!(
        "Final value of the mutext guarded value: {}",
        *mutex.lock().unwrap()
    );
    assert_eq!(mutex.into_inner().unwrap(), 1000);
}

pub fn spawn_thread() {
    let mut file_descriptor = fs::File::open("test.txt").unwrap();
    static I: i32 = 0;
    let mut i = 0i32;
    let numbers = vec![1, 2, 3, 4, 5];

    // i is clonable still can't use it interesting, so even if it is clonable rust doesn't clone it and won't let you borrow it.
    // Use only I and it works since it is static.
    let t1 = thread::spawn(move || {
        let thread_id = thread::current().id();
        println!("Thread 1: {thread_id:?}");
        println!("Static I: {I}");
        println!("Val i: {i}");

        i += 1;
        println!("Val i after increment: {i}");
    });

    let t2 = thread::spawn(thread_fn);

    // spawn takes 'static lifetime as the function argument so anything local being passed is not likely to ge well for you.
    let t3 = thread::spawn(move || {
        // If you konw the exact length of the file you can pre-allocate the buffer, otherwise you risk having 0s at the end of the buffer, which will not
        // play well when you try to convert it to a string.
        let mut buf = Vec::new();

        // You see this is wrong cause file_descriptor does not implement Clone so you got to move its ownership to the thread.
        file_descriptor.read_to_end(&mut buf).unwrap();
        println!("The file read: {}", String::from_utf8(buf).unwrap());

        // Move is required once again because vec is not clonable.
        for n in &numbers {
            print!("{n} ");
        }

        print!("\n");
        println!("{}", &numbers.len());
    });
    println!("Hello, from Main!");

    // Makes sure main does not exit before the threads are finished. t1.join().unwrap();
    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

    // i is still avlaiable here despite moving it into the thread, likely because it is clonable and closure simply clones the value when moving it.
    println!("\nTry i: {}", i);

    // Yup, i works cause it's cloned, this doesn't since it was moved.
    // println!("Numbers available?: {}", numbers.len());
}

pub fn thread_fn() {
    println!("Hello form the thread!");

    let thread_id = thread::current().id();
    println!("Thread ID: {thread_id:?}");
}

pub fn scoped_thread() {
    println!("Scoped One Heh.");
    let numbers = vec![1, 2, 3, 4];

    thread::scope(|s| {
        s.spawn(|| {
            for n in &numbers {
                print!("{n} ");
            }
            print!("\n");
        });

        s.spawn(|| {
            println!("Length: {}", &numbers.len());
        });
    });
}

pub fn reference_counting() {
    let arc_shared = Arc::new([1, 2, 3, 4, 5]);

    // One way to do it. Clone upfornt and share with thread.
    let arc_cloned = arc_shared.clone();
    // This works cause the compiler knows that it needs to move full ownership to the thread, so you don't need to speicify move explicitly.
    // Say the function could take a reference then the compiler would throw an error and tell you to move the thing.
    // let t1 = thread::spawn(|| dbg!(arc_shared));
    let t2 = thread::spawn(|| {
        println!("Reference count cloned: {}", Arc::strong_count(&arc_cloned));
        dbg!(arc_cloned)
    });

    // Can't do this after t1 since it was moved to t1.
    // The other way of doing it. Clone in each thread and have the same name for the varialbe making it more readable.
    let t3 = thread::spawn({
        let arc_shared = arc_shared.clone();
        move || {
            println!(
                "Reference count shared inner: {}",
                Arc::strong_count(&arc_shared)
            );
            dbg!(arc_shared);
        }
    });

    // t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
    println!("Reference count outer: {}", Arc::strong_count(&arc_shared));
}
