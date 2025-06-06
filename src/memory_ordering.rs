use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

static X: AtomicI32 = AtomicI32::new(0);
static DATA: AtomicI32 = AtomicI32::new(0);
static FLAG: AtomicBool = AtomicBool::new(false);
static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);
static mut S: String = String::new();

pub fn spawn_and_join() {
    X.store(1, Ordering::Relaxed);

    let t = thread::spawn(f);

    X.store(2, Ordering::Relaxed);
    t.join().unwrap();

    X.store(3, Ordering::Relaxed);
}

// Always true cause when the thread runs we already stored 1, and then it can be updated to 2, since that is before the thread was joined.
fn f() {
    let x = X.load(Ordering::Relaxed);
    assert!(x == 1 || x == 2);
}

pub fn release_and_acquire() {
    thread::spawn(|| {
        DATA.store(44, Ordering::Relaxed);
        // Makes sure that data store happens before the flag value is set.
        FLAG.store(true, Ordering::Release);
    });

    // Wait for the flag to be set before proceeding. When you see this we can safely assume that the data set has also occurred.
    while !FLAG.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("Waiting on the flag...");
    }

    // After this we're gauranteed to see the value 44 for the DATA.
    assert_eq!(DATA.load(Ordering::Relaxed), 44);
}

// Threads a and b would never access S simultaneously because of sequential consistent ordering, total ordering dictates that the frist opertation will always be
// a store operation on either A or B, gauranteeing that the other will not access while it is doing so.
pub fn sequentially_consistent_ordering() {
    let a = thread::spawn(|| {
        A.store(true, Ordering::SeqCst);
        if !B.load(Ordering::SeqCst) {
            unsafe {
                #[allow(static_mut_refs)]
                S.push('!');
            }
        }
    });

    let b = thread::spawn(|| {
        B.store(true, Ordering::SeqCst);
        if !A.load(Ordering::SeqCst) {
            unsafe {
                #[allow(static_mut_refs)]
                S.push('!');
            }
        }
    });

    a.join().unwrap();
    b.join().unwrap();
}
