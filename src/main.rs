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

/**
 * Data Types
 * 
 * Rust has a 2 types of data types:
 * 1. Scalar (single value)
 * 2. Compound (more than one value)
 */
#[test]
fn explicit() {
    let age:i32 = 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.9;
    println!("{}", b);
}

/**
 * Number Conversion
 * 
 * Rust has a number conversion, meaning you can convert a number to another number.
 * To convert a number, you need to use the `as` keyword.
 * 
 * Tips:
 * 1. The number conversion is not always accurate.
 * 2. The number conversion is not always possible.
 * 3. The number conversion is not always safe.
 * 4. The number conversion is not always fast.
 * 5. The number conversion is not always easy.
 */
#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b:i16 = a as i16;
    println!("{}", b);

    let c:i32 = b as i32;
    println!("{}", c);

    let d:i64 = 1000000000;
    let e:i8 = d as i8;
    println!("{}", e);
}

#[test]
fn comparation() {
    let result = 10 > 5;
    println!("{}", result);

    let result = 10 < 5;
    println!("{}", result);
}

#[test]
fn char_type() {
    let char1 = "I'm a string not a char";
    let char2:char = 'b';
    println!("{}\n{}", char1, char2);
}

/** 
 * Tuple
 * 
 * Tuple is a collection of values.
 * Tuple is immutable by default.
 */
#[test]
fn tupple() {
    let data:(i32, bool, &str, f64) = (1, true, "Hello Everyone!", 93.12);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;
    // let d = data.3;

    let (a, b, c, d) = data;
    println!("{} {} {} {}", a, b, c, d);

    let mut data_mutable = (20, false, "Hello Teacher!", 32.4);
    println!("{:?}", data_mutable);

    data_mutable.0 = 10;
    println!("{:?}", data_mutable);
}

/**
 * Unit
 * 
 * Unit is a type that has no value.
 */
fn unit(){
    println!("This is a unit");
}

#[test]
fn test_unit(){
    let result = unit();
    println!("{:?}", result); // () is a unit

    let test = ();
    println!("{:?}", test); // () is a unit
}

/**
 * Array
 *
 * Array is a collection of values.
 * Array is immutable by default.
 */
#[test]
fn array(){
    let array:[i32; 5] = [1,2,3,4,5];
    println!("{:?}", array);

    // let a  = array[0];
    // let b  = array[1];
    // let c  = array[2];
    // let d  = array[3];
    // let e  = array[4];

    let [a, b, c, d, e] = array;
    println!("{} {} {} {} {}", a, b, c, d, e);

    let mut array_mutable:[i32; 5] = [1,2,3,4,5];
    println!("{:?}", array_mutable);

    array_mutable[0] = 10;
    println!("{:?}", array_mutable);

    let array_1_length = array.len();
    let array_2_length = array_mutable.len();
    println!("The length of array is {} and {}", array_1_length, array_2_length);
}

/**
 * Two Dimensional Array
 *
 * Two Dimensional Array is a collection of values.
 * Two Dimensional Array is immutable by default.
 */
#[test]
fn two_dimensional_array(){
    let matrix:[[i32; 3]; 3] = [
        [1,2,3],
        [4,5,6],
        [7,8,9]
    ];
    println!("{:?}", matrix);
    // print 1,5,9
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[2][2]);
}

/** 
 * Constants
 *
 * Constants are immutable by default.
 * Constants are declared with the `const` keyword.
 */
const MAXIMUMMM: i32 = 100;
const MINIMUM:i32 = 0;

#[test]
fn constant(){
    println!("{}", MAXIMUMMM);

    const MAXIMUM:i32 = 100;
    println!("{} {} {}", MAXIMUM, MAXIMUM, MINIMUM);
}

