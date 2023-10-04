fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;// this is called destructuring

    println!("The value of y is: {y}");


    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;// we allso can acces the tuple by index using .

        let one = x.2;
    }
}