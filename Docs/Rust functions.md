 Before continue, there is more examples about the Rust's functions in the `main.rs` file on [functions_lecture](../Source/functions_lecture/src/main.rs).

# Functions syntaxis
The syntaxis of Rust's functions is actually pretty similar to other programming languages. The have the `fn` keyword, followed by parenthesis and the body of the function delimited by curly brackets (`{}`).

```rust
fn main() {
    println!("Hello World!");
}
```

## Function parameters and type hinting
A function can also recieve parameters. They are such a internal variable that the function uses to work with external data (variables out of the actual function).

The _type hinting_ is the action to define the type of the future passed parameters of our function.

```rust
// ❌ This doesn't compile - type hinting missing
fn sum(a, b) {
    a + b
}

// ❌ This won't compile - Rust needs explicit types
fn process(value) -> String {
    value.to_string()
}
```

To avoid compile time erros we have to specify the type of the parameters:

```rust
// This function take two signed 32 bits numbers
// and the returning has to be another signed number
fn sum(a: i32, b: i32) -> i32{
    a + b;
}

// This function take one unsigned 8 bits number
// and returns a bool type value
fn is_adult(age: u8) -> bool {
    age >= 18
}
```

## Returning hinting
We only have to specify the returning type value if it's different then _unit type_ `()`. For example:

```rust
fn greetings (name: String) {
    println!("Hello, {}!", name);
}
```

Also there is no need to use the `return` keyword. If we let the last line  of the function body without and semicolon (`;`) it will return a value implicitly. If you put the semicolon then the line turns into an statement and is necessary the use of the `return` keyword.

There is more information about what is a statement and what is an expression in the [Statemens and Expression](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions) section on the Rust's book.

#rust
