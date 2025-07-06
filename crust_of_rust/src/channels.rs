// Fair warning we're gonig full comment mode for these things. IT IS NOT AI GENERATED. STOP THE SLANDER!!
// I had to write that goofy ass comment. :shrug_emote:
// I'm gonna have this on all files :toll_face_emote:

// Channel is simply a medium through which we can send data from one place and receive it at a different one.
// It is multi producer, single consumer (mpsc).
// There are many to many channes as well, I don't think the standard library has those.

// This is going to use other parts of the sync module. I'll see if the rust atomics book does it any different.

#![allow(unused)]
use std::{
    collections::{VecDeque, vec_deque},
    sync::{Arc, Condvar, Mutex},
};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

// #[derive(Clone)] would be wrong cause it would have the following signature:
// impl<T: Clone> Clone for Sender<T> { ... }
// This is incorrect for our context cause Arc makes the interior value clonable irrespective of it being clone or not, and we want to keep it that way.
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            // Here we explicitly call Arc::clone cause self.inner.clone would be ambigious. Reason being, Arc dereferences to T (the enclosed type) and if that type T is also clone,
            // we're going dark places.
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, data_to_send: T) {
        // You could wrap the next two lines in a block and the queue would be dropped implicitly but, we'd rather be explicit. Makes it easier to read.
        // These things do return Poision struct which tells us if the other thread has panicked we've ignored it for now.
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(data_to_send);
        // We need to drop the queue and its held lock otherwise the other thread would wake up but never get the Mutex, likely a deadlock.
        drop(queue);
        self.inner.signal_data_sent.notify_one();
    }
}

// The Reciever needs to have a mutex despite there being only one consumer is because a send and receive could happen at the same time, which would likely lead to problems.
// To avoid those we have Mutex on both sender and receiver.
pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    pub fn receive(&mut self) -> T {
        // These things do return Poision struct which tells us if the other thread has panicked we've ignored it for now.
        let mut queue = self.inner.queue.lock().unwrap();

        // We loop but it's not a spinlock type. The condvar makes sure that the thread sleeps when we hit None arm.
        // When the receiver signals the thread is woken up again.
        loop {
            match queue.pop_front() {
                // We retun from here so the Mutex is dropped anyways, no need for explicit mention here.
                Some(data) => return data,
                None => {
                    // wait automatically dros the Mutex so the thread that needs to wake this up can acquire the shred resource. Otherwise you guessed it, it's a deadlock.
                    queue = self.inner.signal_data_sent.wait(queue).unwrap();
                }
            }
        }
    }
}

// The Mutex is on inner
struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    signal_data_sent: Condvar,
}

// Its convention to return the Sender first and then the Receiver.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        signal_data_sent: Condvar::new(),
    };
    let inner = Arc::new(inner);

    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut sender, mut receiver) = channel();
        sender.send(42);
        assert_eq!(receiver.receive(), 42);
    }

    // For poorly designed funciton this one hangs.
    #[test]
    fn drop_sender_instantly() {
        // Cause we're not really sending anything and receiving, we need to specify a type explicitly to the channel function.
        let (mut sender, mut receiver) = channel::<()>();
        let _ = sender;
        let _ = receiver.receive();
    }
}
