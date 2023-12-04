use destructure::Mutation;

#[derive(Debug, Mutation)]
pub struct Book {
    id: String,
    name: String,
    published_at: String,
    author: String,
}

#[allow(unused)]
fn main() {
    let mut book = Book {
        id: "123456789-abc".to_string(),
        name: "name".to_string(),
        author: "author".to_string(),
        published_at: "2023/01/03".to_string(),
    };

    book.substitute(|book| {
        *book.name = "new name".to_string();
        *book.author = "reirokusanami".to_string();
        *book.published_at = "2023/01/04".to_string();
    });

    println!("{:?}", book)
}
