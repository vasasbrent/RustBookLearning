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

fn main() {
    mutability();
    shadowing();
}
