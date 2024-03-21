// struct in Rust
// struct is a blueprint or creating objects with different properties and behaviors

fn main() {
    let book = Book {
        title: String::from("Serious Crypto"),
        author: String::from("Jean-Philipe"),
        published_year: 2005,
    };

    println!(
        "Book name in {},The author of book is
     {} and published year is {}",
        book.title, book.author, book.published_year
    );

    let mut second_book = Book {
        title: String::from("Serious Crypto"),
        author: String::from("Jean-Philipe"),
        published_year: 2005,
    };

    // changing values of Struct
    second_book.title = String::from("The way of Zen");
    second_book.author = String::from("Alan watts");
    second_book.published_year = 1997;

    println!(
        "Book name in {},The author of book is
     {} and published year is {}",
        second_book.title, second_book.author, second_book.published_year
    );

    // calling function
    let get_book = get_book_data(book);

    for data in get_book {
        println!(" {} ", data);
    }

    let my_book = create_book("The path of zen".to_string(), "Simon".to_string(), 2023);

    println!(" {:?}", my_book);

    let my_tuple = TubleForBooks(
        "Emotional Intelligance".to_string(),
        "Daniel Goleman".to_string(),
        2000,
    );
    println!(
        "Book name in {},The author of book is
     {} and published year is {}",
        my_tuple.0, my_tuple.1, my_tuple.2
    );

    let my_rectangle = Rectangle {
        heigt: 10.5,
        width: 20.5,
    };
    let area = my_rectangle.area();
    println!("The area of Rectangle is{}", area);
}

// struct
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published_year: u32,
}

// using struct in function
fn get_book_data(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let published_year = book.published_year.to_string();

    let data: [String; 3] = [title, author, published_year];
    data
}

// function to return struct
fn create_book(title: String, author: String, published_year: u32) -> Book {
    let book = Book {
        title,
        author,
        published_year,
    };
    book
}

struct TubleForBooks(String, String, u32);

struct Rectangle {
    heigt: f64,
    width: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.heigt * self.width
    }
}
