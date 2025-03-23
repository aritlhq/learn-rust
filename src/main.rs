fn main() {
    println!("Hello, world!");
    print!("Hello Ari, You're so cool!"); // Not new line
    println!("Hello, world!");
}

#[test] // cargo test <function_name> -- --exact --nocapture  
fn hello_test() {
    println!("Hello, world everyone!");
}