use std::io;

fn main() {

    loop {
        println!("what to convert?");
        println!("write 1 if fahrenheit ---> celsius");
        println!("write 2 if celsius ---> fahrenheit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please write the number");
                continue
            },
        };

        if choice == 1 {
            println!("Write fahrenheit:");
            let mut fahrenheit= String::new();
            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read line");

            let fahrenheit: i32 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please write the number");
                    continue
                },
            };
            let result = fah_to_cel(fahrenheit);
            println!("{fahrenheit}°F to celsius is {result}°C");
            break;
        }else if choice == 2 {
            println!("Write celsius:");
            let mut celsius = String::new();
            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read line");

            let celsius: i32 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please write the number");
                    continue
                },
            };
            let result = cel_to_fah(celsius);
            println!("{celsius}°C to fahrenheit is {result}°F");
            break;
        } else {
            println!("Wrong option, available options (1,2)")
        }

    }

}

fn fah_to_cel(fah: i32) -> i32 {
    (fah -32) * 5/9
}

fn cel_to_fah(cel: i32) -> i32 {
    (cel * 9/5) + 32
}