fn main() {
    let v: Vec<i32> = Vec::new();// crate a new empty vector
    let v2 = vec![1, 2, 3];// create vector with containing values
    // updating a vector
    let mut a = Vec::new();

    a.push(5);
    a.push(6);
    a.push(7);
    a.push(8);

    // geting values from vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // stroing multiple types in vec using enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


}
