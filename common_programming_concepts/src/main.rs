use std::io;

fn mutability() {
    // Hypothesis: x is not declared as mutable, so one of two things will happen
    //  a) Both print statements print a value of 5
    //  b) Compile time error for trying to mutate an immutable variable <- Result
    //      (of course it threw an error, that's the whole point of Rust it would seem, we will make a better idiot yet)
    //let x = 5;
    //println!("The value of x is: {x}");
    //x = 6;
    //println!("The value of x is: {x}");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); //this should print 12? CORRECT
    }

    println!("The value of x is: {x}"); //this should print 6? CORRECT

    let spaces = "   ";
    println!("Spaces then: {spaces}");
    let spaces = spaces.len();
    println!("Spaces now: {spaces}");
}

fn floating_math() {
    let x = 1.5333333333333333333333333333333;
    let y: f32 = 1.5333333333333333333333333333333;

    println!("X: {x}\nY: {y}");

    let z = x + y; //x resolves to f32 at compile time to allow for this

    println!("Z: {z}");
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("Tuple contents: {x}, {y}, {z}");

    let element_one = tup.1;

    println!("Tuple @ element 1: {element_one}")
}

fn arrays() {
    let array: [f32; 10] = [3.14; 10];

    let arr_element = array[4];
    println!("Array @ 4: {arr_element}");
}

fn array_indexing_oor_example() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn main() {
    mutability();
    shadowing();
    floating_math();
    tuples();
    arrays();
    array_indexing_oor_example();
}
