use std::io;

fn main() {
    println!("========= strings");
    let name = String::from("Andrés");
    // simple and common string variable

    println!("========= numbers");
    let age_negative: i32 = -23;
    let age_positive: u32 = 23;

    println!("Negative number: {age_negative}");
    println!("Positive number: {age_positive}");

    // const PI: f64 = 3.14;
    // a constants with float value

    println!("================== compound types");
    // they can hold several elements (as a 'collection' of elements)

    println!("========= tuples");
    let tup: (u32, i32, String, bool) = (32, -32, String::from("Hello World"), true);
    // a tuple can holds several type of elements
    // this first example shows a tuple with type annotation,
    // but it can be declared without one, by example:
    //
    // let tup = (32, -32, String::from("Hello World"), true);

    println!("========= arrays");
    let arr = ["Andrés", "Moros"];
    // an array with two arrays, the array only can hold a list of values with the same type
    // let arr: [&str; 5] = ["Andrés", "Gabriel", "Álvaro", "Diego", "Daniel"];
    // the annotation indicates that the array have 5 elements with the type "&str"

    println!("Access to array and tuples values");
    println!("{name} {} says '{}'", arr[1], tup.2);
    // we use dot follows for the index of the element we want in a tuple
    // and square brackets follows for the index in arrays

    println!("========= examples 'bout data manipulation on  coompound types");
    let mut tup: (String, i32, &str) = (String::from("Andrés"), 22, "😎");

    println!("Original tup: {:?}", tup); // ("Andrés", 22, "😎")
    println!("Modified tup: {:?}", change_tuple_values(&mut tup)); // First Ricardo Milos, Second 32, Third  💀

    let prices_tup: (f32, f32, f32) = (800.0, 978.0, 765.0);
    let discount_prices = apply_discount(prices_tup);

    println!("Main scope tuple: {:?}", prices_tup); // (800.0, 978.0, 765.0)
    println!("New tuple by function: {:?}", discount_prices); // (720.0, 880.2, 688.5)

    println!("========= booleans");
    // for use the is adult function
    println!("Enter your age");
    let mut age = String::new(); // declare an empty string

    io::stdin().read_line(&mut age).expect("error");
    // using the stdin() function with the read_line() method that reads the input from user
    // read_line() take as parameter an String type variable, that's our 'age'
    // read_line() have an expect() method that return Ok that holds the value o the input or
    // an Err type that holds the error details

    let age: u16 = age.trim().parse().expect("Error at conversion");
    // alike other languages we can use ".trim()" delete whitespaces or enters from our string inputs

    println!(
        "The user is an adult? : {}",
        match is_adult(age) {
            true => "Yes",
            false => "No",
        }
    );
}

// this function has the intention
// of moodify the values of the original tuple/
// it does it because it recieve a mutable tuple reference,
// in order to achieve this, the tuple has to be
// mutable on the main function
fn change_tuple_values(tup: &mut (String, i32, &str)) {
    tup.0 = String::from("Ricardo Milos");
    tup.1 = tup.1 + 10;
    tup.2 = "💀";

    println!("First {}, Second {}, Third  {}", tup.0, tup.1, tup.2)
}

// this function has the intention
// of keeps the original tuple intact
// and create a new one
fn apply_discount(tuple_prices: (f32, f32, f32)) -> (f32, f32, f32) {
    return (
        tuple_prices.0 * 0.9,
        tuple_prices.1 * 0.9,
        tuple_prices.2 * 0.9,
    );
}

// this funcion return true if the age of a person
// is equal ro  greater then 18 years old
fn is_adult(age: u16) -> bool {
    if age >= 18 { true } else { false }
}
