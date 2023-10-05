
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);// These ampersands represent references, and they
    // allow you to refer to some value without taking ownership of it

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn main2() {
    let mut s = String::from("hello");

    change(&mut s);// make barrowed data mutable
}

fn change(some_string: &mut String) {// make barrowed data mutable to edit them
    some_string.push_str(", world");
}
// this code will fail because we cant have other reference to the same value
// error[E0499]: cannot borrow `s` as mutable more than once at a time
// let mut s = String::from("hello");
//
// let r1 = &mut s;
// let r2 = &mut s;
//
// println!("{}, {}", r1, r2);
fn immut_refs() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// Dangling References
//dangling pointer—a pointer that references a location in memory that may have been given to
// someone else—by freeing some memory while preserving a pointer to that memory
