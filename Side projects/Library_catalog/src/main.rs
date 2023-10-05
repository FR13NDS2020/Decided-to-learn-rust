use std::io;
use rand::Rng;




#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    borrowed: bool,
    number: u32,
}
#[derive(Debug)]
struct User {
    name: String,
    borrowed_book: (String),
}


struct Library {
    books: Vec<Book>,
    users: Vec<User>,
}


impl Book {
    fn new(title: String, author: String) -> Book {
        let number = rand::thread_rng().gen_range(1..=1000);
        Book {
            title,
            author,
            borrowed: false,
            number,
        }
    }
    fn borrow(&mut self) {
        self.borrowed = true;

    }
    fn return_book(&mut self) {
        self.borrowed = false;
    }
}

impl User {
    fn login_user(name: String) -> User {
        User {
            name,
            borrowed_book: String::new(),
        }
    }
    fn borrow_book(&mut self, book: &mut Book) {
        &book.borrow();
        let book_title = &book.title;
        self.adding_personal_book(book_title)
    }
    fn adding_personal_book(&mut self, book: &String) {
        self.borrowed_book = String::from(book);
    }
}

impl Library {
    fn list_books(&self, book: &Book) {

        let id: usize = 0;
        id + 1;
        println!("Number: {:?}, Title: {:?} , Author: {:?}", book.number ,book.title, book.author)


    }



    fn get_all_users(&mut self) -> &mut Vec<User> {
        &mut self.users
    }

    fn get_all_books(&self) -> &Vec<Book> {
        let books = &self.books;
        return books;
        // &self.books
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
        // println!("Book: {:?} added to library" ,book.title)
    }
}
fn main() {
    println!("Library Catalog");
    println!("1. List all available books");
    println!("2. Login into the library");
    println!("3. List borrowed books for a user");
    println!("4. Borrow a book");
    println!("5. Return a book");
    println!("6. Quit");

    let mut default_book = Book::new(String::from("The Great Gatsby"), String::from("F. Scott Fitzgerald"));

    let mut library1 = Library {
        books: vec![],
        users: vec![],
    };

    loop {
        let choice: u32 = read_user_input();

        match choice {
            1 => {
                library1.list_books(&default_book)
            }
            2 => {//Login into the library
                println!("Please print your name:");
                let username = get_user_name();
                let user = User {
                    name: username,
                    borrowed_book: String::new(),
                };
                library1.add_user(user)
            }
            3 => {//List borrowed books for a user
                println!("Please print your name:");
                let username = get_user_name();
                list_borrowed_books(&mut library1, username)
            }
            4 => {// borrow a book
                println!("print book number:");
                library1.list_books(&default_book);
                let selected_book = read_user_input();

                println!("on which name to borrow book?");
                let mut all_users = library1.get_all_users();
                let mut username = get_user_name();
                for mut user in all_users {
                    if username == user.name {
                        user.borrow_book(&mut default_book);

                    }
                }
            }
            5 => {
                println!("thanks for returning book");
                default_book.return_book()

            }
            6 => {
                println!("Bye");
                break
            }
            _ => {
                println!("write correct choice");
            }
        }

        println!("1. List all available books");
        println!("2. Login into the library");
        println!("3. List borrowed books for a user");
        println!("4. Borrow a book");
        println!("5. Return a book");
        println!("6. Quit");
    }

}



fn list_borrowed_books(library: &mut Library, name: String) {
    let users = library.get_all_users();
    for user in users {
        println!("name: {:?} books: {:?}", user.name, user.borrowed_book)
    }
}

fn get_user_name() -> String {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        return input
    }

}

fn read_user_input() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // convert stdin into the number
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return input
    }

}
