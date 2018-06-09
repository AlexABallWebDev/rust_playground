// Author: Alex Ball
// I am creating this project as a set of notes on rust.

// These notes were created while following along with a rust tutorial
// by Tensor Programming on Youtube:
// https://www.youtube.com/watch?v=y7iSQ3s_yms&index=3&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW

fn main() {
    // Note that functions that do not return anything will implicitly return an empty tuple.
    // _mutability();
    // _tuples();
    // _arrays();
    // _strings();
    // _ownership();
    // _structures();
    _control_flow();
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

    // import a library/external dependency
    use std::mem;

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

/// This function covers notes about ownership and borrowing, with some
/// introduction to memory references and dereferencing.
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

    fn move_v(v: Vec<i32>) {
        println!("_move() took v: {}", v[10] + v[15]);
    }
    move_v(v);
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

    fn copy(a: i32, b: i32) {
        println!("_copy() combined a and b to make: {}", a + b);
    }
    copy(a, b);
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
    fn return_after_borrowing(v: Vec<i32>) -> Vec<i32> {
        println!("_return_after_borrowing() borrowed and will return v2: {}", v[50] + v[51]);
        v
    }
    v2 = return_after_borrowing(v2);
    println!("after being returned from a function that took v2 as Vec as a param: {}", v2[50]);

    // pass a reference to the function. The function will dereference v2 to access it.
    // When the function is finished, then v2 will own the vector again.
    fn borrow_dereference(v: &Vec<i32>) {
        println!("_borrow_dereference borrowed and dereferenced v2: {}", (*v)[50] + (*v)[51]);
    }
    borrow_dereference(&v2);
    println!("after being passed as a reference to a function that took v2 and dereferenced it, v2 can be accessed in main: {}", v2[50]);

    // pass a reference to the function. It sounds like there is
    // some automatic dereferencing happening here. 
    // When the function is finished, then v2 will own the vector again.
    fn borrow_without_dereference(v: &Vec<i32>) {
        println!("_borrow_without_dereference borrowed v2: {}", v[50] + v[51]);
    }
    borrow_dereference(&v2);
    println!("after being passed as a reference to a function, v2 can be accessed in main: {}", v2[50]);
    println!("Borrowing example end.");

    // example of borrowing with loop and function.
    // loop borrows, then function borrows.
    println!("Loop and function borrowing example start.");

    // vec! is a macro for creating a vector.
    let v3 = vec![4, 5, 3, 6, 7, 4, 8, 6, 4, 2, 4, 2, 5, 3, 7, 7];
    println!("created v3: {}", v3[0]);

    fn count(v: &Vec<i32>, value: i32) -> usize {
        v.into_iter().filter(|&&x| x == value).count()
    }
    for &i in &v3 {
        let i_count = count(&v3, i);
        println!("{} is repeated {} times", i, i_count);
    }
    // v3 is owned after the loop completes because it was only passed by
    // reference to the loop and function.
    println!("v3 is owned after the loop: {}", v3[0]);
    println!("Loop and function borrowing example end.");
}

/// This function covers notes about structures, methods, related functions
/// (like Java static methods), and display/debug traits.
fn _structures() {
    // struct contains data (properties).
    // The derive annotation is for deriving the Debug trait, used later for
    // printing a Rectangle with debug info in println!.
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // access properties with a period (.)
    fn area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }

    // instantiate a Rectangle structure.
    let rect = Rectangle {
        width: 35,
        height: 55,
    };

    println!("Rectangle with {}x{} has area: {}", rect.width, rect.height, area(&rect));

    // Create an implementation of Rectangle that has a method.
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn show(&self) {
            println!("Rectangle show: {}x{} has area: {}", self.width, self.height, self.area());
        }
    }
    println!("Rectangle area method: {}x{} has area: {}", rect.width, rect.height, rect.area());

    // Related functions are like static methods in Java. Related functions
    // can be defined in the same impl block, or in a separate one like this.
    // Separating related functions and methods like this is encouraged.
    impl Rectangle {
        // related function that creates a Rectangle.
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle {
                // Note that if the parameter names are the same as
                // the properties, you don't need to give the value.
                width,//: width,
                height,//: height,
            }
        }
    }

    // Use the related function to create a new rectangle.
    let new_rect = Rectangle::new(57, 83);
    new_rect.show();

    // Because of the derived Debug trait on Rectangle, we can print it with
    // debug info.
    println!("Debug new_rect: {:#?}", new_rect);

    // import for the Display trait.
    use std::fmt;

    // This impl is for adding the Display trait to the Rectangle.
    impl fmt::Display for Rectangle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write! macro is used to write strings to arbitrary writers.
            // Just like Java, we can use it to write to files or buffers
            // or fun things like that.
            write!(f, "({}, {}) and area: {}", self.width, self.height, self.area())
        }
    }

    // With the Display trait, we can print it without debug.
    println!("Print (Display) new_rect: {}", new_rect);
}

fn _control_flow() {
    // logical operators: == != < > >= <=
    let num = 6;

    // if conditions need to evaluate to a boolean.
    // No wierd type coercion for if conditions like in javascript.

    // if block
    if num < 10 {
        println!("{} is less than 10", num);
    }

    // if with else if and else
    if num % 4 == 0 {
        println!("{} is divisible by 4", num);
    } else if num % 3 == 0 {
        println!("{} is divisible by 3", num);
    } else {
        println!("example of an else block.");
    }

    // binding (initialize/set a variable) with if and else.
    // This kind of seems like ternary operators in other languages.
    let condition = true;
    let my_num = if condition {
        50
    } else {
        76
    };
    println!("myNum: {}", my_num);

    // Infinite loop example
    // loop {
    //     println!("infinite loop");
    // }

    // loop example
    let mut count = 0;
    loop {
        println!("in loop, count: {}", count);
        count += 1;

        if count >= 10 {
            // it looks like you have to explicitly break loops.
            // there is no "while condition".
            // break, without any labels given, breaks the innermost loop
            // (assuming nested loops).
            break;
        }
    }

    // nested loop with labels example. Nested loops can break by label.
    let mut end_condition = false;
    'a: loop {
        println!("loop a");
        'b: loop {
            println!("loop b");
            'c: loop {
                println!("loop c");

                if end_condition {
                    // end our loops by breaking the outermost one.
                    break 'a;
                }
                end_condition = true;

                // break loop b, meaning we start the next line in loop a.
                break 'b;
            }
            // println!("This print in loop b will not run since c will break b or a.");
        }

        // continue can also be given a label to start the first line in loop a.
        continue 'a;

        // println!("This print will not run.");
    }

    // binding loops
    let x = loop {
        // break acts like "return" in this context.
        break 10;
    };
    println!("x = {}", x);

    // while loop example
    let mut get_to_zero = 10;
    while get_to_zero != 0 {
        println!("while loop: {}!", get_to_zero);
        get_to_zero = get_to_zero - 1;
    }

    // for loop example. for loops look similar to python.
    let my_vec = vec![1, 2, 3, 4, 5];
    for i in my_vec {
        println!("for loop vec example i: {}", i);
    }

    // for loop without a list/array/vector.
    // Note that the range does not include the endpoint
    // (prints to 9 rather than 10).
    for i in 1..10 {
        println!("for loop with range example i: {}", i);
    }

    // for loop with inclusive range.
    // Was experimental when the tutorial was published, but it seems to work now.
    for i in 1..=10 {
        println!("for loop with range INCLUSIVE example i: {}", i);
    }

    // match example. Similar to switch statements in other languages.
    let mat = 5;
    match mat {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else. This is like a default."),
    }

    // match example with multiple conditions leading to the same case.
    // includes single bar (|) or-like options and a range using ...
    let mat2 = 19;
    println!("Extra matching example, mat2: {}", mat2);
    match mat2 {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // note this range is inclusive, including 19
        13 ... 19 => println!("This is a \"teen\"", ),
        _ => println!("This is not special", ),
    }

    // match with conditions using tuples. We can match on one index,
    // then retrieve the other index for use in a case
    // (in this example, print it out).
    let my_tuple = (0, -2);
    println!("matching example with tuples: {:?}", my_tuple);
    match my_tuple {
        (0, y) => println!("first index is 0, y: {}", y),
        (x, 0) => println!("second index is 0, x: {}", x),
        _ => println!("tuple has no match"),
    }

    // match with extra conditions.
    let pair = (5, -5);
    println!("matching with extra conditions: {:?}", pair);
    match pair {
        (x, y) if x == y => println!("x equals y"),
        (x, y) if x + y == 0 => println!("x plus y equals zero"),
        (x, _) if x % 2 == 0 => println!("x is even"),
        _ => println!("no match"),
    }

    // match with binding a variable to the matched value
    // good for accessing a value that we do not have ownership of,
    // since the bound variable (n) is basically a clone of the given value (p).
    let p = 5;
    println!("matching with binding a variable to the match: {}", p);
    match p {
        n @ 1 ... 12 => println!("n is between 1-12: {}", n),
        n @ 13 ... 19 => println!("n is between 13-19: {}", n),
        _ => println!("no match"),
    }

    // match can also be used to bind a variable.
    let p2 = 14;
    println!("binding a variable with a match: {}", p2);
    let n2 = match p2 {
        n @ 1 ... 12 => n + 1,
        n @ 13 ... 19 => n + 2,
        _ => 0,
    };
    println!("n2 was assigned: {}", n2);
}