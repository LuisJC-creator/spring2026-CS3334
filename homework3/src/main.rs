use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut book = File::create(filename).unwrap();
    for i in 0..books.len(){
        writeln!(book, "{}, {}, {}", books[i].title, books[i].author, books[i].year);
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let book = File::open(filename).unwrap();
    let wrapped_book = BufReader::new(book);
    let mut book_vec: Vec<Book> = Vec::new();
    for line in wrapped_book.lines(){
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        book_vec.push(Book { title: parts[0].to_string(), author: parts[1].to_string(), year: parts[2].parse::<u16>().unwrap() }); // i love rust, trying to figure this out was so fun :)

    }
    book_vec
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}