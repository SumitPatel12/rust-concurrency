use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    thread,
};

pub fn parking() {
    let queue = Mutex::new(VecDeque::new());

    // Consumer
    thread::scope(|s| {
        let t = s.spawn(|| {
            loop {
                let item = queue.lock().unwrap().pop_front();
                if let Some(item) = item {
                    dbg!(item);
                } else {
                    println!("Parking Thread.");
                    thread::park();
                }
            }
        });

        // Producer
        for i in 0.. {
            println!("Producing item {}", i);
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

pub fn condition_variables() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    // Consumer
    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                loop {
                    if let Some(item) = q.pop_front() {
                        dbg!(item);
                    } else {
                        q = not_empty.wait(q).unwrap();
                    }
                }
            }
        });

        // Producer
        for i in 0.. {
            let mut q = queue.lock().unwrap();
            for _ in 0..2 {
                q.push_back(i);
            }
            drop(q);
            not_empty.notify_one();
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
