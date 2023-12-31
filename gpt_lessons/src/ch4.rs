/*
Problem 1: You have a function process_data that takes ownership of a 
String and returns the length of the string. 
Write the function and demonstrate what happens when you try to use 
the string after calling this function.
 */

 pub fn p1() {
    let my_data = String::from("Hello, Rust!");
    let data_length = process_data(my_data);

    // What happens if we try to use `my_data` here?
    //println!("{}", my_data); -- 
    /* since my_data was passed through process_data, it no longer has ownership and throws 
    an error when trying to access it */
}

fn process_data(data: String) -> usize {
    data.len() // returns the length of `data`
}

// --------------------------------------------------------
/*
Problem 2: Cloning and Ownership
Create two String variables, where the second string is a clone of the first. 
Modify the first string and then print both strings. This tests your understanding of how 
cloning works in Rust and the concept of ownership.
 */

pub fn p2() {
    let mut original = String::from("The original string!");
    let clone = original.clone();

    // modify original
    original.push_str("string!!!"); // adds string!!! to orignal but not to clone

    println!("Original: {original}");
    println!("Clone: {clone}");
}

// -----------------------------------------------------------
/*
Problem 3: Immutable References
Create a function calculate_length that takes an immutable reference to a 
String and returns its length. In your main function, create a String and 
call calculate_length with a reference to it. Then, print the string and its length. 
This problem tests your understanding of immutable references and how they allow 
read-only access to data.
 */
pub fn p3() {
    let test_string = String::from("Test string here");
    let string_length = calculate_length(&test_string);

    println!("String: {test_string}");
    println!("Length: {string_length}");

}

fn calculate_length(str: &String) -> usize {
    str.len()
}

// ------------------------------------------------------------
/*
Problem 4: Mutable References
Write a function append_world that takes a mutable reference to a String and appends the word
"world" to it. In your main function, create a String, call append_world, and then 
print the modified string. This problem examines your understanding of 
mutable references and how they allow modification of data they refer to
*/

pub fn p4() {
    let mut str_ = String::from("Greetings, ");
    println!("Original str: {str_}");
    append_world(&mut str_);
    println!("New str: {str_}");

}

fn append_world(str_: &mut String) {
    str_.push_str("world");
}

// ---------------------------------------------------------------
/*
Problem 5: String Slices
Write a function first_word that takes a string slice and returns the first word it finds. 
If the function doesn't find a space in the string, the whole string is one word, 
so the entire string should be returned.
*/

pub fn p5() {
    let my_string = String::from("Hello There.");
    let word = first_word(&my_string);
    println!("First word: {word}");
}

fn first_word(str_: &String) -> String {
    let n = str_.len();
    let mut i = 0;

    for c in str_.chars() {
        if c == ' ' { // denote character with single quotes
            break
        }
        i += 1;
    }

    str_[..i].to_string()
}

// ---------------------------------------------------------
/*
Problem 6: Array Slices
Create a function sum_elements that takes a slice of integers and returns their sum. 
In your main function, create an array of integers, take a slice of it, and 
pass it to sum_elements. Print the result.
*/

pub fn p6() {
    let numbers = [1,2,3,4,5,6];
    let slice = &numbers[1..4];
    let sum_result = sum_elements(&slice);
    println!("Slice: {:?}", slice);
    println!("Sum result: {sum_result}");

}

fn sum_elements(arr: &[i32]) -> i32 {
    let mut res: i32 = 0;
    for elem in arr.iter() {
        res += elem;
    }
    res
}