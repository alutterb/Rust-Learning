mod arithmetic; // this is how we can use functions from our other file
mod adventure;

fn main() { 
    arithmetic::practice_arithmetic(); // call the practice arithmetic function in arithmetic.rs
    adventure::begin_adventure(); // call adventure fn
}
