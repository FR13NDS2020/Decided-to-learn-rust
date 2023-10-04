fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression_inside_let() {
    let y = {
        let x = 3;
        x + 1 // we dont write the semicolon in the end to not make that as a statement , we want
        // leave them as expression to return the value
    };

    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {// function with return value
    x + 1// if we add the semicolon we get and error
}

