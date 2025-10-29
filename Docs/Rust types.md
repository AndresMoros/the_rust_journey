 Before continue, there is more examples about the Rust's types on the `main.rs` file on [types_lecture](../Source/types_lecture/src/main.rs).
# Compound Types
The _compound type_ is capable of hold multiple values into one type, basically they can storage some kind of _collection_ of elements. Rust have `tuples` and `arrays`.

## Tuple type
The tuple is the most common way to grouping a number of values with differents type into one type. It have a fixed length, that means once declared you would not be able to add or substract elements from it. 

You can create a tuple closing it elements between parenthesis `()` and separate them with comas (`,`), by example:

```rust
let tup = ("Andrés", 22, "😎");
// without type annotation

let tup: (String, i32, &str) = (String::from("Andrés"), 22, "😎");
```

## Array type
The array type is very similar to the tuple, but some _restrictions_ (can also call them differences). The first is that we can´t hold different type elements within it, only elements of the same type. This is useful is we want to find only one type of elements in our collection. 

To declare an array we have to enclose the elements between curly brackets `[]` and separate them with comas. We also can declare them with type and length annotation, and unlike JavaScript or PHP, arrays is Rust have a fixed length (I DON'T KNOW WHY THEY DECIDE DO IT LIKE THIS). For example:

```rust
let arr_students = ["Andrés", "Diego", "Álvaro", "Francisco"];
// without type and length annotation

let arr_students: [&str; 4] = ["Andrés", "Diego", "Álvaro", "Francisco"];
```

#rust