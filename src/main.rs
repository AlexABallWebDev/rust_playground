// Author: Alex Ball
// I am creating this project as a set of notes on rust.

// These notes were created while following along with a rust tutorial
// by Tensor Programming on Youtube:
// https://www.youtube.com/watch?v=y7iSQ3s_yms&index=3&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW

// import a library/external dependency
use std::mem;

fn main() {
    // Note that functions that do not return anything will implicitly return an empty tuple.
    // _mutability();
    // _tuples();
    // _arrays();
    // _strings();
    _ownership();
}

// underscore suppresses the "unused" warning.
fn _mutability() {
    // variables are immutable (constant) by default. mut makes them mutatable.
    let mut x: u32 = 5;
    println!("{}", x);
    // this only works with mut variables
    x = 10;
    println!("{}", x);
}

fn _tuples() {
    // tuple example. Can intermix types.
    let tupl = (1, "fifty");

    // print value
    println!("{}", tupl.1);
    // print debug (strings are surrounded by quotes ("))
    println!("{:?}", tupl.1);
    // debug the entire tuple
    println!("{:?}", tupl);
    // debug with pretty for the entire tuple
    println!("{:#?}", tupl);

    // tuples that are too long cannot be printed
    // let tooLong = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    // println!("{:?}", tooLong);
}

fn _arrays() {
    // array example
    let arra: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", arra);
    // print array item
    println!("{:?}", arra[0]);
    // print array length
    println!("{:?}", arra.len());
    // print array memory size
    println!("{:?}", mem::size_of_val(&arra));

    // taking slices. takes items 2,3, and 4. The min index is inclusive, the max index is exclusive.
    let slic = &arra[1..4];
    println!("{:?}", slic);
}

fn _strings() {
    // string literal ("mystring") is a slice of a string (&str).
    // strings are compound types of slices put together into a string.

    // strings can be treated like slices because of this.
    // to create a String with the type String, use this.
    let the_string = String::from("Hello ");
    println!("{:?}", the_string);

    // string concatenation example. Note the reference (&another_string).
    let another_string = String::from("World!");
    let combined_string = the_string + &another_string;
    println!("{}", combined_string);
}

fn _ownership() {
    // scope ends at the closing curly brace, so we cannot print _a after its scope ends.
    {
        let _a = 10;
    }
    // println!("{}", _a);

    // variables own values. x owns 1, stored on the stack.
    // let x = 1;

    // "borrowing" allows another variable to use its owned value.
    let strin = String::from("myString");
    let other_strin_var = &strin;
    println!("{}", other_strin_var);

    // "moving" a value as a parameter to a function causes us to lose access
    // to it since we gave it to the function.
    // Vec is like an expandable array (think python list). It is stored in
    // the heap, so it is a referenced value, rather than primitives which are
    // stored in the stack.
    println!("Moving example start.");
    let mut v = Vec::new();
    for i in 1..100 {
        v.push(i);
    }
    _move(v);
    // If we do not get ownership of the referenced v back, then we can no
    // longer use it since we gave it to _move().
    // println!("{}", v[0]);
    println!("Moving example end.");

    // "copying" example. Similar to "move", but a parameter is passed by copy
    // of the value. This is what happens with primitives (this example uses i32)
    // with the same syntax as the moving example.
    println!("Copying example start.");
    let a = 20;
    let b = 30;
    println!("before calling _copy() main has a: {} and b: {}", a, b);
    _copy(a, b);
    println!("After calling _copy() main still has a: {} and b: {}", a, b);
    println!("Copying example end.");

    // Bigger "borrowing" example.
    println!("Borrowing example start.");
    let mut v2 = Vec::new();
    for i in 1..100 {
        v2.push(i);
    }
    println!("created v2. for testing, the item with index 50 is: {}", v2[50]);

    // v2 gets returned after it is borrowed by the function,
    // so v2 has ownership of its value again.
    v2 = _return_after_borrowing(v2);
    println!("after being returned from a function that took v2 as Vec as a param: {}", v2[50]);

    // pass a reference to the function. The function will dereference v2 to access it.
    // When the function is finished, then v2 will own the vector again.
    _borrow_dereference(&v2);
    println!("after being passed as a reference to a function that took v2 and dereferenced it, v2 can be accessed in main: {}", v2[50]);

    // pass a reference to the function. It sounds like there is
    // some automatic dereferencing happening here. 
    // When the function is finished, then v2 will own the vector again.
    _borrow_dereference(&v2);
    println!("after being passed as a reference to a function, v2 can be accessed in main: {}", v2[50]);
    println!("Borrowing example end.");

    // example of borrowing with loop and function.
    // loop borrows, then function borrows.
    println!("Loop and function borrowing example start.");

    // vec! is a macro for creating a vector.
    let v3 = vec![4, 5, 3, 6, 7, 4, 8, 6, 4, 2, 4, 2, 5, 3, 7, 7];
    println!("created v3: {}", v3[0]);
    for &i in &v3 {
        let i_count = count(&v3, i);
        println!("{} is repeated {} times", i, i_count);
    }
    // v3 is owned after the loop completes because it was only passed by
    // reference to the loop and function.
    println!("v3 is owned after the loop: {}", v3[0]);
    println!("Loop and function borrowing example end.");
}

fn _move(v: Vec<i32>) {
    println!("_move() took v: {}", v[10] + v[15]);
}

fn _copy(a: i32, b: i32) {
    println!("_copy() combined a and b to make: {}", a + b);
}

fn _return_after_borrowing(v: Vec<i32>) -> Vec<i32> {
    println!("_return_after_borrowing() borrowed and will return v2: {}", v[50] + v[51]);
    v
}

fn _borrow_dereference(v: &Vec<i32>) {
    println!("_borrow_dereference borrowed and dereferenced v2: {}", (*v)[50] + (*v)[51]);
}

fn _borrow_without_dereference(v: &Vec<i32>) {
    println!("_borrow_without_dereference borrowed v2: {}", v[50] + v[51]);
}

fn count(v: &Vec<i32>, value: i32) -> usize {
    v.into_iter().filter(|&&x| x == value).count()
}