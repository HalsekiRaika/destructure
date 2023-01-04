#![allow(dead_code)]

use destructure::Destructure;

#[derive(Destructure)]
pub struct Book {
    id: String,
    name: String,
    published_at: String,
    author: String,
}

#[allow(unused)]
fn main() { 
    let book = Book {
        id: "123456789-abc".to_string(),
        name: "name".to_string(),
        published_at: "2023/01/03".to_string(),
        author: "author".to_string()
    };

    let des = book.into_destruct();
}