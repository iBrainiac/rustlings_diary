// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    let char = get_char(data.clone());

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) {
    let uppercase_data = &data.to_uppercase();

    println!("{}", uppercase_data);
}


/**The issue in the  Rust code was that the string_uppercase function 
 * is attempting to take ownership of the data parameter, which is a reference, 
 * instead of taking ownership of the actual String value.
 * 
 * Here's how it was finally solved 
 * 
In the main function, the get_char function now takes a reference 
to the data String instead of taking ownership.
The string_uppercase function now takes ownership of 
the data String parameter.
Inside the string_uppercase function, the data parameter is now
 mutable, allowing the to_uppercase method to be called on it.
The println! statement now uses the uppercase_data variable 
instead of the data parameter directly, as the data parameter
 has been moved into the function.
 */