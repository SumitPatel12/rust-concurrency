use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Exclusive => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(count) => {
                self.refcell.state.set(RefState::Shared(count - 1));
            }
        }
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: No exclusive references have been given out since the state would have been EXCLUSIVE.
        // A Ref is created only in case of shared state which guarantees that no exclusive references have been given out.
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: This one is an exclusive reference so giving out a reference is fine.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: RefMut is created only in case of an exclusive state, meaning that only one reference exists at this time, thus we can safely
        // give out a mutable reference.
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: No exclusive references exist yet. This is the first instance of sharing. Also state would be EXCLUSIVE if we had given out one.
                Some(Ref { refcell: self })
            }
            RefState::Shared(count) => {
                self.state.set(RefState::Shared(count + 1));
                // SAFETY: Once again no exclusive reference exists yet, otherwise the state would have been EXCLUSIVE.
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY: No other references have been given out at this point, so we can safely share a mutable reference.
            // If any reference were given out, we would have the state as either SHARED or EXCLUSIVE.
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub fn test_ref_cell() {
    let cell = RefCell::new(5);
    let cell2 = RefCell::new(10);

    let mut ref_mut = cell.borrow_mut().unwrap();
    *ref_mut = 10;
    let shared_ref = cell.borrow();
    assert_eq!(shared_ref.is_none(), true);
    assert_eq!(*ref_mut, 10);

    let shared_ref1 = cell2.borrow().unwrap();
    let shared_ref2 = cell2.borrow().unwrap();
    let mutable_ref = cell2.borrow_mut();

    assert_eq!(*shared_ref1, 10);
    assert_eq!(*shared_ref2, 10);
    assert_eq!(mutable_ref.is_none(), true);
}
