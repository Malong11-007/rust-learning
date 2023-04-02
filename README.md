Rust-learning
=============

This repository is used for learning Rust.

1.  Using course - [Rust Fundamentals](https://www.udemy.com/course/rust-fundamentals/)

Concepts
--------

### Ownership

-   Each value in Rust has an owner.
-   When the owner goes out of scope, the value is dropped.
-   Ownership can be transferred using move semantics.

``` rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the String "hello"
    let s2 = s1; // ownership of s1's String is moved to s2
    // s1 can no longer be used, as its value was moved to s2
    println!("{}", s2); // prints "hello"
}
```

### Borrowing

-   Borrowing allows a value to be used without taking ownership.
-   Borrowed values are called references.
-   References can be immutable or mutable.
-   Mutable references are exclusive, meaning you can only have one mutable reference to a value at a time.
-   References have lifetimes, which are the scopes in which they are valid.

``` rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s); // borrowing the reference to s
    println!("The length of '{}' is {}.", s, len); // s is still valid here
}

fn calculate_length(s: &String) -> usize { // borrowing a reference to s
    s.len()
}
```

### Ownership and Borrowing Rules

-   Rust enforces a set of ownership and borrowing rules to prevent common errors.
-   One value can only have one owner at a time.
-   References must always be valid and not outlive the value they are referencing.
-   Mutable references cannot coexist with any other references (mutable or immutable) to the same value.

```rust
fn main() {
    let mut s = String::from("hello");
    change_string(&mut s); // borrowing a mutable reference to s
    println!("{}", s); // prints "goodbye"
}

fn change_string(s: &mut String) { // borrowing a mutable reference to s
    s.push_str(", goodbye");
}

fn main() {
    let mut s = String::from("hello");
    let r1 = &s; // borrowing an immutable reference to s
    let r2 = &s; // borrowing another immutable reference to s
    let r3 = &mut s; // ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable
}
```
