#![allow(dead_code)]

use std::marker::PhantomData;

#[allow(non_snake_case)]
mod A {
    use destructure::Destructure;

    #[derive(Destructure)]
    pub struct Book {
        id: crate::NumId<Book>,
        name: String,
        published_at: String,
        author: String,
    }

    impl Book {
        pub fn new(id: impl Into<i32>, name: impl Into<String>,
            published_at: impl Into<String>, author: impl Into<String>) -> Self {
                Self { id: crate::NumId::new(id), name: name.into(), published_at: published_at.into(), author: author.into() }
        }
    }
}

pub struct NumId<T> {
    id: i32,
    _mark: PhantomData<T>
}

impl<T> NumId<T> {
    pub fn new(id: impl Into<i32>) -> Self {
        Self { id: id.into(), _mark: PhantomData }
    }
}

#[allow(unused)]
fn main() { 
    let book = A::Book::new(123456789, "name", "2023/01/03", "author");

    let des = book.into_destruct();
}