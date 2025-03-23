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
    let age: i32 = 20;
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

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;
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
    let char2: char = 'b';
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
    let data: (i32, bool, &str, f64) = (1, true, "Hello Everyone!", 93.12);
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
fn unit() {
    println!("This is a unit");
}

#[test]
fn test_unit() {
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
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    // let a  = array[0];
    // let b  = array[1];
    // let c  = array[2];
    // let d  = array[3];
    // let e  = array[4];

    let [a, b, c, d, e] = array;
    println!("{} {} {} {} {}", a, b, c, d, e);

    let mut array_mutable: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array_mutable);

    array_mutable[0] = 10;
    println!("{:?}", array_mutable);

    let array_1_length = array.len();
    let array_2_length = array_mutable.len();
    println!(
        "The length of array is {} and {}",
        array_1_length, array_2_length
    );
}

/**
 * Two Dimensional Array
 *
 * Two Dimensional Array is a collection of values.
 * Two Dimensional Array is immutable by default.
 */
#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
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
const MINIMUM: i32 = 0;

#[test]
fn constant() {
    println!("{}", MAXIMUMMM);

    const MAXIMUM: i32 = 100;
    println!("{} {} {}", MAXIMUM, MAXIMUM, MINIMUM);
}

/**
 * Variable Scope
 *
 * Variable Scope is the area of code where a variable is valid.
 * Variable Scope is determined by the `{}`.
 */
#[test]
fn variable_scope() {
    let ari = 1;

    {
        // Inner scope
        println!("{}", ari);
        let cool = 2;
        println!("{}", cool);
    }

    // println!("{}", kurniawan); // Error
}

fn function_a() {
    let a = 10; // It's a stack variable. It's size is known at compile time. It is stored in the stack.
    let b = String::from("Hello"); // String:: is a data type that size is not known at compile time. It is stored in the heap.

    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("World");
    println!("{} {}", a, b);
}

/**
 * Stack and Heap
 *
 * Stack is a data structure that stores values.
 * Stack is used to store values that size is known at compile time.
 *
 * Heap is a data structure that stores values.
 * Heap is used to store values that size is not known at compile time.
 */
#[test]
fn stack_heap() {
    function_a();
    function_b();
}

/**
 * String slice (&str)
 *
 * String slice is a reference to a part of a string.
 * String slice is used to get a part of a string.
 */
#[test]
fn string() {
    let name: &str = "       Ari Cool ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);
}

/**
 * String Type
 *
 * String Type is a data type that stores a string.
 * String Type is used to store a string.
 */
#[test]
fn string_type() {
    let mut name = String::from("Awesome Ari");
    name.push_str(" Cool"); // It's a method. It's used to add a string to a string.
    println!("{}", name);

    let budi = name.replace("Ari", "Cool"); // It's a method. It's used to replace a string with another string.
    println!("{}", budi);
}

/**
 * Ownership Rules
 *
 * Ownership Rules is a rule that determines how a value is stored in memory.
 * Ownership Rules is used to determine how a value is stored in memory.
 */
#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 20;
        println!("{}", b);
    }

    println!("{}", a);
}

/**
 * Data Copy
 *
 * Data Copy is a rule that determines how a value is stored in memory.
 * Data Copy is used to determine how a value is stored in memory.
 */
#[test]
fn data_copy() {
    // if fixed size, it's copied.
    // if not fixed size, it's moved.
    let a = 10;
    let b = a; // Copy data from a to b
    println!("{} {}", a, b);
}

/**
 * Ownership Movement
 *
 * Ownership Movement is a rule that determines how a value is stored in memory.
 * Ownership Movement is used to determine how a value is stored in memory.
 */
#[test]
fn ownership_movement() {
    let name1 = String::from("Hello World! Welcome to Rust!");

    let name2 = name1; // Move data from name1 to name2
    println!("{}", name2);
    // println!("{}", name1); // Error
}

/**
 * Clone
 *
 * Clone is a method that is used to clone a value.
 * Clone is used to clone a value.
 */
#[test]
fn clone() {
    let name1 = String::from("Hello World! Welcome to Rust!");
    let name2 = name1.clone(); // Clone data from name1 to name2
    println!("{}", name2);
    println!("{}", name1);
}

/**
 * If Expression
 *
 * If Expression is a condition that is used to determine the flow of the program.
 * If Expression is used to determine the flow of the program.
 */
#[test]
fn if_expression() {
    let number = 10;
    if number > 10 {
        println!("Number is greater than 10");
    } else if number < 10 {
        println!("Number is less than 10");
    } else {
        println!("Number is equal to 10");
    }

    // Let statement
    let variable1 = 19;
    let result;

    if variable1 > 10 {
        result = "OK ++ 10";
    } else {
        result = "Nope -- 10";
    }
    println!("{}", result);
}

/**
 * Loop Expression
 *
 * Loop Expression is a condition that is used to determine the flow of the program.
 * Loop Expression is used to determine the flow of the program.
 */
#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter == 5  {
            continue;
        }
    }

    println!("Counter: {}", counter);
}

/**
 * Loop Return Value
 *
 * Loop Return Value is a condition that is used to determine the flow of the program.
 * Loop Return Value is used to determine the flow of the program.
 */
#[test]
fn loop_return_value(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };

    println!("Counter: {}", result);
}

/**
 * Loop Label
 *
 * Loop Label is a condition that is used to determine the flow of the program.
 * Loop Label is used to determine the flow of the program.
 */
#[test]
fn loop_label() {
    let mut number = 10;
    'outer: loop { // 'outer' is a label
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i );
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

/**
 * While Loop
 *
 * While Loop is a condition that is used to determine the flow of the program.
 * While Loop is used to determine the flow of the program.
 */
#[test]
fn while_loop() {
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        println!("Counter: {}", counter);
    }

    // With break and continue
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        if counter == 5 {
            continue;
        } else if counter == 8 {
            break;
        }
        println!("Counter with break and continue: {}", counter);
    }
}

/**
 * For Loop
 *
 * For Loop is a condition that is used to determine the flow of the program.
 * For Loop is used to determine the flow of the program.
 */
#[test]
fn array_iteration_for_loop() {
    let array = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < array.len() {
        println!("The first way:{}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop_2() {
    let array = [1, 2, 3, 4, 5];
    for element in array {
        println!("The second way: {}", element);
    }
}

/**
 * Exclusive Range
 *
 * Exclusive Range is a condition that is used to determine the flow of the program.
 * Exclusive Range is used to determine the flow of the program.
 */
#[test]
fn exclusive_range(){
    let array: [&str; 5] = ["Ari", "Cool", "Hello World!", "Coding", "Rust"];

    let range = 0..5; // Exclusive range
    println!("START: {}", range.start);
    println!("END: {}", range.end);

    for i in range {
        println!("With variable named 'range': {}", array[i]);
        println!("With variable named 'range': {}", i);
    }

    for i in 0..5 {
        println!("With 0..5: {}", array[i]);
        println!("With 0..5: {}", i);
    }
}

/**
 * Inclusive Range
 *
 * Inclusive Range is a condition that is used to determine the flow of the program.
 * Inclusive Range is used to determine the flow of the program.
 */
#[test]
fn inclusive_range(){
    let array = ["Ari", "Cool", "Hello World!", "Coding", "Rust"];

    let range = 0..=4; // Inclusive range
    // Can't directly access start of RangeInclusive, use alternative approaches if needed
    println!("START: {}", range.start()); // Using start() method to access the start value
    println!("END: {}", range.end()); // * named dereference operator to get the value

    for i in range {
        println!("With variable named 'range': {}", array[i]);
        println!("With variable named 'range': {}", i);
    }
    for i in 0..=4 {
        println!("With 0..=4: {}", array[i]);
        println!("With 0..=4: {}", i);
    }
}