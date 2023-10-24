#![allow(dead_code)]

use destructure::Destructure;

#[derive(Debug, Destructure)]
pub struct Book {
    id: String,
    name: String,
    published_at: String,
    author: Author,
}

#[derive(Debug, Clone)]
pub struct Author(String);

impl Author {
    pub fn try_new(name: impl Into<String>) -> anyhow::Result<Author> {
        let name = name.into();
        if name.is_empty() {
            Err(anyhow::Error::msg("`name` is must not empty."))
        } else {
            Ok(Self(name))
        }
    }
}

#[allow(unused)]
fn main() -> anyhow::Result<()> {
    let book = Book {
        id: "123456789-abc".to_string(),
        name: "name".to_string(),
        published_at: "2023/01/03".to_string(),
        author: Author::try_new("author").unwrap(),
    };

    let book = book.try_reconstruct(|des| -> anyhow::Result<()> {
        des.author = Author::try_new("reirokusanami")?;
        Ok(())
    })?;

    println!("{:?}", book);

    Ok(())
}
