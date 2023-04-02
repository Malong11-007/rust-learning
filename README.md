Rust-learning
=============

This repository is used for learning Rust.

1.  Using course - [Rust Fundamentals](https://www.udemy.com/course/rust-fundamentals/)

Naming Conventions
------------------
| Type                                               | Example                                                                                                                      |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Snake case for function and variable names         | `let my_variable = 42;`<br>`fn my_function() {}`                                                                             |
| Camel case for struct and enum names               | `struct MyStruct { ... }`<br>`enum MyEnum { ... }`                                                                           |
| Pascal case for trait names                        | `trait MyTrait { ... }`                                                                                                      |
| Upper snake case for constants                     | `const MY_CONSTANT: u32 = 42;`<br>`static MY_STATIC_CONSTANT: &str = "Hello, world!";`                                       |
| Prefixing boolean variables with "is" or "has"     | `let is_ready = true;`<br>`let has_error = false;`                                                                           |
| Using "mut" to indicate mutable variables          | `let mut my_variable = 42;`                                                                                                  |
| Using "ref" and "&" to pass variables by reference | `fn my_function(arg1: &u32, arg2: &str) -> bool { ... }`<br>`let my_variable = 42;`<br>`my_function(&my_variable, "Hello");` |

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

### Enums

-   Enums are a custom data type that allows you to define a set of related values.
-   Each value in an enum is called a variant, and variants can have associated data.
-   Enums are useful for representing different states or types of objects in your code.

### Match Expressions

-   Match expressions are used to compare a value against a set of patterns and execute code based on which pattern matches.
-   Match expressions can be used with any type that implements the `Eq` and `Copy` traits.
-   Match expressions are a common way to handle errors and parse input in Rust.

```rust
match value {
    pattern1 => {
        // code to execute if value matches pattern1
        result1;
    },
    pattern2 if condition => {
        // code to execute if value matches pattern2 and condition is true
        result2;
    },
    pattern3 | pattern4 => {
        // code to execute if value matches either pattern3 or pattern4
        result3;
    },
    _ => {
        // code to execute if value does not match any of the previous patterns
        default_result;
    },
};

enum Fruit {
    Apple,
    Banana,
    Orange,
}

// Use match in combination with enum and other value
fn get_fruit_price(fruit: &Fruit, is_organic: bool) -> u32 {
    match (fruit, is_organic) {
        (Fruit::Apple, true) => 50,
        (Fruit::Apple, false) => 40,
        (Fruit::Banana, true) => 60,
        (Fruit::Banana, false) => 30,
        (Fruit::Orange, _) => 20,
    }
}

fn main() {
    let apple = Fruit::Apple;
    let banana = Fruit::Banana;
    let orange = Fruit::Orange;

    println!("The price of an organic apple is {}.", get_fruit_price(&apple, true));
    println!("The price of a non-organic apple is {}.", get_fruit_price(&apple, false));
    println!("The price of an organic banana is {}.", get_fruit_price(&banana, true));
    println!("The price of a non-organic banana is {}.", get_fruit_price(&banana, false));
    println!("The price of an orange is {}.", get_fruit_price(&orange, true));
}
```