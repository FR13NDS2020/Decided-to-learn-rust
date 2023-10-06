enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {// here is advanced way to use enum
    V4(u8, u8, u8, u8),
    V6(String),
}

fn advanced_enum_usage() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
// using structs in enum as data types
struct Ipv4Addr {
    // --snip--
}
struct Ipv6Addr {
    // --snip--
}
enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}


// there is more amounts of types of values

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// there is how we create method for enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
fn use_method_in_enum() {
    let m = Message::Write(String::from("hello"));
    m.call();
}



// -------------------------------------------------

fn main() {
    // creating instances of each variant
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

}

fn route(ip_kind: IpAddrKind) {
    route(IpAddrKind::V4);// and that is how we call the function
    route(IpAddrKind::V6);
}// and this i sfunction that takes any variant of that enum


// Option Enum

enum Option<T> {
    None,
    Some(T),
}
