#![allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::sync::Arc;
use std::thread::{self};

pub mod atomics;
pub mod interior_mutability;
pub mod lock;
pub mod memory_ordering;
pub mod parking_and_condition_variables;
pub mod spin_lock_guard;
pub mod spin_lock_guard_without_lifetime;

fn main() {
    spin_lock_guard::spin_lock_test();
    spin_lock_guard_without_lifetime::spin_lock_without_lifetime();
}
