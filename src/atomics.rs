use std::{
    i32,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, AtomicUsize},
    },
    thread,
    time::Duration,
};

pub fn stop_flag() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = std::thread::spawn(|| {
        while !STOP.load(std::sync::atomic::Ordering::Relaxed) {
            println!("Doing some work and going to sleep for 1 sec.");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("Available commands: help, stop"),
            "stop" => {
                STOP.store(true, std::sync::atomic::Ordering::Relaxed);
                break;
            }
            _ => println!("Unknown command"),
        }
    }

    background_thread.join().unwrap();
}

pub fn progress_reporting() {
    let num_done = Arc::new(AtomicUsize::new(0));

    let t = thread::spawn({
        let num_done = num_done.clone();
        move || {
            for i in 0..10 {
                println!("Putting thread to sleep {i}");
                thread::sleep(Duration::from_secs(1));
                num_done.store(i + 1, std::sync::atomic::Ordering::Relaxed);
            }
        }
    });

    loop {
        let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
        if n == 10 {
            break;
        }

        println!("Working: {n} / 100");
        thread::sleep(Duration::from_millis(1100));
    }

    t.join().unwrap();

    println!("Huh, I'm Done.");
}

pub fn lazy_initialization() {
    println!("{}", get_x());
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(std::sync::atomic::Ordering::Relaxed);

    if x == 0 {
        x = calculate_x();
        X.store(x, std::sync::atomic::Ordering::Relaxed);
    }
    x
}

fn calculate_x() -> u64 {
    // Simulate some work
    thread::sleep(Duration::from_millis(500));
    42
}

pub fn fetch_add() {
    let a = AtomicI32::new(0);
    let b = a.fetch_add(23, std::sync::atomic::Ordering::Relaxed);
    let c = a.load(std::sync::atomic::Ordering::Relaxed);
    let atomic_i32_max = AtomicI32::new(i32::MAX);
    let _ = atomic_i32_max.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let atomic_i32_min = atomic_i32_max.load(std::sync::atomic::Ordering::Relaxed);

    assert_eq!(b, 0);
    assert_eq!(c, 23);
    // The add operation wraps unlike with pure i32 where it panics.
    assert_eq!(atomic_i32_min, i32::MIN);
}

pub fn multiple_thread_progress_report() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // Spawn progress reporter here cause thread scope blocks further execution of the main thread until all of the threads its has spawned
        // are completed, essentially leading to no progress reporting if the main thread loops after the scope.
        // Since process_items is blocking we are spawning a new thread to report progress. Otherwise the scope thread is a good place to handle the progress reporting loop.
        s.spawn(|| {
            loop {
                let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
                println!("Progress: {}%", n);
                if n == 100 {
                    break;
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        for thread in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(thread * 25 + i);
                    num_done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
    });

    // Or you can delegate the main thread to do it in the thread itself which I think is a better idea than spawning it as its own thread.
    // This only works if the process items does not put the threads to sleep, cause if it does then the loop will never be called.
    // loop {
    //     let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
    //     println!("Progress: {}%", n);
    //     if n == 100 {
    //         break;
    //     }
    //     thread::sleep(Duration::from_secs(1));
    // }
}

fn process_item(item: i32) {
    println!("Processing item {}", item);
    thread::sleep(Duration::from_millis(500));
}

// Uses compare and exchange to atomically allocate IDs and checks for max values so we do not encounter overflows.
pub fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(std::sync::atomic::Ordering::Relaxed);

    loop {
        assert!(id < 1000);
        match NEXT_ID.compare_exchange(
            id,
            id + 1,
            std::sync::atomic::Ordering::Relaxed,
            std::sync::atomic::Ordering::Relaxed,
        ) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}
