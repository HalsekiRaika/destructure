# Automation of Destructure Pattern
[<img alt="crate.io" src="https://img.shields.io/crates/v/destructure?label=crate.io&logo=rust&style=flat-square">](https://crates.io/crates/destructure)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/destructure?color=6162ff&label=docs.rs&logo=docs.rs&style=flat-square">](https://docs.rs/destructure/0.1.1/destructure/)

`destructure` is a automation library for `destructure pattern`.

### What is `destructure pattern`?
A structure with too many fields makes it hard to call constructors, but it is also hard work to prepare a `Getter/Setter` for each one. There are macros for this purpose, but even so, a large number of macros reduces readability. This is especially true when using `From<T>` Trait.  

So how can this be simplified? It is the technique of "converting all fields to public". 
  
This allows for a simplified representation, as in the following example

```rust
pub struct AuthenticateResponse {
    id: Uuid,
    user_code: String,
    verification_uri: String,
    expires_in: i32,
    message: String,
    ... // too many fields...
}

impl AuthenticateResponse {
    pub fn into_destruct(self) -> DestructAuthenticateResponse {
        DestructAuthenticateResponse {
            id: self.id,
            user_code: self.user_code,
            verification_uri: self.verification_uri,
            expires_in: self.expires_in,
            message: self.message,
            ...
        }
    }
}

pub struct DestructAuthenticateResponse {
    pub id: Uuid,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: i32,
    pub message: String,
    ... // too many fields (All `public`.)...
}

fn main() {
    let res = reqwest::get("http://example.com")
        .send().await.unwrap()
        .json::<AuthenticateResponse>().await.unwrap();
    
    let des = res.into_destruct();

    println!("{:?}", des.id);
}
```
  
There are several problems with this method, the most serious of which is the increase in boilerplate.  
Using the multi-cursor feature of the editor, this can be done by copy-pasting, but it is still a hassle.  

Therefore, I created a *Procedural Macro* that automatically generates structures and methods:

```rust
use destructure::Destructure;

#[derive(Destructure)]
pub struct AuthenticateResponse {
    id: Uuid,
    user_code: String,
    verification_uri: String,
    expires_in: i32,
    message: String,
    ... // too many fields...
}

fn main() {
    let res = reqwest::get("http://example.com")
        .send().await.unwrap()
        .json::<AuthenticateResponse>().await.unwrap();

    // Auto generate
    let des: DestructAuthenticateResponse = res.into_destruct();

    println!("{:?}", des.id);
}
```

## Problem
It is still lacking in functionality, but we will accept PullRequests and Issues if there are any problems.