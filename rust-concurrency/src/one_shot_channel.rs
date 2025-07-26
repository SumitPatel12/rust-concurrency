use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};

#[allow(unused)]
pub struct OneShotChannel<T> {
    // The message to be stored, Option<T> is not used to avoid excess memory overhead.
    message: UnsafeCell<MaybeUninit<T>>,

    // Indicates whether a message is being sent.
    in_use: AtomicBool,

    // Bool check of whether there is any message to be consumed from the channel.
    ready: AtomicBool,
}

unsafe impl<T> Sync for OneShotChannel<T> where T: Send {}

// There might still be issues with this, I'm just not good enought to see those :shrug:.
impl<T> OneShotChannel<T> {
    pub fn new() -> Self {
        // Of course the new function returns an unitialized message, in_use, and ready are of course set to false.
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    pub fn send(&self, message: T) {
        // SAFETY: Let them continue only if in_use is false, and we atomically set it to true to prevent anyone else from entering.
        if !self.in_use.swap(true, Ordering::Relaxed) {
            panic!("Cannot send more than one message.");
        }

        unsafe { (*self.message.get()).write(message) };

        // Release ordering casue this makes sure that the receiver get's to the message only after it's been initialized.
        self.ready.store(true, Ordering::Release);
    }

    pub fn has_message(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    /// Panics if no message is available, or if the message was already consumed.
    /// TIP: Use `has_message` to first.
    pub fn receive(&self) -> T {
        // Check if the value was true and set it to false afterwards to indicate that we've consumed the message.
        if !self.ready.swap(false, Ordering::Acquire) {
            panic!("No message available.")
        }

        // SAFETY: The above check gaurantees that the message is present.
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for OneShotChannel<T> {
    fn drop(&mut self) {
        // SAFETY: If there was a message waiting to be consumed and all the channeld drops drop the message
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}
