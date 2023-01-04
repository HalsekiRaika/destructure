#![allow(dead_code)]

use destructure::Destructure;

#[derive(Destructure)]
pub struct Book {
    id: String,
    name: String,
    published_at: String,
    author: String,
}

fn main() { /* no-op */ }