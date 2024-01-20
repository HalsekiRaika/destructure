use destructure::{Destructure, Mutation};

#[derive(Debug, Clone, Destructure, Mutation)]
pub struct Domain<A, B> {
    a: A,
    b: B
}

fn main() {
    // no-op, build only
}