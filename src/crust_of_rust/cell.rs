use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// Implied by UnsafeCell
// impl !Sync for Cell<T> {}

// Rudimentary Implementation of Cell
impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    pub fn set(&self, value: T) {
        // SAFETY: No one-else concurrently mutates this Cell because it is not Sync.
        // SAFETY: Once again we are not invalidating any references to the value since we never give one out.
        unsafe { *self.value.get() = value };
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    // fn incorrect() {
    //     use std::sync::Arc;

    //     let x = Arc::new(Cell::new(43));

    //     let x1 = Arc::clone(&x);
    //     // Incorrect Because UnsafeCell is not Sync and by extension our struct Cell is also not Sync.
    //     std::thread::spawn(move || {
    //         x1.set(42);
    //     });

    //     let x2 = Arc::clone(&x);
    //     std::thread::spawn(move || {
    //         x2.set(44);
    //     });
    // }
}
