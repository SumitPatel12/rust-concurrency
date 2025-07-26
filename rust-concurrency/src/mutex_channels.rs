use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

// The potential problems with this one:
//    1. No size limit on VecDeq.
//    2. Sending and receiving are blocking, meaning that when the size of the queue needs to grow, all the other threads will be blocked for a significant amount of time.
//    3. If all the senders are dropped there might be receivers wistfully waiting for a message that would never come.
#[allow(unused)]
pub struct MutexChannel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> MutexChannel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    // Simply acquire the lock and push the message to the back of the queue.
    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    // Get the lock on the queue, and check if there is a message to be consumed.
    // If there are no messages wait for the sender's signal.
    pub fn receive(&self) -> T {
        let mut message_queue = self.queue.lock().unwrap();

        loop {
            if let Some(message) = message_queue.pop_front() {
                return message;
            }
            message_queue = self.item_ready.wait(message_queue).unwrap();
        }
    }
}
