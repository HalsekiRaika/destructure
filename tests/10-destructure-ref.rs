#![allow(dead_code)]

use destructure::DestructureRef;

#[derive(DestructureRef)]
pub struct Book<T> {
    name: String,
    author: String,
    tags: Vec<T>,
}

#[allow(unused)]
fn main() {
    let book: Book<String> = Book {
        name: "Drive".to_owned(),
        author: "Literally Me".to_owned(),
        tags: Vec::new(),
    };

    let DestructBookRef { name, author, tags } = book.as_destruct();

    drop(book);
}
