# Variables
The variables on Rust can be declared with the keyword `let`. The name's convention is the same that in other languages. No numbers at the variable's name beginning, and it doesn't allows the use of special characters on it. Example:

```rust
let name = "Andrés";
```

## Mutability
By default, all variables on Rust are inmutables, that mean we can't change their value during the program. In order to change this, we have to use the `mut` keyword at the declaration moment. For example:

 ```rust
 let mut age: u32 = 22;
 ```

Now we can change the value of `age` later.

```rust
let mut age: u32 = 22;
age = 27;
```

# Constants
The constants in Rust are inmutables, and their values can't change during the program. Other difference between constants and variables are how you can declare them: using the keyword `const` and you should specify the type. 

## Declaring constants
The actual name convention for variables are all the letters in `UPPERCASE` and using underscores (`_`) as separator. For example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

The constants can be declare in any scope (global or within a function), and their values HAS to be know, that means the value of a constant cannot be the result of a calculation in run time. How to **NOT** declare a constant:

```rust
const USER_COUNT: u32 = get_user_count(); // NOT ALLOWED
```

At the practice, you'll be able to access the constants at any point off your program, that makes them useful to values that never changes like `MAX_POINTS`, `SPEED_OF_LIGHT` or limits/configurations. At the same time, if you need to change their value you only have to do it in one place. 

# Shadowing
In Rust we can declare a variable, use it, and then later declare _another_ variable with the same name. With this action, the first variable was _shadowed_ by the second. This means the compiler will actually use the second one that overshadows the first. This will happen until the new variable get shadowed it self or the scope ends. In order to do this, we have to declare the variable again with the `let` keyword. By example:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

The print:

```
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

## Shadowing or `mut`?
Although they may seem similar, `mut` doesn't allow us to change the type of a variable at compile time. So, if the value change involves a type change (without using the `let` keyword), we'll get an error and the program won't compile.

We are NOT reassigning a value — we are actually creating a new variable while reusing an existing name, and the _old_ variable is _shadowed_, so it's not longer accesible in that scope.

#rust