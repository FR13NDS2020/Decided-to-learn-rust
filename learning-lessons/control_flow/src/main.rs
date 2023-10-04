fn main() {
    println!("Hello, world!");
    if_exp();
    loops();
}

// if expression

fn if_exp() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }
    // using else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in let
    let condition = true;
    let number = if condition { 5 } else { 6 };// we can use only similar types, cant use { 6 } as { "six" }
    println!("The value of number is: {number}");

}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;// can write anything after break to return that value after the loop
        }
    };

    println!("The result is {result}");


    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;// here we are braking the first loop not the inner.
                // as if we just write break
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // condition loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // runing for loop only wanted times
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}