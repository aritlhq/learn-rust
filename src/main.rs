fn main() {
    println!("Hello, world!");
    print!("Hello Ari, You're so cool!"); // Not new line
    println!("Hello, world!");
}

#[test] // cargo test <function_name> -- --exact --nocapture  
fn hello_test() {
    println!("Hello, world everyone!");
}

/**
 * Variables
 *
 * Variables are used to store data.
 * Variables are immutable by default.
 */
#[test]
fn test_variable() {
    let name = "Ari";
    println!("Hello, {}!", name);
}


/**
 * Mutable Variables
 * 
 * Variables can be mutable, meaning they can be changed.
 * To make a variable mutable, you need to use the `mut` keyword.
 */
#[test]
fn test_mutable() {
    let mut name = "Ari Cool";
    println!("Hello, {}!", name);

    name = "Ari COooOOoooOoOoOolllll";
    println!("Hello, {}!", name);

    // name = 10; // Error
    // println!("Hello, {}!", name);
}

/**
 * Shadowing
 *
 * Variables can be shadowed, meaning they can be redefined.
 * To shadow a variable, you need to use the `let` keyword.
 */
#[test]
fn shadowing() {
    let name = "Ari Cool!";
    println!("Hello, {}!", name);

    let name = "Ari Cool Shadowing!";
    println!("Hello, {}!", name);
}