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

fn main() {
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

    let adivb = divide(a, b);

    println!("{:?}", adivb);
}
