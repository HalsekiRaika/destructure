use criterion::{Criterion, criterion_group, criterion_main};
use destructure::{Destructure, Mutation};

#[derive(Debug, Destructure, Mutation)]
pub struct Book {
    id: String,
    name: String,
    published_at: String,
    author: String,
}

impl Default for Book {
    fn default() -> Self {
        Book {
            id: "123456789-abc".to_string(),
            name: "name".to_string(),
            author: "author".to_string(),
            published_at: "2023/01/03".to_string(),
        }
    }
}

#[allow(unused)]
fn destruct(c: &mut Criterion) {
    c.bench_function("destruct", |b| {
        b.iter(|| {
            let mut book = Book::default();
            let mut des = book.into_destruct();
            des.name = "new name".to_string();
            des.author = "reirokusanami".to_string();
            des.published_at = "2023/01/04".to_string();
            book = des.freeze();
        });
    });
}

#[allow(unused)]
fn reconstruct(c: &mut Criterion) {
    c.bench_function("reconstruct", |b| {
        b.iter(|| {
            let mut book = Book::default();
            book = book.reconstruct(|des| {
                des.name = "new name".to_string();
                des.author = "reirokusanami".to_string();
                des.published_at = "2023/01/04".to_string();
            });
        });
    });
}

#[allow(unused)]
fn mutation(c: &mut Criterion) {
    c.bench_function("mutation", |b| {
        b.iter(|| {
            let mut book = Book::default();
            book.substitute(|book| {
                *book.name = "new name".to_string();
                *book.author = "reirokusanami".to_string();
                *book.published_at = "2023/01/04".to_string();
            });
        });
    });
}

criterion_group!(benches, destruct, reconstruct, mutation);
criterion_main!(benches);