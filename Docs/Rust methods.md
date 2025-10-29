# Standard Libraries
By default Rust has a set of items defined on it's standard library, to use them  we have to bring them to the current _scope_ of every program. This is called _prelude_, if a type we want to use isn't in the prelude, we have to bring that type into the scope explicitly using an `use` statement.

```rust
// Examples
use std::io;
use rand::Rng;

fn main() {
	// -- your program --
}
```
# std::io
the `std::io` library provide us a lot of useful futures. That includes the ability to accept user inputs.

### io::stdin()
The `stdin()` function represents a handle standard input for the terminal. To actually get the input from the user, we have to use the method `read_line(mut String)` fo the `stdin` function.

`read_line()` method take whatever the user types into standard input and append that into a string (it doesn't overwrite the content), so we have to pass that string as an argument. 

To handle error, we use the method `expect()`. The method `read_line()` return a `Result` value, and it can have multiple states. The instance of `Result` have the `expect()` method. If the `read_line` method returns an `Err`, it would likely be the result of an error coming from the underlying operating system. If this instance of `Result` is an `Ok` value, `expect` will take the return value that `Ok` is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.

If you don’t call `expect`, the program will compile, but you’ll get a warning.

So, the line to get an user's input is:

```rust
let mut name = String::new();

io::stdin().read_line(&mut name).expect("Failed to read line");
// We have to pass the name variables as a mutable to change it's value
// and wth the '&' keyword to indicate that it's a variable's reference
```
``
# Type methods

## String::new
The method `::new` it's the way we can create a new _empty_ `String`. 

## rand::thread_rng().gen_range(start..=end)
The rand library showed as above, allows us generate a random integer between two (`star` - `end`). We can create a variable and assign it a random _number_ value:

```rust
let random_num: i32 = rand::thread_rng().gen_range(1..=100);
// this line will storage a random number on the variable "random_num"
// between 1 and 100; the "=" is to include the end number, 100 in this case
```

## .expect() method
The `.expected()` help us to find what causes the error on our code. Several functions/macros includes this method. 

This method return two posible types: `Ok` and `Err`. The first one is returned when the operation is correctly completed, and hold the actual value from the used function. The `Err` type hold error's information. For example, if we go to the [types_lecture main.rs file](../Source/types_lecture/src/main.rs), we'll see:

```rust
let age: u16 = age.trim().parse().expect("Error at conversion");
```

With this line we use the method `.expect()` on `.parse()`. If `age` is a _number_ alike values like `"33"`, it will be converted into a `u16` type successfully, the `Ok` type in `.expect()` would return the `33`. If the value isn't _parseble_ will return the `Err` type and look like this in runtime:

```rust
========= booleans
Enter your age
xs

thread 'main' panicked at src\main.rs:63:39:
Error at conversion: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\functions_lecture.exe` (exit code: 101)
```

#rust