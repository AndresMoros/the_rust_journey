fn main() {
    print!("{}", reverse_string("Hola como estas"));
    // The functions are expression
}

fn reverse_string(str: &str) -> String {
    // to indicate that the function actually returns a value, we have to specify
    // that value's type with an arrow (->), we also have to indicate the parameter's type
    let reverse_str: String = str.chars().rev().collect();

    reverse_str
    // the last line in a function doesn't use an return keyword or semicolon (;)
    // because at the end of the function the returning is implicit
}
