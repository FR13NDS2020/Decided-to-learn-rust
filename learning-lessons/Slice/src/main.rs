fn main() {
    println!("Hello, world!");
}
// returning the index of the end of the first word
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// String Slices
fn String_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn return_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];// here we return first word
        }
    }

    &s[..]
}

fn slice_list() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}