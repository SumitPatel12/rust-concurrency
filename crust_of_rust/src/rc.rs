use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    // This is something your would like to search for: https://doc.rust-lang.org/nomicon/dropck.html
    // Essentially it turns out that depending on the order in which you have declared T,
    // and Rc you could end up in a situation where T is dropped before Rc, leading to use-after-free.
    //
    // You can look at the bad test at the bottom, compiler says that x does not live long enough, which is one of the issues.
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });

        // Since Rc is reference counted, we need something that is allocated on the heap and we get a pointer to it. This enables sharing of data.
        // Which leads us to using Box, since it is heap allocated and gives us a pointer
        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        // We use unsafe because the the compiler doesn't konw whether or not the Rc is valid, i.e. it has been deallocated or not.
        let inner = unsafe { &*self.inner.as_ref() };
        let refcount = inner.refcount.get();
        inner.refcount.set(refcount + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away.
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { &*self.inner.as_ref() };
        let count = &inner.refcount.get();

        if *count == 1 {
            // inner is just a reference, no need to explicitly drop it
            // SAFETY: This is the sole reference to the RcInner, meaning we can drop it.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // There are other references so we just decrement the refcount.
            inner.refcount.set(count - 1);
        }
    }
}

// Look deeper into this.
// #[cfg(test)]
// mod test {
//     use crate::crust_of_rust::rc::Rc;

//     #[test]
//     fn bad() {
//         let (y, x);
//         x = String::from("something");
//         y = Rc::new(&x);
//     }
// }
