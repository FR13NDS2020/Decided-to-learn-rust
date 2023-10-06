fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {// here is if let that runs only with Some variable
        println!("The maximum is configured to be {}", max);
    }
    // if let with coin
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}
