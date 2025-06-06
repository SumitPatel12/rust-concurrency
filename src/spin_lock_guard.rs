use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use crate::spin_lock_guard;

// The lifetime gaurantees that the Guard does not outlive the SpinLock.
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Direct return is safe because the existence of the Guard itself guarantees the the lock is held.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Direct return is safe because the existence of the Guard itself guarantees the the lock is held.
        unsafe { &mut *self.lock.value.get() }
    }
}

// The drop of course makes sure the lock is released when the lifetime of the lock ends.
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

// The easiest implementation is using a bool and using std::hint::spin_loop().
// hint::spin_loop() because it hints the processor of the spin lock scenario allowing it to make optimizations as required.
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock<'a>(&'a self) -> Guard<'a, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        Guard { lock: self }
    }

    /// Safety: The &mut T from lock() must be dropped before unlock() is called.
    /// Make sure not to keep references to any field of T either. It might result in undefined behaviour, and believe me you're not up for that.
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

pub fn spin_lock_test() {
    let spin_lock = std::sync::Arc::new(SpinLock::new(Vec::new()));

    thread::scope(|s| {
        s.spawn({
            let spin_lock = spin_lock.clone();
            move || spin_lock.lock().push(1)
        });

        s.spawn({
            let spin_lock = spin_lock.clone();
            move || {
                let mut g = spin_lock.lock();
                g.push(2);
                g.push(3);
            }
        });
    });

    let g = spin_lock.lock();
    dbg!(g.as_slice());
    assert!(g.as_slice() == [1, 2, 3] || g.as_slice() == [2, 3, 1]);
}
