use std::cell::Cell;
use std::cell::RefCell;

pub fn interior_mutability() {
    pointer_caller();
    cell_caller();
    ref_cell_caller();
}

fn pointer_caller() {
    let a = 1;
    let mut b = 2;
    pointer_things(&a, &mut b);
}

fn pointer_things(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;

    // This of course is not possible since a is a shared reference.
    if before != after {
        println!("This should never happen.");
    } else {
        println!("Expected outcome for the pointer function.");
    }
}

fn cell_caller() {
    let c1 = Cell::new(0i32);
    let c2 = Cell::new(1i32);
    cell_function(&c1, &c2);
}

// Only for single thread, can either copy the value or replace it.
fn cell_function(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    println!("Before: {}, After: {}", before, after);

    // This here is possible cause cell allows interior mutability.
    if before != after {
        println!("Before does not equal after.");
    } else {
        println!("Other outcome for the cell fucntion.");
    }
}

fn ref_cell_caller() {
    let rc1: RefCell<Vec<i32>> = RefCell::new(Vec::new());
    ref_cell_function(rc1);
}

fn ref_cell_function(a: RefCell<Vec<i32>>) {
    let before = a.borrow();
    let after = a.borrow();

    // Results in panic as there can either be one mutable borrow or multiple immutable borrows.
    // let mut mut_borrow = a.borrow_mut();
    // mut_borrow.push(2);
    // let mut mut_borrow2 = a.borrow_mut();
    // mut_borrow2.push(3);

    println!("Before: {:?}\nAfter: {:?}\n", before, after);
}
