use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};

#[allow(unused)]
pub struct OneShotChannel<T> {
    // The message to be stored, Option<T> is not used to avoid excess memory overhead.
    message: UnsafeCell<MaybeUninit<T>>,

    // Bool check of whether there is any message to be consumed from the channel.
    ready: AtomicBool,
}

unsafe impl<T> Sync for OneShotChannel<T> where T: Send {}

// Issues with this:
//  1. Sender race conditions, calling send more than once would result in overriding of data even if a receiver is actively trying to consume it.
//  2. Two threads concurrently trying to send, would also result in race conditions.
//  3. Calling the receive more than once can result in the message being copied, even if it was not a Copy type.
//  4. We once again never drop anything, even after our senders and receivers are dropped, the message would not be dropped since MaybeUninit is unsafe and doesn't have it's own checks.
impl<T> OneShotChannel<T> {
    pub fn new() -> Self {
        // Of course the new function returns an unitialized message and an atomic bool set to false.
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    // We've punted the responsibility of making sure the message is there and such to the caller.
    /// SAFETY: Only call this once. Calling it more than once can lead to race conditions.
    pub unsafe fn send(&self, message: T) {
        unsafe { (*self.message.get()).write(message) };

        // Release ordering casue this makes sure that the receiver get's to the message only after it's been initialized.
        self.ready.store(true, Ordering::Release);
    }

    pub fn has_message(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// SAFETY: Only call this once.
    pub unsafe fn receive(&self) -> T {
        unsafe { (*self.message.get()).assume_init_read() }
    }
}
