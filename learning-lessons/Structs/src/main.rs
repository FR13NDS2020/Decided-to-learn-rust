fn main() {
    // basic ecsample of Struct
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let mut user1 = User {// using Struct
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // here is how we change values in some mutable Struct
    user1.email = String::from("anotheremail@example.com");

    // A build_user function that uses field init shorthand because the username and email
    // parameters have the same name as struct fields
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    // Struct Update Syntax
    let user2 = User {//Creating a new User instance using one of the values from user1
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    //Using struct update syntax to set a new email value for a User instance but
    // to use the rest of the values from user2
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    //Using Tuple Structs Without Named Fields to Create Different Types
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


}
//Unit-Like Structs Without Any Fields
struct AlwaysEqual;

fn main2() {
    let subject = AlwaysEqual;
}