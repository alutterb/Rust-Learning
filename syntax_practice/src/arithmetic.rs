// function definition
fn add_two_numbers(x:i32, y:i32) -> i32 {
    x + y
}

// error handling
fn divide(x:i32, y:i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("Division by zero error")
    } else {
        Ok(x / y)
    }
}

// Option<dtype> is used when a function may not return a value. This is used in place of None
fn div_result(x:i32, y:i32) -> Option<i32> {
    let xdivy = divide(x,y);
    // match syntax, similar to switch in other languages
    // error handling is the standard way to use match to handle the different return types
    match xdivy {
        Ok(result) => {
            return Some(result);
        },
        Err(_e) => {
            None
        }
    }
}
// use keyword pub so that it can be called in other files
pub fn practice_arithmetic() {
    // variable assignment
    let a: i32 = 100; // specifies a 32 bit integer
    //a = 101; // this will throw an error since we did not specify a to be mutable
    let mut b: i32 = 100;
    

    // if statement
    if b <= a {
        b = 101; // does not throw error since b is mutable
        println!("Updated b to 101")
    } else {
        println!("b is already greater than a!")
    }

    // while loop
    while b > a - 5 { b -= 1; }

    println!("{}", b); // string formatting - here we print the value of the variable
    println!("---------");

    // for loop
    for i in b..a {
        println!("{}", i);
    }

    println!("---------");

    // function calling
    let result = add_two_numbers(a,b);

    println!("{}", result);

    println!("------");

    let adivb = div_result(a, b);

    // since advib could be Some or None, we can use match to handle this as well
    match adivb {
        Some(value) => {
            println!("Result of {} divided by {} = {}", a, b, value);
        }
        None => {
            println!("Values a = {} and b = {} result in division error.", a, b);
        }
    }
}