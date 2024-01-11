/*
Problem 1 - Define and Use a Struct
Define a struct named Book with fields title,author, and pages
where title and author are Strings and pages is a u32. Then, in the main functiion,
create an instance of Book and print out its title and number of pages
*/

struct Book {
    title: String,
    author: String,
    pages: u32
}

pub fn p1() {
    let new_book = Book {
        title: String::from("Tao Te Ching"),
        author: String::from("Lao zu"),
        pages: 172 as u32,
    };
    println!("Book title: {}", new_book.title);
    println!("Author: {}", new_book.author);
    println!("Pages: {}", new_book.pages);
}

// -----------------------------------------------------------------------------

/*
Problem 2 - Update Struct
Create a mutable instance of the Book struct from Problem 1.
Change the title of the book and then print the new title
*/

pub fn p2() {
    let mut new_book = Book {
        title: String::from("Tao Te Ching"),
        author: String::from("Lao zu"),
        pages: 172 as u32,
    };
    println!("Old book title: {}", new_book.title);
    new_book.title = String::from("Omniplex");
    println!("New book title: {}", new_book.title);

}