_Control Flow_ is the programming language's ability to run some code depending on whether a condition is `true` of `false`. So we _gonna_ do this or that, it's actually a way to make decisions on our programs.
# `if` expressions
Alike other programming languages Rust counts with the `if` and `if..else` conditional structure. With only `if` we can handle one condition at the time, and with `else` the _falsy_ variant of the condition. With `else if` we can handle multiple conditions. For example:

```rust
fn main() {

	// single 'choice'

    let age = 23;

    if age >= 18 {
    println!("You're an adult");
    } else {
    println!("You're a kiddo");
    }

    // multiple 'choices'
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

## Using `if` statements for variable's initialization
We can use the `if` statements as a method to assign values on variables depending on the value of another variable/expression. Example:

```rust

fn user_life_stage (age: i32) -> &str {

    let stage = if age >= 18 {
        "Adult"
    } else if age >= 14 {
        "Teenager"
    } else {
        "Childish"
    };

    // In this case we use the semicolon at the end of the if because we are declaring the value of the "stage" value
    stage
}

```

# Repetition with loops
Loops is the way we can execute a block of code more then once without declaring it several times. Rust will execute any instruction that is inside the loop body and then go to the beginning, until we indicates that must to stop.

## Infinite `loop`
The `loop` instruction will repite it content code forever until we tell it to stop it. We can do this with the keyword `break`. If in any part of the cycle we want to skip a part or the whole loop body we have to use `continue`. Here is an example:

```rust
fn main() {
    let mut counter = 0;

    loop {
     counter += 1;

	    if counter == 5 {
			continue;
	    }

	    if counter == 10 {
		    break;
		}
		println!("The counter value is {}", counter)
	};
}
```

### Returning values from loops
One of the uses of `loop` is retry an operation that we know might fail, like read an input, consume an API, etc. Te result of this operation mostly we'll want storage that somewhere. In order to do this we should write the returned value after the `break` keyword.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

### `loop` labels
I don't understand this. Read more in this [link](https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops).

## Conditional Loops using `while`
In Rust the `while` loop works like in many other languages: the loop continues _while_ a condition is `true`. With this one you must have a _counter_ variable (in Rust have to be mutable) and it has to change in every single loop to make this will be stopped during the program. When the condition is `true` the program call `break`. A simple example:

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

This `while` loop will print the value of `number` while the variable's values IS NOT EQUAL to zero (0), and after that subtract 1 from the `number` variable. When the value of the variable reach 0 and the condition is `true` the `while` loop calls `break` and just stop the execution.

```bash
3!
2!
1!
LIFTOFF!!!
```

## Looping through a collection with `for`
The `for` loop allows us to execute code for each item in a collection of values. For example:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

This will print:

```bash
the value is: 10 // loop 1
the value is: 20 // loop 2
the value is: 30 // loop 3
the value is: 40 // loop 4
the value is: 50 // loop 5
```

Notice that `element` references the value of the item on the `array` (`10`, `20` ...) and not the _index_ (0, 1, ...)

If we want to get the _index_ of the elements in the collection, we have to use a counter variable. This is how it looks with a `while` loop:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

#rust
